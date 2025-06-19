use serde::{Deserialize, Serialize};
use crate::models::{User, Post};

// Request DTOs
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub full_name: Option<String>,
    pub bio: Option<String>,
    pub location: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    pub username: Option<String>,
    pub email: Option<String>,
    pub full_name: Option<String>,
    pub bio: Option<String>,
    pub location: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePostRequest {
    pub author_id: i64,
    pub content: String,
    pub post_type: String,
    pub visibility: Option<String>,
    pub media_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateFriendshipRequest {
    pub user1_id: i64,
    pub user2_id: i64,
    pub relationship_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateFollowRequest {
    pub follower_id: i64,
    pub followee_id: i64,
    pub follow_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateLikeRequest {
    pub user_id: i64,
    pub target_id: i64,
    pub reaction_type: String,
}

// Response DTOs
#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub full_name: Option<String>,
    pub bio: Option<String>,
    pub profile_picture_url: Option<String>,
    pub created_time: i64,
    pub last_active_time: Option<i64>,
    pub is_verified: bool,
    pub location: Option<String>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
            full_name: user.full_name,
            bio: user.bio,
            profile_picture_url: user.profile_picture_url,
            created_time: user.created_time,
            last_active_time: user.last_active_time,
            is_verified: user.is_verified,
            location: user.location,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct PostResponse {
    pub id: i64,
    pub author_id: i64,
    pub content: String,
    pub media_url: Option<String>,
    pub created_time: i64,
    pub updated_time: Option<i64>,
    pub post_type: String,
    pub visibility: Option<String>,
    pub like_count: i32,
    pub comment_count: i32,
    pub share_count: i32,
}

impl From<Post> for PostResponse {
    fn from(post: Post) -> Self {
        Self {
            id: post.id,
            author_id: post.author_id,
            content: post.content,
            media_url: post.media_url,
            created_time: post.created_time,
            updated_time: post.updated_time,
            post_type: post.post_type,
            visibility: post.visibility,
            like_count: post.like_count,
            comment_count: post.comment_count,
            share_count: post.share_count,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct UserStats {
    pub user_id: i64,
    pub friend_count: i64,
    pub follower_count: i64,
    pub following_count: i64,
    pub post_count: i64,
}

#[derive(Debug, Serialize)]
pub struct GraphData {
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
}

#[derive(Debug, Serialize)]
pub struct GraphNode {
    pub id: String,
    pub name: String,
    pub node_type: String,
    pub verified: bool,
}

#[derive(Debug, Serialize)]
pub struct GraphEdge {
    pub source: String,
    pub target: String,
    pub edge_type: String,
    pub weight: f64,
}

// Query parameters
#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    pub limit: Option<i32>,
    pub offset: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct UserQuery {
    pub limit: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct GraphQuery {
    pub max_users: Option<i32>,
    pub viewer_id: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct PostsQuery {
    pub limit: Option<i32>,
    pub viewer_id: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct FriendsQuery {
    pub limit: Option<i32>,
    pub viewer_id: Option<i64>,
}

// Generic API Response wrapper
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: None,
        }
    }

    pub fn success_with_message(data: T, message: String) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: Some(message),
        }
    }
}

impl ApiResponse<()> {
    pub fn success_message(message: String) -> Self {
        Self {
            success: true,
            data: None,
            message: Some(message),
        }
    }
}