use twilight_model::gateway::payload::MessageCreate;
use std::sync::Arc;
use crate::Assyst;

pub async fn handle(
    assyst: Arc<Assyst>,
    message: Box<MessageCreate>
) -> () {
   if !should_handle_message(&message).await { return };
   if let Err(e) = assyst.handle_command(message.0).await {
       println!("Command execution failed: {:?}", e);
   }
}

async fn should_handle_message(
    message: &Box<MessageCreate>
) -> bool {
    !message.author.bot && message.author.discriminator != "0000" && message.guild_id.is_some()
}