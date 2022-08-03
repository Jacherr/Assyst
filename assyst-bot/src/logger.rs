use crate::Assyst;
use assyst_logger as logger;

/// A thin wrapper around the `assyst_logger` crate. This makes the logger nicer to use in this crate,
/// because the caller only needs to pass a reference to the `Assyst` struct.

#[inline]
pub async fn panic(assyst: &Assyst, message: &str) {
    logger::panic(&assyst.config, &assyst.http, message).await
}

#[inline]
pub async fn fatal(assyst: &Assyst, message: &str) {
    logger::fatal(&assyst.config, &assyst.database, message).await
}

#[inline]
pub async fn info(assyst: &Assyst, message: &str) {
    logger::info(&assyst.config, &assyst.database, message).await
}

#[inline]
pub async fn guild_add(assyst: &Assyst, message: &str) {
    logger::guild_add(&assyst.config, &assyst.database, message).await
}

#[inline]
pub async fn log_vote(assyst: &Assyst, message: &str) {
    logger::log_vote(&assyst.config, &assyst.http, message).await
}

#[inline]
pub async fn log_command_use(assyst: &Assyst, message: &str) {
    logger::command_use(&assyst.config, &assyst.database, message).await
}