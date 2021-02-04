use std::collections::HashMap;
use twilight_model::channel::Message;
use std::sync::Arc;
pub struct Replies {
    cache: HashMap<u64, Reply>
}
impl Replies {
    pub fn new() -> Self {
        Replies {
            cache: HashMap::new()
        }
    }
    pub fn get_or_set_reply(&mut self, reply_to_insert: Reply) -> &mut Reply {
        self.cache.entry(reply_to_insert.invocation.id.0)
            .or_insert_with(|| reply_to_insert)
    }
}
pub struct Reply {
    invocation: Arc<Message>,
    reply: Option<Message>
}
impl Reply {
    pub fn new(invocation: Arc<Message>) -> Self {
        Reply {
            invocation,
            reply: None
        }
    }
    pub fn has_replied(&self) -> bool {
        self.reply.is_some()
    }
    pub fn set_reply(&mut self, reply: Message) -> &mut Self {
        self.reply = Some(reply);
        self
    }
}