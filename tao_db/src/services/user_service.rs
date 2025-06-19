use std::sync::Arc;
use chrono::Utc;

use crate::database::TaoDatabase;
use crate::dto::{CreateUserRequest, UpdateUserRequest, UserResponse, UserStats};
use crate::error::{AppError, AppResult};
use crate::models::{User, AssociationType};

#[derive(Clone)]
pub struct UserService {
    db: Arc<TaoDatabase>,
}

impl UserService {
    pub fn new(db: Arc<TaoDatabase>) -> Self {
        Self { db }
    }

    pub async fn create_user(&self, req: CreateUserRequest) -> AppResult<UserResponse> {
        // Validate input
        if req.username.is_empty() {
            return Err(AppError::Validation("Username cannot be empty".to_string()));
        }
        if req.email.is_empty() {
            return Err(AppError::Validation("Email cannot be empty".to_string()));
        }

        let now = Utc::now().timestamp();
        let user = User {
            id: 0, // Will be set by database
            username: req.username,
            email: req.email,
            full_name: req.full_name,
            bio: req.bio,
            profile_picture_url: None,
            created_time: now,
            last_active_time: Some(now),
            is_verified: false,
            location: req.location,
        };

        let obj = self.db.create_user(&user).await?;
        let mut created_user = user;
        created_user.id = obj.id;
        
        Ok(UserResponse::from(created_user))
    }

    pub async fn get_user(&self, user_id: i64) -> AppResult<UserResponse> {
        match self.db.get_user(user_id).await? {
            Some(user) => Ok(UserResponse::from(user)),
            None => Err(AppError::NotFound(format!("User with id {} not found", user_id))),
        }
    }

    pub async fn get_all_users(&self, limit: Option<i32>) -> AppResult<Vec<UserResponse>> {
        let users = self.db.get_all_users(limit).await?;
        Ok(users.into_iter().map(UserResponse::from).collect())
    }

    pub async fn update_user(&self, user_id: i64, req: UpdateUserRequest) -> AppResult<UserResponse> {
        // Get existing user
        let mut user = match self.db.get_user(user_id).await? {
            Some(user) => user,
            None => return Err(AppError::NotFound(format!("User with id {} not found", user_id))),
        };

        // Update fields
        if let Some(username) = req.username {
            if username.is_empty() {
                return Err(AppError::Validation("Username cannot be empty".to_string()));
            }
            user.username = username;
        }
        if let Some(email) = req.email {
            if email.is_empty() {
                return Err(AppError::Validation("Email cannot be empty".to_string()));
            }
            user.email = email;
        }
        if let Some(full_name) = req.full_name {
            user.full_name = Some(full_name);
        }
        if let Some(bio) = req.bio {
            user.bio = Some(bio);
        }
        if let Some(location) = req.location {
            user.location = Some(location);
        }

        self.db.update_user(&user).await?;
        Ok(UserResponse::from(user))
    }

    pub async fn delete_user(&self, user_id: i64) -> AppResult<()> {
        // Check if user exists
        if self.db.get_user(user_id).await?.is_none() {
            return Err(AppError::NotFound(format!("User with id {} not found", user_id)));
        }

        self.db.delete_object(user_id).await?;
        Ok(())
    }

    pub async fn get_user_stats(&self, user_id: i64) -> AppResult<UserStats> {
        // Check if user exists
        if self.db.get_user(user_id).await?.is_none() {
            return Err(AppError::NotFound(format!("User with id {} not found", user_id)));
        }

        let friend_count = self.db.get_association_count(user_id, AssociationType::Friendship).await?;
        let following_count = self.db.get_association_count(user_id, AssociationType::Follow).await?;
        let post_count = self.db.get_association_count(user_id, AssociationType::PostAuthor).await?;
        
        // For follower count, we'd need a reverse query (simplified for now)
        let follower_count = 0;

        Ok(UserStats {
            user_id,
            friend_count,
            follower_count,
            following_count,
            post_count,
        })
    }
}