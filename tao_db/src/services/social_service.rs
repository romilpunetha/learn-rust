use std::sync::Arc;
use chrono::Utc;

use crate::database::TaoDatabase;
use crate::dto::{CreateFriendshipRequest, CreateFollowRequest, CreateLikeRequest, UserResponse, PostResponse};
use crate::error::{AppError, AppResult};
use crate::models::{Friendship, Follow, Like, AssociationType, AssociationQuery};
use crate::thrift_utils::thrift_serialize;
use crate::viewer::ViewerContext;

#[derive(Clone)]
pub struct SocialService {
    db: Arc<TaoDatabase>,
}

impl SocialService {
    pub fn new(db: Arc<TaoDatabase>) -> Self {
        Self { db }
    }

    pub async fn create_friendship(&self, req: CreateFriendshipRequest) -> AppResult<()> {
        // Validate users exist
        if self.db.get_user(req.user1_id).await?.is_none() {
            return Err(AppError::NotFound(format!("User with id {} not found", req.user1_id)));
        }
        if self.db.get_user(req.user2_id).await?.is_none() {
            return Err(AppError::NotFound(format!("User with id {} not found", req.user2_id)));
        }

        // Check if friendship already exists
        if self.are_friends(req.user1_id, req.user2_id).await? {
            return Err(AppError::BadRequest("Friendship already exists".to_string()));
        }

        let now = Utc::now().timestamp();
        let friendship = Friendship {
            created_time: now,
            status: "accepted".to_string(),
            relationship_type: req.relationship_type,
            mutual_friends_count: None,
        };

        let data = thrift_serialize(&friendship)?;
        
        // Create bidirectional friendship
        self.db.create_association(
            req.user1_id,
            req.user2_id,
            AssociationType::Friendship,
            Some(&data),
            Some(now),
            None,
        ).await?;
        
        self.db.create_association(
            req.user2_id,
            req.user1_id,
            AssociationType::Friendship,
            Some(&data),
            Some(now),
            None,
        ).await?;

        Ok(())
    }

    pub async fn create_follow(&self, req: CreateFollowRequest) -> AppResult<()> {
        // Validate users exist
        if self.db.get_user(req.follower_id).await?.is_none() {
            return Err(AppError::NotFound(format!("User with id {} not found", req.follower_id)));
        }
        if self.db.get_user(req.followee_id).await?.is_none() {
            return Err(AppError::NotFound(format!("User with id {} not found", req.followee_id)));
        }

        // Check if follow already exists
        if self.is_following(req.follower_id, req.followee_id).await? {
            return Err(AppError::BadRequest("Follow relationship already exists".to_string()));
        }

        let now = Utc::now().timestamp();
        let follow = Follow {
            created_time: now,
            notifications_enabled: true,
            follow_type: req.follow_type,
        };

        let data = thrift_serialize(&follow)?;
        self.db.create_association(
            req.follower_id,
            req.followee_id,
            AssociationType::Follow,
            Some(&data),
            Some(now),
            None,
        ).await?;

        Ok(())
    }

    pub async fn create_like(&self, req: CreateLikeRequest) -> AppResult<()> {
        // Validate user exists
        if self.db.get_user(req.user_id).await?.is_none() {
            return Err(AppError::NotFound(format!("User with id {} not found", req.user_id)));
        }

        // Validate target exists (could be post, comment, etc.)
        // For simplicity, we'll assume it's a post
        if self.db.get_post(req.target_id).await?.is_none() {
            return Err(AppError::NotFound(format!("Target with id {} not found", req.target_id)));
        }

        let now = Utc::now().timestamp();
        let like = Like {
            created_time: now,
            reaction_type: req.reaction_type,
            source: Some("api".to_string()),
        };

        let data = thrift_serialize(&like)?;
        self.db.create_association(
            req.user_id,
            req.target_id,
            AssociationType::Like,
            Some(&data),
            Some(now),
            None,
        ).await?;

        Ok(())
    }

    pub async fn get_friends(&self, viewer: &ViewerContext, user_id: i64, limit: Option<i32>) -> AppResult<Vec<UserResponse>> {
        // Check if viewer has permission to see this user's friends
        if !self.can_view_friends(viewer, user_id).await? {
            return Err(AppError::BadRequest("Not authorized to view friends".to_string()));
        }

        let query = AssociationQuery {
            id1: user_id,
            id2: None,
            assoc_type: AssociationType::Friendship.as_str().to_string(),
            start_time: None,
            end_time: None,
            limit,
            offset: None,
        };

        let associations = self.db.get_associations(&query).await?;
        let mut friends = Vec::new();

        for assoc in associations {
            if let Some(user) = self.db.get_user(assoc.id2).await? {
                friends.push(UserResponse::from(user));
            }
        }

        Ok(friends)
    }

    pub async fn get_posts_by_user(&self, viewer: &ViewerContext, user_id: i64, limit: Option<i32>) -> AppResult<Vec<PostResponse>> {
        // Check if viewer has permission to see this user's posts
        if !self.can_view_posts(viewer, user_id).await? {
            return Err(AppError::BadRequest("Not authorized to view posts".to_string()));
        }

        let query = AssociationQuery {
            id1: user_id,
            id2: None,
            assoc_type: AssociationType::PostAuthor.as_str().to_string(),
            start_time: None,
            end_time: None,
            limit,
            offset: None,
        };

        let associations = self.db.get_associations(&query).await?;
        let mut posts = Vec::new();

        for assoc in associations {
            if let Some(post) = self.db.get_post(assoc.id2).await? {
                posts.push(PostResponse::from(post));
            }
        }

        Ok(posts)
    }

    // Helper methods
    async fn can_view_friends(&self, viewer: &ViewerContext, user_id: i64) -> AppResult<bool> {
        Ok(viewer.user_id == user_id || self.are_friends(viewer.user_id, user_id).await?)
    }

    async fn can_view_posts(&self, viewer: &ViewerContext, user_id: i64) -> AppResult<bool> {
        Ok(viewer.user_id == user_id || self.are_friends(viewer.user_id, user_id).await?)
    }

    async fn are_friends(&self, user1_id: i64, user2_id: i64) -> AppResult<bool> {
        let query = AssociationQuery {
            id1: user1_id,
            id2: Some(user2_id),
            assoc_type: AssociationType::Friendship.as_str().to_string(),
            start_time: None,
            end_time: None,
            limit: Some(1),
            offset: None,
        };

        let associations = self.db.get_associations(&query).await?;
        Ok(!associations.is_empty())
    }

    async fn is_following(&self, follower_id: i64, followee_id: i64) -> AppResult<bool> {
        let query = AssociationQuery {
            id1: follower_id,
            id2: Some(followee_id),
            assoc_type: AssociationType::Follow.as_str().to_string(),
            start_time: None,
            end_time: None,
            limit: Some(1),
            offset: None,
        };

        let associations = self.db.get_associations(&query).await?;
        Ok(!associations.is_empty())
    }
}