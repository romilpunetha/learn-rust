namespace rs tao_db.models.user

// Core user entity representing a person in the social graph
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