use twilight_model::gateway::payload::MessageCreate;
use std::sync::Arc;
use crate::Assyst;

pub async fn handle(
    assyst: Arc<Assyst>,
    message: Box<MessageCreate>
) -> () {
   if !should_handle_message(&message).await { return };
   assyst.handle_command(message.0).await;
}

async fn should_handle_message(
    message: &Box<MessageCreate>
) -> bool {
    if message.author.bot 
    || message.author.discriminator == "0000"
    || message.guild_id == None
    {
        false
    } else {
        true
    }
}