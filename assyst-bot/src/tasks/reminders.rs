use crate::{assyst::Assyst, logger, util::message_link};
use assyst_common::util::{ChannelId, UserId};
use assyst_database::DatabaseReminder;
use std::{sync::Arc, time::Duration};
use tokio::time::sleep;
use twilight_model::channel::message::AllowedMentions;

const FETCH_INTERVAL: i64 = 30000;

async fn process_single_reminder(
    assyst: &Arc<Assyst>,
    reminder: &DatabaseReminder,
) -> Result<(), Box<dyn std::error::Error>> {
    assyst
        .http
        .create_message(ChannelId::new(reminder.channel_id as u64))
        .allowed_mentions(Some(&AllowedMentions {
            parse: vec![],
            replied_user: false,
            roles: vec![],
            users: vec![UserId::new(reminder.user_id as u64)],
        }))
        .content(&format!(
            "<@{}> Reminder: {}\n{}",
            reminder.user_id,
            reminder.message,
            message_link(
                reminder.guild_id as u64,
                reminder.channel_id as u64,
                reminder.message_id as u64
            )
        ))?
        .await?;

    Ok(())
}

async fn process_reminders(
    assyst: &Arc<Assyst>,
    reminders: Vec<DatabaseReminder>,
) -> Result<(), anyhow::Error> {
    if reminders.len() < 1 {
        return Ok(());
    }

    for reminder in &reminders {
        if let Err(e) = process_single_reminder(assyst, &reminder).await {
            eprintln!("Failed to process reminder: {:?}", e);
        }
    }

    // Once we're done, delete them from database
    Ok(assyst.database.delete_reminders(reminders).await?)
}

pub fn init_reminder_loop(assyst: Arc<Assyst>) {
    if !assyst.config.disable_reminder_check {
        tokio::spawn(async move {
            let assyst = assyst.clone();

            loop {
                let reminders = assyst.database.fetch_reminders(FETCH_INTERVAL).await;

                match reminders {
                    Ok(reminders) => {
                        if let Err(e) = process_reminders(&assyst, reminders).await {
                            logger::fatal(
                                &assyst,
                                &format!("Processing reminder queue failed: {:?}", e),
                            )
                            .await;
                        }
                    }
                    Err(e) => {
                        logger::fatal(&assyst, &format!("Fetching reminders failed: {:?}", e))
                            .await;
                    }
                }

                sleep(Duration::from_millis(FETCH_INTERVAL as u64)).await;
            }
        });
    }
}
