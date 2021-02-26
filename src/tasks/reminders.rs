use crate::{assyst::Assyst, database::Reminder};
use std::{sync::Arc, time::Duration};
use tokio::time::sleep;
use twilight_model::id::ChannelId;

const FETCH_INTERVAL: i64 = 30000;

async fn process_single_reminder(
    assyst: &Arc<Assyst>,
    reminder: &Reminder,
) -> Result<(), Box<dyn std::error::Error>> {
    assyst
        .http
        .create_message(ChannelId(reminder.channel_id as u64))
        .allowed_mentions()
        .build()
        .content(&format!(
            "<@{}> Reminder: {}",
            reminder.user_id, reminder.message
        ))?
        .await?;

    Ok(())
}

async fn process_reminders(
    assyst: &Arc<Assyst>,
    reminders: Vec<Reminder>,
) -> Result<(), sqlx::Error> {
    if reminders.len() < 1 {
        return Ok(());
    }

    for reminder in &reminders {
        if let Err(e) = process_single_reminder(assyst, &reminder).await {
            eprintln!("Failed to process reminder: {:?}", e);
        }
    }

    // Once we're done, delete them from database
    assyst.database.delete_reminders(reminders).await
}

pub fn init_reminder_loop(assyst: Arc<Assyst>) {
    tokio::spawn(async move {
        let assyst = assyst.clone();

        loop {
            let reminders = assyst.database.fetch_reminders(FETCH_INTERVAL).await;

            match reminders {
                Ok(reminders) => {
                    if let Err(e) = process_reminders(&assyst, reminders).await {
                        println!("Processing reminder queue failed: {:?}", e);
                    }
                }
                Err(e) => {
                    eprintln!("Fetching reminders failed: {:?}", e);
                }
            }

            sleep(Duration::from_millis(FETCH_INTERVAL as u64)).await;
        }
    });
}
