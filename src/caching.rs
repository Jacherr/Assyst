use std::{collections::HashMap, u64};
use tokio::sync::Mutex;
use twilight_model::channel::Message;
use util::get_current_millis;
use std::sync::Arc;

use crate::util;

pub const MESSAGE_EDIT_HANDLE_LIMIT: u32 = 60000;

pub struct Replies {
    cache: HashMap<u64, Arc<Mutex<Reply>>>
}
impl Replies {
    pub fn new() -> Self {
        Replies {
            cache: HashMap::new()
        }
    }
    pub fn get_or_set_reply(&mut self, reply_to_insert: Reply) -> &mut Arc<Mutex<Reply>> {
        self.cache.entry(reply_to_insert.invocation.id.0)
            .or_insert_with(|| Arc::new(Mutex::new(reply_to_insert)))
    }
}
pub struct Reply {
    invocation: Arc<Message>,
    pub reply: Option<Arc<Message>>,
    expire: u64,
    pub in_use: bool
}
impl Reply {
    pub fn new(invocation: Arc<Message>) -> Self {
        Reply {
            invocation,
            reply: None,
            expire: get_current_millis() + MESSAGE_EDIT_HANDLE_LIMIT as u64,
            in_use: false
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