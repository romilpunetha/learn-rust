use anyhow::Result;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqlitePool, Row, FromRow};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::cache::Cache;
use crate::models::{User, Post, AssociationQuery, ObjectType, AssociationType};
use crate::thrift_utils::{thrift_serialize, thrift_deserialize};

// Object structure for SQLx operations
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TaoObject {
    pub id: i64,
    pub object_type: String,
    pub data: Vec<u8>,
    pub created_time: i64,
    pub updated_time: i64,
}

// Association structure
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TaoAssociation {
    pub id: i64,
    pub id1: i64,
    pub id2: i64,
    pub assoc_type: String,
    pub data: Option<Vec<u8>>,
    pub created_time: i64,
    pub updated_time: i64,
    pub time1: Option<i64>,
    pub time2: Option<i64>,
}

// Modern async TAO Database with SQLx connection pool
pub struct TaoDatabase {
    pool: SqlitePool,
    object_cache: Arc<Mutex<Cache<i64, TaoObject>>>,
    assoc_cache: Arc<Mutex<Cache<String, Vec<TaoAssociation>>>>,
    count_cache: Arc<Mutex<Cache<String, i64>>>,
}

impl TaoDatabase {
    pub async fn new(database_url: &str, cache_capacity: usize) -> Result<Self> {
        // Create connection pool with proper configuration
        let pool = SqlitePool::connect(database_url).await?;

        Ok(TaoDatabase {
            pool,
            object_cache: Arc::new(Mutex::new(Cache::new(cache_capacity))),
            assoc_cache: Arc::new(Mutex::new(Cache::new(cache_capacity * 2))),
            count_cache: Arc::new(Mutex::new(Cache::new(cache_capacity / 2))),
        })
    }

    pub async fn init(&self) -> Result<()> {
        // Create objects table
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS objects (
                id INTEGER PRIMARY KEY,
                object_type TEXT NOT NULL,
                data BLOB NOT NULL,
                created_time INTEGER NOT NULL,
                updated_time INTEGER NOT NULL
            )"
        )
        .execute(&self.pool)
        .await?;

