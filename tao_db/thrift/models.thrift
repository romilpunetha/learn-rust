namespace rs tao_db.thrift

// Core object types that represent entities in the social graph
struct User {
  1: required i64 id,
  2: required string username,
  3: required string email,
  4: optional string full_name,
  5: optional string bio,
  6: optional string profile_picture_url,
  7: required i64 created_time,
  8: optional i64 last_active_time,
  9: required bool is_verified,
  10: optional string location
}

struct Post {
  1: required i64 id,
  2: required i64 author_id,
  3: required string content,
  4: optional string media_url,
  5: required i64 created_time,
  6: optional i64 updated_time,
  7: required string post_type, // "text", "photo", "video", "link"
  8: optional string visibility, // "public", "friends", "private"
  9: required i32 like_count,
  10: required i32 comment_count,
  11: required i32 share_count
}

struct Comment {
  1: required i64 id,
  2: required i64 post_id,
  3: required i64 author_id,
  4: required string content,
  5: required i64 created_time,
  6: optional i64 updated_time,
  7: optional i64 parent_comment_id, // for nested comments
  8: required i32 like_count
}

struct Group {
  1: required i64 id,
  2: required string name,
  3: optional string description,
  4: required i64 created_time,
  5: required i64 creator_id,
  6: required string privacy, // "public", "closed", "secret"
  7: required i32 member_count,
  8: optional string cover_photo_url
}

struct Page {
  1: required i64 id,
  2: required string name,
  3: optional string description,
  4: required string category,
  5: required i64 created_time,
  6: optional string website,
  7: required i32 follower_count,
  8: optional string profile_picture_url,
  9: required bool is_verified
}

// Association data structures that represent relationships/edges
struct Friendship {
  1: required i64 created_time,
  2: required string status, // "pending", "accepted", "blocked"
  3: optional string relationship_type, // "friend", "family", "colleague"
  4: optional i64 mutual_friends_count
}

struct Follow {
  1: required i64 created_time,
  2: required bool notifications_enabled,
  3: optional string follow_type // "close_friend", "acquaintance", "default"
}

struct Like {
  1: required i64 created_time,
  2: required string reaction_type, // "like", "love", "laugh", "angry", "sad", "wow"
  3: optional string source // "timeline", "notification", "search"
}

struct Membership {
  1: required i64 joined_time,
  2: required string role, // "member", "admin", "moderator"
  3: required string status, // "active", "pending", "banned"
  4: optional i64 invited_by_user_id
}

struct PostAuthor {
  1: required i64 created_time,
  2: optional string post_source // "timeline", "group", "page"
}

struct CommentAuthor {
  1: required i64 created_time,
  2: optional bool is_edited
}

// Generic structures for extensibility
struct ObjectData {
  1: required string object_type, // "user", "post", "comment", "group", "page"
  2: required binary data // serialized object data
}

struct AssociationData {
  1: required string assoc_type, // "friendship", "follow", "like", "membership", etc.
  2: required binary data, // serialized association data
  3: required i64 created_time,
  4: optional i64 updated_time,
  5: optional i64 time1, // TAO uses time1/time2 for temporal queries
  6: optional i64 time2
}

// Query structures for complex operations
struct AssociationQuery {
  1: required i64 id1, // source object id
  2: optional i64 id2, // target object id (optional for "get all" queries)
  3: required string assoc_type,
  4: optional i64 start_time,
  5: optional i64 end_time,
  6: optional i32 limit,
  7: optional i64 offset
}

struct BatchObjectRequest {
  1: required list<i64> object_ids,
  2: required string object_type
}

struct BatchAssociationRequest {
  1: required list<AssociationQuery> queries
}
