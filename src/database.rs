use std::{borrow::Cow, collections::HashMap};

use futures::StreamExt;
use sqlx::postgres::{PgPool, PgPoolOptions};
use tokio::sync::RwLock;

use crate::{badtranslator::ChannelCache, util::get_current_millis};

#[derive(sqlx::FromRow, Debug)]
pub struct Reminder {
    pub user_id: i64,
    pub timestamp: i64,
    pub guild_id: i64,
    pub channel_id: i64,
    pub message_id: i64,
    pub message: String,
}
#[derive(sqlx::FromRow, Debug)]
pub struct CommandUsage {
    pub command_name: String,
    pub uses: i32,
}

struct Cache {
    pub prefixes: HashMap<u64, Box<str>>,
}
impl Cache {
    pub fn new() -> Self {
        Cache {
            prefixes: HashMap::new(),
        }
    }
}
pub struct Database {
    cache: RwLock<Cache>,
    pool: PgPool,
}
impl Database {
    pub async fn new(max_connections: u32, url: String) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(max_connections)
            .connect(&url)
            .await?;

        Ok(Database {
            cache: RwLock::new(Cache::new()),
            pool,
        })
    }

    pub async fn get_or_set_prefix_for<'a, 'b>(
        &'a self,
        guild_id: u64,
        set_prefix: &'b str,
    ) -> Result<Option<Cow<'b, str>>, sqlx::error::Error> {
        let cache_lock = self.cache.read().await;
        let try_fetch_cache = cache_lock.prefixes.get(&guild_id);
        if let Some(prefix) = try_fetch_cache {
            return Ok(Some(Cow::Owned(prefix.to_string())));
        }
        drop(cache_lock);

        let query = "
            SELECT *
            FROM prefixes
            WHERE guild = $1
            ";

        return match sqlx::query_as::<_, (String,)>(query)
            .bind(guild_id as i64)
            .fetch_one(&self.pool)
            .await
        {
            Ok(res) => {
                let mut cache_lock = self.cache.write().await;
                cache_lock
                    .prefixes
                    .insert(guild_id, res.0.clone().into_boxed_str());
                Ok(Some(Cow::Owned(res.0)))
            }
            Err(sqlx::Error::RowNotFound) => {
                self.set_prefix_for(guild_id, set_prefix).await?;
                Ok(Some(Cow::Borrowed(set_prefix)))
            }
            Err(err) => Err(err),
        };
    }

    pub async fn set_prefix_for(
        &self,
        guild_id: u64,
        prefix: &str,
    ) -> Result<(), sqlx::error::Error> {
        let mut cache_lock = self.cache.write().await;
        cache_lock
            .prefixes
            .insert(guild_id, prefix.to_owned().into_boxed_str());

        let query = r#" INSERT INTO prefixes(guild, prefix) VALUES($1, $2) ON CONFLICT (guild) DO UPDATE SET prefix = $2 WHERE prefixes.guild = $1 "#;

        sqlx::query(query)
            .bind(guild_id as i64)
            .bind(prefix)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn get_bt_channels(&self) -> Result<ChannelCache, sqlx::Error> {
        let query = "SELECT id FROM bt_channels";

        let mut channels: ChannelCache = HashMap::new();

        let mut rows = sqlx::query_as::<_, (i64,)>(query).fetch(&self.pool);

        while let Some(Ok(row)) = rows.next().await {
            channels.insert(row.0 as u64, None);
        }

        Ok(channels)
    }

    pub async fn add_bt_channel(&self, id: u64) -> Result<(), sqlx::Error> {
        let query = r#"INSERT INTO bt_channels VALUES ($1)"#;

        sqlx::query(query)
            .bind(id as i64)
            .execute(&self.pool)
            .await
            .and_then(|_| Ok(()))
    }

    pub async fn add_reminder(&self, reminder: Reminder) -> Result<(), sqlx::Error> {
        let query = r#"INSERT INTO reminders VALUES ($1, $2, $3, $4, $5, $6)"#;

        sqlx::query(query)
            .bind(reminder.user_id)
            .bind(reminder.timestamp)
            .bind(reminder.guild_id)
            .bind(reminder.channel_id)
            .bind(reminder.message_id)
            .bind(&*reminder.message)
            .execute(&self.pool)
            .await
            .and_then(|_| Ok(()))
    }

    pub async fn fetch_user_reminders(
        &self,
        user: u64,
        count: u64,
    ) -> Result<Vec<Reminder>, sqlx::Error> {
        let query = r#"SELECT * FROM reminders WHERE user_id = $1 ORDER BY timestamp ASC LIMIT $2"#;

        sqlx::query_as::<_, Reminder>(query)
            .bind(user as i64)
            .bind(count as i64)
            .fetch_all(&self.pool)
            .await
    }

    pub async fn fetch_reminders(&self, time_delta: i64) -> Result<Vec<Reminder>, sqlx::Error> {
        let query = "SELECT * FROM reminders WHERE timestamp < $1";

        sqlx::query_as::<_, Reminder>(query)
            .bind(get_current_millis() as i64 + time_delta)
            .fetch_all(&self.pool)
            .await
    }

    pub async fn delete_reminders(&self, reminders: Vec<Reminder>) -> Result<(), sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        println!("Deleting {:?}", reminders);

        for reminder in reminders {
            sqlx::query("DELETE FROM reminders WHERE message_id = $1 AND channel_id = $2")
                .bind(reminder.message_id)
                .bind(reminder.channel_id)
                .execute(&mut tx)
                .await?;
        }

        tx.commit().await
    }

    pub async fn get_command_usage_stats(&self) -> Result<Vec<CommandUsage>, sqlx::Error> {
        let query = "SELECT * FROM command_uses order by uses desc";
        sqlx::query_as::<_, CommandUsage>(query)
            .fetch_all(&self.pool)
            .await
    }

    pub async fn increment_command_uses(&self, command: &str) -> Result<(), sqlx::Error> {
        let query = "insert into command_uses (command_name, uses) values ($1, 1) on conflict (command_name) do update set uses = command_uses.uses + 1 where command_uses.command_name = $1;";
        sqlx::query(query).bind(command).execute(&self.pool).await?;
        Ok(())
    }
}
