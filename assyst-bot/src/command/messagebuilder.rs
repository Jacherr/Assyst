use twilight_model::channel::embed::Embed;

#[derive(Clone, Debug)]
pub struct Attachment {
    pub name: Box<str>,
    pub data: Vec<u8>,
}

#[derive(Clone, Debug)]
pub struct MessageBuilder {
    pub attachment: Option<Attachment>,
    pub content: Option<Box<str>>,
    pub embed: Option<Embed>,
}
impl MessageBuilder {
    pub fn new() -> Self {
        MessageBuilder {
            attachment: None,
            content: None,
            embed: None,
        }
    }

    pub fn attachment(mut self, name: Box<str>, value: Vec<u8>) -> Self {
        self.attachment = Some(Attachment { name, data: value });
        self
    }

    pub fn content(mut self, content: Box<str>) -> Self {
        self.content = Some(content);
        self
    }
}
