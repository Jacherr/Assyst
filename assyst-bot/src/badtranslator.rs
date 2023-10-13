use crate::assyst::Assyst;
use crate::util::{self, normalize_mentions};
use crate::{
    rest::bt, util::get_current_millis, util::normalize_emojis, util::sanitize_message_content,
};
use anyhow::Context;
use assyst_common::bt::{BadTranslatorEntry, ChannelCache};
use assyst_common::consts;
use assyst_common::util::{ChannelId, UserId};
use std::collections::HashMap;
use std::{borrow::Cow, time::Duration};
use tokio::sync::RwLock;
use twilight_http::error::ErrorType;
use twilight_model::gateway::payload::incoming::MessageCreate;
use twilight_model::channel::Webhook;

mod flags {
    pub const DISABLED: u32 = 0x1;
}

#[derive(Debug)]
struct BadTranslatorRatelimit(u64);

impl BadTranslatorRatelimit {
    pub fn new() -> Self {
        Self(get_current_millis())
    }

    pub fn expired(&self) -> bool {
        get_current_millis() - self.0 >= consts::BT_RATELIMIT_LEN
    }
}

pub struct BadTranslator {
    flags: RwLock<u32>,
    channels: RwLock<ChannelCache>,
    ratelimits: RwLock<HashMap<u64, BadTranslatorRatelimit>>,
}

type Snowflake = u64;

impl BadTranslator {
    pub fn new() -> Self {
        Self::with_channels(HashMap::new())
    }

    pub async fn add_channel(&self, id: Snowflake, language: &str) {
        if !self.is_disabled().await {
            let mut lock = self.channels.write().await;
            lock.insert(id, BadTranslatorEntry::with_language(language));
        }
    }

    pub async fn set_channel_language(&self, id: u64, language: impl Into<Box<str>>) {
        let mut lock = self.channels.write().await;
        lock.entry(id).and_modify(|e| e.language = language.into());
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

    pub async fn get_or_fetch_entry(
        &self,
        assyst: &Assyst,
        id: &ChannelId,
    ) -> Option<(Webhook, Box<str>)> {
        {
            // This is its own scope so that the cache lock gets dropped early
            let cache = self.channels.read().await;

            // In the perfect case where we already have the webhook cached, we can just return early
            if let Some(entry) = cache.get(&id.get()).cloned() {
                if let Some(entry) = entry.zip() {
                    return Some(entry);
                }
            }
        }

        // TODO: maybe return Result?
        let webhooks = assyst
            .http
            .channel_webhooks(*id)
            .await
            .ok()?
            .models()
            .await
            .ok()?;

        let webhook = webhooks.into_iter().find(|w| w.token.is_some())?;

        let mut cache = self.channels.write().await;
        let entry = cache
            .get_mut(&id.get())
            .expect("This can't fail, and if it does then that's a problem.");

        entry.webhook = Some(webhook.clone());

        Some((webhook, entry.language.clone()))
    }

    pub async fn remove_bt_channel(&self, id: u64) {
        self.channels.write().await.remove(&id);
    }

    pub async fn delete_bt_channel(&self, assyst: &Assyst, id: &ChannelId) -> anyhow::Result<()> {
        assyst
            .database
            .delete_bt_channel(id.get())
            .await
            .context("Deleting BT channel failed")?;

        self.channels.write().await.remove(&id.get());
        Ok(())
    }

    /// Returns true if given user ID is ratelimited
    pub async fn try_ratelimit(&self, id: &UserId) -> bool {
        let mut cache = self.ratelimits.write().await;

        if let Some(entry) = cache.get(&id.get()) {
            let expired = entry.expired();

            if !expired {
                return true;
            } else {
                cache.remove(&id.get());
                return false;
            }
        }

        cache.insert(id.get(), BadTranslatorRatelimit::new());

        false
    }

    pub async fn handle_message(
        &self,
        assyst: &Assyst,
        message: Box<MessageCreate>,
    ) -> anyhow::Result<()> {
        // We're assuming the caller has already made sure this is a valid channel
        // So we don't check if it's a BT channel again

        let guild_id = message.guild_id.expect("There are no BT channels in DMs");

        if is_webhook(&message) || is_ratelimit_message(assyst, &message) {
            // ignore its own webhook/ratelimit messages
            return Ok(());
        }

        if message.content.is_empty() || message.author.bot {
            let _ = assyst
                .http
                .delete_message(message.channel_id, message.id)
                .await;

            return Ok(());
        }

        let ratelimit = self.try_ratelimit(&message.author.id).await;
        if ratelimit {
            // delete source, respond with error, wait, delete error

            let _ = assyst
                .http
                .delete_message(message.channel_id, message.id)
                .await;

            let response = assyst
                .http
                .create_message(message.channel_id)
                .content(&format!(
                    "<@{}>, {}",
                    message.author.id.get(),
                    consts::BT_RATELIMIT_MESSAGE
                ))?
                .await?
                .model()
                .await?;

            tokio::time::sleep(Duration::from_secs(5)).await;

            assyst
                .http
                .delete_message(message.channel_id, response.id)
                .await?;

            return Ok(());
        }

        let content = normalize_emojis(&message.content);
        let content = normalize_mentions(&content, &message.mentions);

        let (webhook, language) = match self.get_or_fetch_entry(assyst, &message.channel_id).await {
            Some(webhook) => webhook,
            None => return self.delete_bt_channel(assyst, &message.channel_id).await,
        };

        let translation = match bt::bad_translate_debug(
            &assyst.reqwest_client,
            &content,
            message.author.id.get(),
            guild_id.get(),
            &language,
        )
        .await
        {
            Ok(res) => Cow::Owned(res.result.text),
            Err(bt::TranslateError::Raw(msg)) => Cow::Borrowed(msg),
            _ => return Ok(()),
        };

        let delete_state = assyst
            .http
            .delete_message(message.channel_id, message.id)
            .await;

        // dont respond with translation if the source was prematurely deleted
        if let Err(ErrorType::Response {
            status,
            body: _,
            error: _,
        }) = delete_state.as_ref().map_err(|e| e.kind())
        {
            if status.get() == 404 {
                return Ok(());
            }
        }

        let token = webhook.token.as_ref().context("Failed to extract token")?;

        let translation =
            sanitize_message_content(translation.chars().take(2000).collect::<String>().as_str());

        assyst
            .http
            .execute_webhook(webhook.id, token)
            .content(&translation)?
            .username(&message.author.name)?
            .avatar_url(&util::get_avatar_url(&message.author))
            .await
            .context("Executing webhook failed")?;

        // Increase metrics counter for this guild
        register_badtranslated_message_to_db(&assyst, guild_id.get())
            .await
            .with_context(|| format!("Error updating BT message metric for {}", guild_id))?;

        Ok(())
    }
}

async fn register_badtranslated_message_to_db(
    assyst: &Assyst,
    guild_id: u64,
) -> Result<(), sqlx::Error> {
    assyst
        .database
        .increment_badtranslator_messages(guild_id)
        .await
}

fn is_webhook(message: &MessageCreate) -> bool {
    message.author.system.unwrap_or(false) || message.webhook_id.is_some()
}

fn is_ratelimit_message(assyst: &Assyst, message: &MessageCreate) -> bool {
    // TODO: check if message was sent by the bot itself
    message.content.contains(consts::BT_RATELIMIT_MESSAGE)
        && message.author.id.get() == assyst.config.bot_id
}
