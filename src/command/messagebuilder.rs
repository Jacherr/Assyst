use twilight_model::channel::embed::Embed;
use crate::box_str;
#[derive(Clone)]
pub struct Attachment {
    pub name: Box<str>,
    pub data: Vec<u8>
}

#[derive(Clone)]
pub struct MessageBuilder {
    pub attachment: Option<Attachment>,
    pub content: Option<Box<str>>,
    pub embed: Option<Embed>
}
impl MessageBuilder {
    pub fn new() -> Self {
        MessageBuilder {
            attachment: None,
            content: None,
            embed: None
        }
    }

    pub fn attachment(&mut self, name: &str, value: Vec<u8>) -> &mut Self {
        self.attachment = Some(Attachment { name: box_str!(name), data: value });
        self
    }

    pub fn content(&mut self, content: &str) -> &mut Self {
        self.content = Some(box_str!(content));
        self
    }
}