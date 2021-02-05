use std::{borrow::Cow, collections::HashMap};

use sqlx::postgres::{PgPool, PgPoolOptions};
use tokio::sync::RwLock;
use futures::StreamExt;

use crate::badtranslator::ChannelCache;

macro_rules! generate_query_task {
    ($query:expr, $pool:expr, $ret:tt, $($v:expr),+) => {{
        let query = $query;
        return match sqlx::query_as::<_, $ret>(query)
            $(.bind($v))+
            .fetch_one($pool)
            .await
            {
                Ok(_) => Ok(()),
                Err(err) => Err(err),
            };
    }}
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
            return Ok(Some(Cow::Owned(prefix.to_string())))
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
                cache_lock.prefixes.insert(guild_id, res.0.clone().into_boxed_str());
                Ok(Some(Cow::Owned(res.0)))
            },
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
        cache_lock.prefixes.insert(guild_id, prefix.to_owned().into_boxed_str());

        generate_query_task!(
            r#" INSERT INTO prefixes(guild, prefix) VALUES($1, $2) "#,
            &self.pool,
            (String,),
            guild_id as i64,
            prefix
        )
    }

    pub async fn get_bt_channels(&self) -> Result<ChannelCache, sqlx::Error> {
        let query = "SELECT id FROM bt_channels";

        let mut channels: ChannelCache = HashMap::new();

        let mut rows = sqlx::query_as::<_, (i64,)>(query)
            .fetch(&self.pool);

        while let Some(Ok(row)) = rows.next().await {
            channels.insert(row.0 as u64, None);
        }

        Ok(channels)
    }

    pub async fn delete_bt_channel(&self, id: u64) -> Result<(), sqlx::Error> {
        let query = "DELETE FROM bt_channels WHERE id = $1";

        sqlx::query(query)
            .bind(id as i64)
            .execute(&self.pool)
            .await
            .and_then(|_| Ok(()))
    }
}