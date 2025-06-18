use tao_db::{service::TaoService, db::Database, viewer::ViewerContext};

#[test]
fn test_insert_and_query() {
    let db = Database::new(":memory:", 4).unwrap();
    let mut service = TaoService::new(db);
    service.init().unwrap();

    let a = service.create_node("user", Some("A")).unwrap();
    let b = service.create_node("user", Some("B")).unwrap();
    service.create_edge(a.id, b.id, "friend", None).unwrap();

    let viewer = ViewerContext::new(a.id);
    let edges = service.get_edges(&viewer, a.id).unwrap();
    assert_eq!(edges.len(), 1);
    assert_eq!(edges[0].target, b.id);
}