        // Create associations table
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS associations (
                id INTEGER PRIMARY KEY,
                id1 INTEGER NOT NULL,
                id2 INTEGER NOT NULL,
                assoc_type TEXT NOT NULL,
                data BLOB,
                created_time INTEGER NOT NULL,
                updated_time INTEGER NOT NULL,
                time1 INTEGER,
                time2 INTEGER,
                UNIQUE(id1, id2, assoc_type)
            )"
        )
        .execute(&self.pool)
        .await?;

        // Create indexes for performance
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_objects_type ON objects(object_type)")
            .execute(&self.pool)
            .await?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_assoc_id1_type ON associations(id1, assoc_type)")
            .execute(&self.pool)
            .await?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_assoc_id2_type ON associations(id2, assoc_type)")
            .execute(&self.pool)
            .await?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_assoc_time1 ON associations(time1)")
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn create_object(&self, object_type: ObjectType, data: &[u8]) -> Result<TaoObject> {
        let now = Utc::now().timestamp();
        let type_str = object_type.as_str();

        let result = sqlx::query(
            "INSERT INTO objects (object_type, data, created_time, updated_time) VALUES (?, ?, ?, ?)"
        )
        .bind(type_str)
        .bind(data)
        .bind(now)
        .bind(now)
        .execute(&self.pool)
        .await?;

        let id = result.last_insert_rowid();


        let obj = TaoObject {
            id,
            object_type: type_str.to_string(),
            data: data.to_vec(),
            created_time: now,
            updated_time: now,
        };

        // Cache the new object
        self.object_cache.lock().await.insert(id, obj.clone());

        Ok(obj)
    }

    pub async fn get_object(&self, id: i64) -> Result<Option<TaoObject>> {
        // Check cache first
        {
            let mut cache = self.object_cache.lock().await;
            if let Some(obj) = cache.get(&id).cloned() {
                return Ok(Some(obj));
            }
        }

        // Query database using SQLx
        let obj = sqlx::query_as::<_, TaoObject>(
            "SELECT id, object_type, data, created_time, updated_time FROM objects WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(ref object) = obj {
            self.object_cache.lock().await.insert(id, object.clone());
        }

        Ok(obj)
    }

    pub async fn update_object(&self, id: i64, data: &[u8]) -> Result<()> {
        let now = Utc::now().timestamp();

        sqlx::query("UPDATE objects SET data = ?, updated_time = ? WHERE id = ?")
            .bind(data)
            .bind(now)
            .bind(id)
            .execute(&self.pool)
            .await?;

        // Invalidate cache
        self.object_cache.lock().await.remove(&id);

        Ok(())
    }

    pub async fn delete_object(&self, id: i64) -> Result<()> {
        // Delete all associations involving this object
        sqlx::query("DELETE FROM associations WHERE id1 = ? OR id2 = ?")
            .bind(id)
            .bind(id)
            .execute(&self.pool)
            .await?;

        // Delete the object
        sqlx::query("DELETE FROM objects WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        // Invalidate caches
        self.object_cache.lock().await.remove(&id);
        self.assoc_cache.lock().await.clear();
        self.count_cache.lock().await.clear();

        Ok(())
    }

    pub async fn create_association(
        &self,
        id1: i64,
        id2: i64,
        assoc_type: AssociationType,
        data: Option<&[u8]>,
        time1: Option<i64>,
        time2: Option<i64>,
    ) -> Result<TaoAssociation> {
        let now = Utc::now().timestamp();
        let type_str = assoc_type.as_str();

        let result = sqlx::query(
            "INSERT OR REPLACE INTO associations (id1, id2, assoc_type, data, created_time, updated_time, time1, time2)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(id1)
        .bind(id2)
        .bind(type_str)
        .bind(data)
        .bind(now)
        .bind(now)
        .bind(time1)
        .bind(time2)
        .execute(&self.pool)
        .await?;

        let id = result.last_insert_rowid();

        let assoc = TaoAssociation {
            id,
            id1,
            id2,
            assoc_type: type_str.to_string(),
            data: data.map(|d| d.to_vec()),
            created_time: now,
            updated_time: now,
            time1,
            time2,
        };

        // Invalidate relevant caches
        let cache_key = format!("{}:{}", id1, type_str);
        self.assoc_cache.lock().await.remove(&cache_key);
        let count_key = format!("count:{}:{}", id1, type_str);
        self.count_cache.lock().await.remove(&count_key);

        Ok(assoc)
    }

    pub async fn get_associations(&self, query: &AssociationQuery) -> Result<Vec<TaoAssociation>> {
        let cache_key = format!("{}:{}", query.id1, query.assoc_type);

        // Check cache first for simple queries
        if query.id2.is_none() && query.start_time.is_none() && query.end_time.is_none() {
            let mut cache = self.assoc_cache.lock().await;
            if let Some(assocs) = cache.get(&cache_key).cloned() {
                return Ok(self.apply_limit_offset(assocs, query.limit, query.offset));
            }
        }

        // Query database using SQLx
        let assocs = sqlx::query_as::<_, TaoAssociation>(
            "SELECT id, id1, id2, assoc_type, data, created_time, updated_time, time1, time2
             FROM associations WHERE id1 = ? AND assoc_type = ? ORDER BY created_time DESC"
        )
        .bind(query.id1)
        .bind(&query.assoc_type)
        .fetch_all(&self.pool)
        .await?;

        // Cache simple queries
        if query.id2.is_none() && query.start_time.is_none() && query.end_time.is_none()
           && query.limit.is_none() && query.offset.is_none() {
            self.assoc_cache.lock().await.insert(cache_key, assocs.clone());
        }

        Ok(assocs)
    }

    pub async fn delete_association(&self, id1: i64, id2: i64, assoc_type: AssociationType) -> Result<()> {
        let type_str = assoc_type.as_str();

        sqlx::query("DELETE FROM associations WHERE id1 = ? AND id2 = ? AND assoc_type = ?")
            .bind(id1)
            .bind(id2)
            .bind(type_str)
            .execute(&self.pool)
            .await?;

        // Invalidate caches
        let cache_key = format!("{}:{}", id1, type_str);
        self.assoc_cache.lock().await.remove(&cache_key);
        let count_key = format!("count:{}:{}", id1, type_str);
        self.count_cache.lock().await.remove(&count_key);

        Ok(())
    }

    pub async fn get_association_count(&self, id1: i64, assoc_type: AssociationType) -> Result<i64> {
        let type_str = assoc_type.as_str();
        let cache_key = format!("count:{}:{}", id1, type_str);

        // Check cache first
        {
            let mut cache = self.count_cache.lock().await;
            if let Some(count) = cache.get(&cache_key).cloned() {
                return Ok(count);
            }
        }

        let row = sqlx::query("SELECT COUNT(*) FROM associations WHERE id1 = ? AND assoc_type = ?")
            .bind(id1)
            .bind(type_str)
            .fetch_one(&self.pool)
            .await?;

        let count: i64 = row.get(0);

        self.count_cache.lock().await.insert(cache_key, count);
        Ok(count)
    }

    pub async fn get_all_users(&self, limit: Option<i32>) -> Result<Vec<User>> {
        let limit = limit.unwrap_or(100);

        let user_ids: Vec<i64> = sqlx::query("SELECT id FROM objects WHERE object_type = 'user' LIMIT ?")
            .bind(limit)
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|row| row.get::<i64, _>(0))
            .collect();

        let mut users = Vec::new();
        for user_id in user_ids {
            if let Some(obj) = self.get_object(user_id).await? {
                if obj.object_type == "user" {
                    let mut user: User = thrift_deserialize(&obj.data)?;
                    user.id = obj.id; // Set the correct database ID
                    users.push(user);
                }
            }
        }

        Ok(users)
    }

    // Utility methods
    fn apply_limit_offset(&self, mut items: Vec<TaoAssociation>, limit: Option<i32>, offset: Option<i64>) -> Vec<TaoAssociation> {
        if let Some(offset) = offset {
            if offset > 0 {
                let offset_usize = offset as usize;
                if offset_usize < items.len() {
                    items = items.into_iter().skip(offset_usize).collect();
                } else {
                    return Vec::new();
                }
            }
        }

        if let Some(limit) = limit {
            if limit > 0 {
                items.truncate(limit as usize);
            }
        }

        items
    }

    // Convenience methods for typed object creation with Thrift serialization
    pub async fn create_user(&self, user: &User) -> Result<TaoObject> {
        let data = thrift_serialize(user)?;
        self.create_object(ObjectType::User, &data).await
    }

    pub async fn create_post(&self, post: &Post) -> Result<TaoObject> {
        let data = thrift_serialize(post)?;
        self.create_object(ObjectType::Post, &data).await
    }

    pub async fn get_user(&self, id: i64) -> Result<Option<User>> {
        if let Some(obj) = self.get_object(id).await? {
            if obj.object_type == "user" {
                let mut user: User = thrift_deserialize(&obj.data)?;
                user.id = obj.id; // Set the correct database ID
                return Ok(Some(user));
            }
        }
        Ok(None)
    }

    pub async fn get_post(&self, id: i64) -> Result<Option<Post>> {
        if let Some(obj) = self.get_object(id).await? {
            if obj.object_type == "post" {
                let mut post: Post = thrift_deserialize(&obj.data)?;
                post.id = obj.id; // Set the correct database ID
                return Ok(Some(post));
            }
        }
        Ok(None)
    }

    pub async fn update_user(&self, user: &User) -> Result<()> {
        let data = thrift_serialize(user)?;
        self.update_object(user.id, &data).await
    }
}