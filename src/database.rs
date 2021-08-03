use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
};

use futures::StreamExt;
use sqlx::postgres::{PgPool, PgPoolOptions};
use tokio::sync::RwLock;
use twilight_model::id::GuildId;

use crate::{badtranslator::ChannelCache, caching::Cache, util::get_current_millis};

#[derive(sqlx::FromRow, Debug)]
pub struct DatabaseReminder {
    pub id: i32,
    pub user_id: i64,
    pub timestamp: i64,
    pub guild_id: i64,
    pub channel_id: i64,
    pub message_id: i64,
    pub message: String,
}

#[derive(sqlx::FromRow, Debug)]
pub struct Reminder {
    pub user_id: i64,
    pub timestamp: i64,
    pub guild_id: i64,
    pub channel_id: i64,
    pub message_id: i64,
    pub message: String,
}

impl From<DatabaseReminder> for Reminder {
    fn from(r: DatabaseReminder) -> Self {
        Reminder {
            user_id: r.user_id,
            timestamp: r.timestamp,
            guild_id: r.guild_id,
            channel_id: r.channel_id,
            message_id: r.message_id,
            message: r.message,
        }
    }
}

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct CommandUsage {
    pub command_name: String,
    pub uses: i32,
}

#[derive(sqlx::FromRow, Debug)]
pub struct BadTranslatorMessages {
    pub guild_id: i64,
    pub message_count: i64,
}

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct DisabledCommandEntry {
    pub command_name: String,
    pub guild_id: i64,
}

#[derive(sqlx::FromRow, Debug)]
pub struct FreeTier1Requests {
    pub user_id: i64,
    pub count: i32,
}

#[derive(sqlx::FromRow, Debug)]
pub struct Voter {
    pub user_id: i64,
    pub username: String,
    pub discriminator: String,
    pub count: i32,
}

type GuildDisabledCommands = Cache<GuildId, HashSet<String>>;

pub struct DatabaseCache {
    pub prefixes: HashMap<u64, Box<str>>,
    pub disabled_commands: GuildDisabledCommands,
}
impl DatabaseCache {
    pub fn new() -> Self {
        DatabaseCache {
            prefixes: HashMap::new(),
            disabled_commands: Cache::new(100),
        }
    }
}
pub struct Database {
    pub cache: RwLock<DatabaseCache>,
    pool: PgPool,
}
impl Database {
    pub async fn new(max_connections: u32, url: String) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(max_connections)
            .connect(&url)
            .await?;

