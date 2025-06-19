namespace rs tao_db.models.associations

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