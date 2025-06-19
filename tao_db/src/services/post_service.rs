use std::sync::Arc;
use chrono::Utc;

use crate::database::TaoDatabase;
use crate::dto::{CreatePostRequest, PostResponse};
use crate::error::{AppError, AppResult};
use crate::models::{Post, PostAuthor, AssociationType};
use crate::thrift_utils::thrift_serialize;

#[derive(Clone)]
pub struct PostService {
    db: Arc<TaoDatabase>,
}

impl PostService {
    pub fn new(db: Arc<TaoDatabase>) -> Self {
        Self { db }
    }

    pub async fn create_post(&self, req: CreatePostRequest) -> AppResult<PostResponse> {
        // Validate input
        if req.content.is_empty() {
            return Err(AppError::Validation("Post content cannot be empty".to_string()));
        }

        // Check if author exists
        if self.db.get_user(req.author_id).await?.is_none() {
            return Err(AppError::NotFound(format!("Author with id {} not found", req.author_id)));
        }

        let now = Utc::now().timestamp();
        let post = Post {
            id: 0, // Will be set by database
            author_id: req.author_id,
            content: req.content,
            media_url: req.media_url,
            created_time: now,
            updated_time: None,
            post_type: req.post_type,
            visibility: req.visibility,
            like_count: 0,
            comment_count: 0,
            share_count: 0,
        };

        let obj = self.db.create_post(&post).await?;
        let mut created_post = post;
        created_post.id = obj.id;

        // Create post-author association
        let post_author = PostAuthor {
            created_time: now,
            post_source: Some("api".to_string()),
        };
        let assoc_data = thrift_serialize(&post_author)?;
        self.db.create_association(
            req.author_id,
            created_post.id,
            AssociationType::PostAuthor,
            Some(&assoc_data),
            Some(now),
            None,
        ).await?;

        Ok(PostResponse::from(created_post))
    }

    pub async fn get_post(&self, post_id: i64) -> AppResult<PostResponse> {
        match self.db.get_post(post_id).await? {
            Some(post) => Ok(PostResponse::from(post)),
            None => Err(AppError::NotFound(format!("Post with id {} not found", post_id))),
        }
    }

    pub async fn delete_post(&self, post_id: i64) -> AppResult<()> {
        // Check if post exists
        if self.db.get_post(post_id).await?.is_none() {
            return Err(AppError::NotFound(format!("Post with id {} not found", post_id)));
        }

        self.db.delete_object(post_id).await?;
        Ok(())
    }
}