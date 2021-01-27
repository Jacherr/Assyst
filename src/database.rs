use std::borrow::Cow;

use sqlx::postgres::{PgPool, PgPoolOptions};

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

#[derive(Clone)]
pub struct Database {
    pool: PgPool,
}
impl Database {
    pub async fn new(max_connections: u32, url: String) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(max_connections)
            .connect(&url)
            .await?;

        Ok(Database { pool })
    }

    pub async fn get_or_set_prefix_for<'a, 'b>(
        &'a self,
        guild_id: u64,
        set_prefix: &'b str,
    ) -> Result<Option<Cow<'b, str>>, sqlx::error::Error> {
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
            Ok(res) => Ok(Some(Cow::Owned(res.0))),
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
        generate_query_task!(
            r#" INSERT INTO prefixes(guild, prefix) VALUES($1, $2) "#,
            &self.pool,
            (String,),
            guild_id as i64,
            prefix
        )
    }
}
