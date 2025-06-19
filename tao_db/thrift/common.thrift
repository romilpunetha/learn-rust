namespace rs tao_db.models.common

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