        Ok(Database {
            cache: RwLock::new(DatabaseCache::new()),
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

    pub async fn delete_bt_channel(&self, id: u64) -> Result<(), sqlx::Error> {
        let query = r#"DELETE FROM bt_channels WHERE id = $1"#;

        sqlx::query(query)
            .bind(id as i64)
            .execute(&self.pool)
            .await
            .map(|_| ())
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
    ) -> Result<Vec<DatabaseReminder>, sqlx::Error> {
        let query = r#"SELECT * FROM reminders WHERE user_id = $1 ORDER BY timestamp ASC LIMIT $2"#;

        sqlx::query_as::<_, DatabaseReminder>(query)
            .bind(user as i64)
            .bind(count as i64)
            .fetch_all(&self.pool)
            .await
    }

    pub async fn delete_reminder_by_id(&self, user_id: u64, id: i32) -> Result<bool, sqlx::Error> {
        let query = r#"DELETE FROM reminders WHERE user_id = $1 AND id = $2 RETURNING *"#;

        sqlx::query(query)
            .bind(user_id as i64)
            .bind(id)
            .fetch_all(&self.pool)
            .await
            .map(|s| !s.is_empty())
    }

    pub async fn fetch_reminders(
        &self,
        time_delta: i64,
    ) -> Result<Vec<DatabaseReminder>, sqlx::Error> {
        let query = "SELECT * FROM reminders WHERE timestamp < $1";

        sqlx::query_as::<_, DatabaseReminder>(query)
            .bind(get_current_millis() as i64 + time_delta)
            .fetch_all(&self.pool)
            .await
    }

    pub async fn delete_reminders<R>(&self, reminders: Vec<R>) -> Result<(), sqlx::Error>
    where
        R: Into<Reminder>,
    {
        let mut tx = self.pool.begin().await?;

        for reminder in reminders {
            let reminder = reminder.into();
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

    pub async fn get_command_usage_stats_for(
        &self,
        command: &str,
    ) -> Result<CommandUsage, sqlx::Error> {
        let query = "SELECT * FROM command_uses where command_name = $1 order by uses desc";
        sqlx::query_as::<_, CommandUsage>(query)
            .bind(command)
            .fetch_one(&self.pool)
            .await
    }

    pub async fn increment_command_uses(&self, command: &str) -> Result<(), sqlx::Error> {
        let query = "insert into command_uses (command_name, uses) values ($1, 1) on conflict (command_name) do update set uses = command_uses.uses + 1 where command_uses.command_name = $1;";
        sqlx::query(query).bind(command).execute(&self.pool).await?;
        Ok(())
    }

    pub async fn increment_badtranslator_messages(&self, guild_id: u64) -> Result<(), sqlx::Error> {
        let query = "insert into bt_messages (guild_id, message_count) values ($1, 1) on conflict (guild_id) do update set message_count = bt_messages.message_count + 1 where bt_messages.guild_id = $1;";
        sqlx::query(query)
            .bind(guild_id as i64)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn get_badtranslator_message_stats(
        &self,
        guild_id: u64,
    ) -> Result<(u64, u64), sqlx::Error> {
        let query = "select * from bt_messages";
        let result = sqlx::query_as::<_, BadTranslatorMessages>(query)
            .fetch_all(&self.pool)
            .await?;
        let total_messages: i64 = result.iter().map(|i| i.message_count).sum();
        let guild_messages: i64 = result
            .iter()
            .find(|i| i.guild_id == guild_id as i64)
            .map(|i| i.message_count)
            .unwrap_or_default();

        Ok((total_messages as u64, guild_messages as u64))
    }

    pub async fn get_badtranslator_messages_raw(
        &self,
    ) -> Result<Vec<BadTranslatorMessages>, sqlx::Error> {
        let query = "select * from bt_messages order by message_count desc";
        let result = sqlx::query_as::<_, BadTranslatorMessages>(query)
            .fetch_all(&self.pool)
            .await?;

        Ok(result)
    }

    pub async fn is_command_disabled(&self, command: &str, guild_id: GuildId) -> bool {
        let lock = self.cache.read().await;

        let guild_disabled_commands = lock.disabled_commands.get(&guild_id);

        if let Some(commands) = guild_disabled_commands {
            return if commands.contains(command) {
                true
            } else {
                false
            };
        }

        drop(lock);

        let query = "select * from disabled_commands where guild_id = $1";

        let result: Vec<DisabledCommandEntry> = sqlx::query_as::<_, DisabledCommandEntry>(query)
            .bind(guild_id.0 as i64)
            .fetch_all(&self.pool)
            .await
            .unwrap();

        let is_disabled = result.iter().any(|cmd| &cmd.command_name == command);

        let mut write_lock = self.cache.write().await;
        let guild = write_lock.disabled_commands.get(&guild_id);

        let mut commands = HashSet::new();
        for command in result {
            commands.insert(command.command_name);
        }

        if guild.is_none() {
            write_lock.disabled_commands.insert(guild_id, commands);
        };

        is_disabled
    }

    pub async fn add_disabled_command(
        &self,
        guild_id: GuildId,
        command: &str,
    ) -> Result<(), sqlx::Error> {
        let query = "insert into disabled_commands(guild_id, command_name) values($1, $2)";

        sqlx::query(query)
            .bind(guild_id.0 as i64)
            .bind(command)
            .execute(&self.pool)
            .await?;

        let mut write_lock = self.cache.write().await;
        let guild = (*write_lock).disabled_commands.get_mut(&guild_id);

        if let Some(cmds) = guild {
            cmds.insert(command.to_string());
        }

        Ok(())
    }

    pub async fn remove_disabled_command(
        &self,
        guild_id: GuildId,
        command: &str,
    ) -> Result<(), sqlx::Error> {
        let query = "delete from disabled_commands where guild_id = $1 and command_name = $2";

        sqlx::query(query)
            .bind(guild_id.0 as i64)
            .bind(command)
            .execute(&self.pool)
            .await?;

        let mut write_lock = self.cache.write().await;
        let guild = (*write_lock).disabled_commands.get_mut(&guild_id);

        if let Some(cmds) = guild {
            cmds.remove(command);
        }

        Ok(())
    }

    pub async fn add_free_tier_1_requests(
        &self,
        user_id: i64,
        add: i64,
    ) -> Result<(), sqlx::Error> {
        let query = "insert into free_tier1_requests values($1, $2) on conflict (user_id) do update set count = free_tier1_requests.count + $2 where free_tier1_requests.user_id = $1";

        sqlx::query(query)
            .bind(user_id)
            .bind(add)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn get_and_subtract_free_tier_1_request(&self, user_id: i64) -> bool {
        let result = self.get_user_free_tier1_requests(user_id).await;

        if result == 0 {
            false
        } else {
            let new_count = result - 1;

            if new_count < 1 {
                self.delete_user_from_free_tier_1_requests(user_id)
                    .await
                    .unwrap();
            } else {
                self.add_free_tier_1_requests(user_id, -1).await.unwrap();
            }

            true
        }
    }

    pub async fn get_user_free_tier1_requests(&self, user_id: i64) -> i32 {
        let fetch_query = "select * from free_tier1_requests where user_id = $1";

        let result: Vec<FreeTier1Requests> = sqlx::query_as::<_, FreeTier1Requests>(fetch_query)
            .bind(user_id)
            .fetch_all(&self.pool)
            .await
            .unwrap();

        if result.is_empty() {
            0
        } else {
            result[0].count
        }
    }

    pub async fn delete_user_from_free_tier_1_requests(
        &self,
        user_id: i64,
    ) -> Result<(), sqlx::Error> {
        let query = "delete from free_tier1_requests where user_id = $1";

        sqlx::query(query).bind(user_id).execute(&self.pool).await?;

        Ok(())
    }

    pub async fn increment_user_votes(&self, user_id: i64, username: &str, discriminator: &str) {
        let query = "insert into user_votes values($1, $2, $3, $4) on conflict (user_id) do update set count = user_votes.count + 1 where user_votes.user_id = $1";

        sqlx::query(query)
            .bind(user_id)
            .bind(username)
            .bind(discriminator)
            .execute(&self.pool)
            .await
            .unwrap();
    }

    pub async fn get_voters(&self) -> Vec<Voter> {
        let fetch_query = "select * from user_votes order by count desc";

        let result: Vec<Voter> = sqlx::query_as::<_, Voter>(fetch_query)
            .fetch_all(&self.pool)
            .await
            .unwrap();
        
        return result;
    }
}
