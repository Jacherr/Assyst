use crate::Assyst;
use assyst_logger as logger;

/// A thin wrapper around the `assyst_logger` crate. This makes the logger nicer to use in this crate.
pub struct Logger;
impl Logger {
    pub async fn panic(&self, assyst: &Assyst, message: &str) {
        logger::panic(&assyst.config, &assyst.http, message).await
    }

    pub async fn fatal(&self, assyst: &Assyst, message: &str) {
        logger::fatal(&assyst.config, &assyst.http, message).await
    }

    pub async fn info(&self, assyst: &Assyst, message: &str) {
        logger::info(&assyst.config, &assyst.http, message).await
    }

    pub async fn guild_add(&self, assyst: &Assyst, message: &str) {
        logger::guild_add(&assyst.config, &assyst.http, message).await
    }

    pub async fn log_vote(&self, assyst: &Assyst, message: &str) {
        logger::log_vote(&assyst.config, &assyst.http, message).await
    }
}
