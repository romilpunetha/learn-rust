// Thrift-generated models for TAO Database

// Re-export all generated types
pub mod associations;
pub mod common;
pub mod group;
pub mod post;
pub mod user;

// Re-export commonly used types for convenience
pub use associations::{Friendship, Follow, Like, Membership, PostAuthor, CommentAuthor};
pub use common::{ObjectData, AssociationData, AssociationQuery, BatchObjectRequest, BatchAssociationRequest};
pub use group::{Group, Page};
pub use post::{Post, Comment};
pub use user::User;

// Object and Association type enums for type safety
#[derive(Debug, Clone, PartialEq)]
pub enum ObjectType {
    User,
    Post,
    Comment,
    Group,
    Page,
}

impl ObjectType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ObjectType::User => "user",
            ObjectType::Post => "post", 
            ObjectType::Comment => "comment",
            ObjectType::Group => "group",
            ObjectType::Page => "page",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AssociationType {
    Friendship,
    Follow,
    Like,
    Membership,
    PostAuthor,
    CommentAuthor,
}

impl AssociationType {
    pub fn as_str(&self) -> &'static str {
        match self {
            AssociationType::Friendship => "friendship",
            AssociationType::Follow => "follow",
            AssociationType::Like => "like",
            AssociationType::Membership => "membership",
            AssociationType::PostAuthor => "post_author",
            AssociationType::CommentAuthor => "comment_author",
        }
    }
}