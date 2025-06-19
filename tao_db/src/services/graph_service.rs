use std::sync::Arc;

use crate::database::TaoDatabase;
use crate::dto::{GraphData, GraphNode, GraphEdge};
use crate::error::AppResult;
use crate::models::{AssociationType, AssociationQuery};
use crate::viewer::ViewerContext;

#[derive(Clone)]
pub struct GraphService {
    db: Arc<TaoDatabase>,
    user_service: Arc<crate::services::user_service::UserService>,
    social_service: Arc<crate::services::social_service::SocialService>,
    post_service: Arc<crate::services::post_service::PostService>,
}

impl GraphService {
    pub fn new(
        db: Arc<TaoDatabase>,
        user_service: Arc<crate::services::user_service::UserService>,
        social_service: Arc<crate::services::social_service::SocialService>,
        post_service: Arc<crate::services::post_service::PostService>,
    ) -> Self {
        Self { 
            db, 
            user_service, 
            social_service, 
            post_service 
        }
    }

    pub async fn get_social_graph_data(&self, _viewer: &ViewerContext, max_users: i32) -> AppResult<GraphData> {
        let users = self.db.get_all_users(Some(max_users)).await?;
        let mut nodes = Vec::new();
        let mut edges = Vec::new();

        // Create nodes
        for user in &users {
            nodes.push(GraphNode {
                id: user.id.to_string(),
                name: user.full_name.clone().unwrap_or_else(|| user.username.clone()),
                node_type: "user".to_string(),
                verified: user.is_verified,
            });
        }

        // Get all associations between these users
        let user_ids: Vec<i64> = users.iter().map(|u| u.id).collect();
        
        for user in &users {
            // Get friendships
            let friend_query = AssociationQuery {
                id1: user.id,
                id2: None,
                assoc_type: AssociationType::Friendship.as_str().to_string(),
                start_time: None,
                end_time: None,
                limit: Some(20),
                offset: None,
            };

            if let Ok(friendships) = self.db.get_associations(&friend_query).await {
                for friendship in friendships {
                    if user_ids.contains(&friendship.id2) {
                        edges.push(GraphEdge {
                            source: friendship.id1.to_string(),
                            target: friendship.id2.to_string(),
                            edge_type: "friendship".to_string(),
                            weight: 1.0,
                        });
                    }
                }
            }

            // Get follows
            let follow_query = AssociationQuery {
                id1: user.id,
                id2: None,
                assoc_type: AssociationType::Follow.as_str().to_string(),
                start_time: None,
                end_time: None,
                limit: Some(20),
                offset: None,
            };

            if let Ok(follows) = self.db.get_associations(&follow_query).await {
                for follow in follows {
                    if user_ids.contains(&follow.id2) {
                        edges.push(GraphEdge {
                            source: follow.id1.to_string(),
                            target: follow.id2.to_string(),
                            edge_type: "follow".to_string(),
                            weight: 1.0,
                        });
                    }
                }
            }
        }

        Ok(GraphData { nodes, edges })
    }

    pub async fn seed_sample_data(&self) -> AppResult<()> {
        use crate::dto::{CreateUserRequest, CreateFriendshipRequest, CreateFollowRequest, CreatePostRequest};
        
        // Create sample users using the proper service layer with validation
        let user_requests = vec![
            CreateUserRequest {
                username: "alice".to_string(),
                email: "alice@example.com".to_string(),
                full_name: Some("Alice Johnson".to_string()),
                bio: Some("Software engineer who loves hiking".to_string()),
                location: Some("San Francisco, CA".to_string()),
            },
            CreateUserRequest {
                username: "bob".to_string(),
                email: "bob@example.com".to_string(),
                full_name: Some("Bob Smith".to_string()),
                bio: Some("Data scientist and coffee enthusiast".to_string()),
                location: Some("New York, NY".to_string()),
            },
            CreateUserRequest {
                username: "charlie".to_string(),
                email: "charlie@example.com".to_string(),
                full_name: Some("Charlie Brown".to_string()),
                bio: Some("Designer with a passion for UX".to_string()),
                location: Some("Austin, TX".to_string()),
            },
            CreateUserRequest {
                username: "diana".to_string(),
                email: "diana@example.com".to_string(),
                full_name: Some("Diana Prince".to_string()),
                bio: Some("Product manager and martial artist".to_string()),
                location: Some("Seattle, WA".to_string()),
            },
            CreateUserRequest {
                username: "eve".to_string(),
                email: "eve@example.com".to_string(),
                full_name: Some("Eve Chen".to_string()),
                bio: Some("AI researcher and book lover".to_string()),
                location: Some("Boston, MA".to_string()),
            },
        ];

        // Create users through the service layer (with validation and business logic)
        let mut user_ids = Vec::new();
        for user_req in user_requests {
            let user_response = self.user_service.create_user(user_req).await?;
            user_ids.push(user_response.id);
        }

        // Create friendships through the service layer
        let friendship_requests = vec![
            CreateFriendshipRequest {
                user1_id: user_ids[0],
                user2_id: user_ids[1],
                relationship_type: Some("friend".to_string()),
            },
            CreateFriendshipRequest {
                user1_id: user_ids[1],
                user2_id: user_ids[2],
                relationship_type: Some("friend".to_string()),
            },
            CreateFriendshipRequest {
                user1_id: user_ids[2],
                user2_id: user_ids[3],
                relationship_type: Some("friend".to_string()),
            },
            CreateFriendshipRequest {
                user1_id: user_ids[3],
                user2_id: user_ids[4],
                relationship_type: Some("friend".to_string()),
            },
            CreateFriendshipRequest {
                user1_id: user_ids[0],
                user2_id: user_ids[4],
                relationship_type: Some("family".to_string()),
            },
        ];

        for friendship_req in friendship_requests {
            self.social_service.create_friendship(friendship_req).await?;
        }

        // Create follow relationships through the service layer
        let follow_requests = vec![
            CreateFollowRequest {
                follower_id: user_ids[0],
                followee_id: user_ids[2],
                follow_type: Some("default".to_string()),
            },
            CreateFollowRequest {
                follower_id: user_ids[1],
                followee_id: user_ids[3],
                follow_type: Some("close_friend".to_string()),
            },
            CreateFollowRequest {
                follower_id: user_ids[4],
                followee_id: user_ids[0],
                follow_type: Some("default".to_string()),
            },
        ];

        for follow_req in follow_requests {
            self.social_service.create_follow(follow_req).await?;
        }

        // Create posts through the service layer
        let post_requests = vec![
            CreatePostRequest {
                author_id: user_ids[0],
                content: "Just finished an amazing hike in the mountains!".to_string(),
                post_type: "text".to_string(),
                visibility: Some("public".to_string()),
                media_url: None,
            },
            CreatePostRequest {
                author_id: user_ids[1],
                content: "Working on some interesting machine learning models today.".to_string(),
                post_type: "text".to_string(),
                visibility: Some("public".to_string()),
                media_url: None,
            },
            CreatePostRequest {
                author_id: user_ids[2],
                content: "New design system is looking great! 🎨".to_string(),
                post_type: "text".to_string(),
                visibility: Some("friends".to_string()),
                media_url: None,
            },
        ];

        for post_req in post_requests {
            self.post_service.create_post(post_req).await?;
        }

        Ok(())
    }
}