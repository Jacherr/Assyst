use crate::Assyst;
use std::sync::Arc;
use twilight_model::gateway::payload::MessageCreate;

pub async fn handle(assyst: Arc<Assyst>, message: Box<MessageCreate>) {
    // If this is the first message, we want to populate the BT channel cache
    // If it fails, we just log the error and disable it
    if assyst.badtranslator.should_fetch().await {
        match assyst.database.get_bt_channels().await {
            Ok(channels) => assyst.badtranslator.set_channels(channels).await,
            Err(e) => {
                assyst
                    .logger
                    .fatal(
                        &assyst,
                        &format!(
                            "Fetching BadTranslator channels failed, disabling feature... {:?}",
                            e
                        ),
                    )
                    .await;
                assyst.badtranslator.disable().await;
            }
        }
    }

    if assyst.badtranslator.is_channel(message.channel_id.0).await {
        assyst.badtranslator.handle_message(&assyst, message).await;
        return;
    }

    // Regular commands
    if !should_handle_message(&message).await {
        return;
    };

    if let Err(e) = assyst.handle_command(message.0).await {
        assyst
            .logger
            .fatal(&assyst, &format!("Command execution failed: {:?}", e))
            .await;
    }
}

async fn should_handle_message(message: &Box<MessageCreate>) -> bool {
    !message.author.bot && message.author.discriminator != "0000" && message.guild_id.is_some()
}
