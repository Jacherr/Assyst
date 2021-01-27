use twilight_model::gateway::payload::MessageCreate;
use std::sync::Arc;
use crate::Assyst;

pub async fn handle(
    assyst: Arc<Assyst>,
    message: Box<MessageCreate>
) {
   if !should_handle_message(&assyst, &message).await { return };
   
}

async fn should_handle_message(
    assyst: &Arc<Assyst>,
    message: &Box<MessageCreate>
) -> bool {
    if let Some(id) = message.guild_id {
        let prefix = assyst.database.get_or_set_prefix_for(id.0, &assyst.config.default_prefix).await;
        let prefix_is_valid = prefix.ok().flatten().and_then(|x| Some(x.starts_with("a"))).unwrap_or(false);
        if !prefix_is_valid {
            return false;
        }
    }

    if message.author.bot 
    || message.author.discriminator == "0000"
    {
        false
    } else {
        true
    }
}