namespace rs tao_db.models.group

// Group and page entities
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