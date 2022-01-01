use std::collections::HashMap;

use twilight_model::channel::Webhook;

#[derive(Debug, Clone)]
pub struct BadTranslatorEntry {
    pub webhook: Option<Webhook>,
    pub language: Box<str>,
}

impl BadTranslatorEntry {
    pub fn with_language(language: impl Into<Box<str>>) -> Self {
        Self {
            webhook: None,
            language: language.into(),
        }
    }

    pub fn zip(self) -> Option<(Webhook, Box<str>)> {
        let language = self.language;
        self.webhook.map(|webhook| (webhook, language))
    }
}

type Snowflake = u64;
pub type ChannelCache = HashMap<Snowflake, BadTranslatorEntry>;
