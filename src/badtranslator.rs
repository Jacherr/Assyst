use crate::assyst::Assyst;
use crate::{rest::bt, util::get_current_millis, util::sanitize_message_content, util::normalize_emojis};
use std::borrow::Cow;
use std::{cmp::min, collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use twilight_model::gateway::payload::MessageCreate;
use twilight_model::{
    channel::Webhook,
    id::{ChannelId, UserId},
    user::User,
};

macro_rules! unwrap_or_eprintln {
    ($what:expr, $msg:expr) => {
        match $what {
            Some(x) => x,
            None => {
                eprintln!($msg);
                return;
            }
        }
    };
}

pub type ChannelCache = HashMap<Snowflake, Option<Webhook>>;
type Snowflake = u64;

mod flags {
    pub const DISABLED: u32 = 0x1;
}

mod constants {
    pub const MAX_MESSAGE_LEN: usize = 500;
    pub const RATELIMIT_LEN: u64 = 3000;
    pub const RATELIMITED_MESSAGE: &'static str = "You are sending messages too quickly!";
}

#[derive(Debug)]
struct BadTranslatorRatelimit(u64);

impl BadTranslatorRatelimit {
    pub fn new() -> Self {
        Self(get_current_millis())
    }

    pub fn expired(&self) -> bool {
        get_current_millis() - self.0 >= constants::RATELIMIT_LEN
    }
}

pub struct BadTranslator {
    flags: RwLock<u32>,
    channels: RwLock<ChannelCache>,
    ratelimits: RwLock<HashMap<u64, BadTranslatorRatelimit>>,
}

impl BadTranslator {
    pub fn new() -> Self {
        Self::with_channels(HashMap::new())
    }

    pub async fn add_channel(&self, id: Snowflake) {
        if !self.is_disabled().await {
            let mut lock = self.channels.write().await;
            lock.insert(id, None);
        }
    }

    pub fn with_channels(channels: ChannelCache) -> Self {
        Self {
            channels: RwLock::new(channels),
            ratelimits: RwLock::new(HashMap::new()),
            flags: RwLock::new(0),
        }
    }

    pub async fn is_channel(&self, k: Snowflake) -> bool {
        self.channels.read().await.contains_key(&k)
    }

    pub async fn set_channels(&self, channels: ChannelCache) {
        *self.channels.write().await = channels;
    }

    pub async fn should_fetch(&self) -> bool {
        !self.is_disabled().await && self.channels.read().await.len() == 0
    }

    pub async fn disable(&self) {
        let mut value = self.flags.write().await;
        *value |= flags::DISABLED;
    }

    pub async fn is_disabled(&self) -> bool {
        (*self.flags.read().await & flags::DISABLED) == flags::DISABLED
    }

    pub async fn get_or_fetch_webhook(
        &self,
        assyst: &Arc<Assyst>,
        id: &ChannelId,
    ) -> Option<Webhook> {
        let cache = self.channels.read().await;

        if let Some(Some(value)) = cache.get(&id.0) {
            return Some(value.clone());
        }

        // Drop early so we don't keep the read mutex locked for hundreds of ms
        drop(cache);

        // TODO: maybe return Result?
        let webhooks = assyst.http.channel_webhooks(*id).await.ok()?;

        let webhook = webhooks.get(0)?;

        let mut cache = self.channels.write().await;
        cache.insert(id.0, Some(webhook.clone()));

        Some(webhook.clone())
    }

    /// Returns true if given user ID is ratelimited
    pub async fn try_ratelimit(&self, id: &UserId) -> bool {
        let cache = self.ratelimits.read().await;

        if let Some(entry) = cache.get(&id.0) {
            let expired = entry.expired();
            if !expired {
                return true
            }
        }

        drop(cache);

        let mut cache = self.ratelimits.write().await;

        cache.insert(id.0, BadTranslatorRatelimit::new());

        false
    }

    pub async fn handle_message(&self, assyst: &Arc<Assyst>, message: Box<MessageCreate>) {
        // We're assuming the caller has already made sure this is a valid channel
        // So we don't check if it's a BT channel again

        let message_len = message.content.len();

        if is_webhook(&message.author) || is_ratelimit_message(assyst, &message) {
            return;
        }

        if message_len == 0 || message_len >= constants::MAX_MESSAGE_LEN || message.author.bot {
            let _ = assyst
                .http
                .delete_message(message.channel_id, message.id)
                .await;
            return;
        }

        let ratelimit = self.try_ratelimit(&message.author.id).await;
        if ratelimit {
            assyst
                .http
                .create_message(message.channel_id)
                .content(&format!(
                    "<@{}>, {}",
                    message.author.id.0,
                    constants::RATELIMITED_MESSAGE
                ))
                .unwrap()
                .await
                .unwrap();
            return;
        }

        let content = normalize_emojis(&message.content);

        let translation = match bt::translate(&assyst.reqwest_client, &content).await {
            Ok(res) => Cow::Owned(res.result.text),
            Err(bt::TranslateError::Raw(msg)) => Cow::Borrowed(msg),
            _ => return,
        };

        // If we don't have permissions to delete messages, we just silently ignore it
        let _ = assyst
            .http
            .delete_message(message.channel_id, message.id)
            .await;

        let webhook = unwrap_or_eprintln!(
            self.get_or_fetch_webhook(assyst, &message.channel_id).await,
            "Could not find webhook for channel"
        );

        let token = unwrap_or_eprintln!(webhook.token.as_ref(), "Failed to extract token");

        let translation = sanitize_message_content(&translation[0..min(translation.len(), 1999)]);

        // Again, this might be a permission problem, so we ignore it if it fails
        let _ = assyst
            .http
            .execute_webhook(webhook.id, token)
            .content(translation)
            .username(&message.author.name)
            .avatar_url(get_avatar_url(&message.author))
            .await;

        // Increase metrics counter for this guild
        // BadTranslator is only available in guilds, so it's safe to unwrap
        let guild_id = message.guild_id.unwrap().0;
        let mut metrics_lock = assyst.metrics.write().await;
        *metrics_lock.bt_messages.0.entry(guild_id).or_insert(0) += 1;
    }
}

fn get_default_avatar_url(user: &User) -> String {
    // Unwrapping discrim parsing is ok, it should never be out of range or non-numeric
    format!(
        "https://cdn.discordapp.com/embed/avatars/{}.png",
        user.discriminator.parse::<u16>().unwrap() % 5
    )
}

fn get_avatar_url(user: &User) -> String {
    let avatar = match &user.avatar {
        Some(av) => av,
        None => return get_default_avatar_url(user),
    };

    let ext = if avatar.starts_with("a_") {
        "gif"
    } else {
        "png"
    };
    format!(
        "https://cdn.discordapp.com/avatars/{}/{}.{}",
        user.id, avatar, ext
    )
}

fn is_webhook(user: &User) -> bool {
    user.system.unwrap_or(false) || user.discriminator == "0000"
}

fn is_ratelimit_message(_: &Assyst, message: &MessageCreate) -> bool {
    // TODO: check if message was sent by the bot itself
    message.content.contains(constants::RATELIMITED_MESSAGE)
}
