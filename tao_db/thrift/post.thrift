namespace rs tao_db.models.post

// Content entities
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