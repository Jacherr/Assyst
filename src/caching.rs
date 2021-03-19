use std::sync::Arc;
use std::{collections::HashMap, hash::Hash, u64, usize};
use tokio::sync::Mutex;
use twilight_model::channel::Message;
use twilight_model::id::MessageId;
use util::get_current_millis;

use crate::{box_str, util};

pub const MESSAGE_EDIT_HANDLE_LIMIT: u32 = 60000;
pub const PER_GUILD_COMMAND_RATELIMIT: u32 = 2000;

pub struct Cache<T, U> {
    pub cache: HashMap<T, U>,
    pub limit: usize,
    key_queue: Vec<T>,
}
impl<T: Hash + Eq + Clone, U> Cache<T, U> {
    pub fn new(limit: usize) -> Self {
        Cache {
            cache: HashMap::new(),
            limit,
            key_queue: Vec::new(),
        }
    }

    pub fn insert(&mut self, key: T, value: U) {
        if let Some(i) = self.key_queue.iter().position(|e| *e == key) {
            self.key_queue.drain(i..(i + 1));
            self.key_queue.push(key.clone());
        };

        if self.cache.len() == self.limit {
            self.cache.remove(self.key_queue.pop().as_ref().unwrap());
        }

        self.cache.insert(key, value);
    }
}

#[derive(Debug)]
pub struct Ratelimits {
    cache: HashMap<u64, GuildRatelimits>,
}
impl Ratelimits {
    pub fn new() -> Self {
        Ratelimits {
            cache: HashMap::new(),
        }
    }

    pub fn set_command_expire_at(
        &mut self,
        guild_id: twilight_model::id::GuildId,
        command: &str,
    ) -> () {
        self.cache
            .entry(guild_id.0)
            .or_insert_with(|| GuildRatelimits::new())
            .set_command_expiry(
                command,
                get_current_millis() + PER_GUILD_COMMAND_RATELIMIT as u64,
            );
    }

    pub fn time_until_guild_command_usable(
        &self,
        guild_id: twilight_model::id::GuildId,
        command: &str,
    ) -> Option<u64> {
        let guild_ratelimits = self.cache.get(&guild_id.0)?;
        let command_ratelimit = guild_ratelimits.get_command_expiry(command)?;
        let millis = get_current_millis();
        if millis > *command_ratelimit {
            None
        } else {
            Some(*command_ratelimit - millis)
        }
    }
}
#[derive(Debug)]
pub struct GuildRatelimits {
    cache: HashMap<Box<str>, u64>,
}
impl GuildRatelimits {
    pub fn new() -> Self {
        GuildRatelimits {
            cache: HashMap::new(),
        }
    }
    pub fn get_command_expiry(&self, command: &str) -> Option<&u64> {
        self.cache.get(&box_str!(command))
    }
    pub fn set_command_expiry(&mut self, command: &str, expiry: u64) -> () {
        self.cache.insert(box_str!(command), expiry);
    }
}

pub struct Replies {
    cache: HashMap<u64, Arc<Mutex<Reply>>>,
}
impl Replies {
    pub fn new() -> Self {
        Replies {
            cache: HashMap::new(),
        }
    }

    pub async fn garbage_collect(&mut self) {
        let entries = self
            .cache
            .iter()
            .map(|a| (a.0.clone(), a.1.clone()))
            .collect::<Vec<_>>();
        for (key, value) in entries {
            if value.lock().await.has_expired() {
                self.cache.remove(&key);
            };
        }
    }

    pub fn get_or_set_reply(&mut self, reply_to_insert: Reply) -> &mut Arc<Mutex<Reply>> {
        self.cache
            .entry(reply_to_insert.invocation.id.0)
            .or_insert_with(|| Arc::new(Mutex::new(reply_to_insert)))
    }

    pub async fn get_reply_from_invocation_id(&self, id: MessageId) -> Option<Arc<Mutex<Reply>>> {
        self.cache.get(&id.0).and_then(|r| Some(r.clone()))
    }
}
pub struct Reply {
    invocation: Arc<Message>,
    pub reply: Option<Arc<Message>>,
    expire: u64,
    pub in_use: bool,
}
impl Reply {
    pub fn new(invocation: Arc<Message>) -> Self {
        Reply {
            invocation,
            reply: None,
            expire: get_current_millis() + MESSAGE_EDIT_HANDLE_LIMIT as u64,
            in_use: false,
        }
    }
    pub fn has_expired(&self) -> bool {
        self.expire < get_current_millis()
    }
    pub fn has_replied(&self) -> bool {
        self.reply.is_some()
    }
    pub fn set_reply(&mut self, reply: Arc<Message>) -> &mut Self {
        self.reply = Some(reply);
        self
    }
}
