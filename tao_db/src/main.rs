use tao_db::service::TaoService;
use tao_db::db::Database;
use tao_db::viewer::ViewerContext;

fn main() -> rusqlite::Result<()> {
    let db = Database::new("tao.db", 16)?;
    let mut service = TaoService::new(db);
    service.init()?;

    let alice = service.create_node("user", Some("Alice"))?;
    let bob = service.create_node("user", Some("Bob"))?;
    service.create_edge(alice.id, bob.id, "friend", None)?;

    let viewer = ViewerContext::new(alice.id);
    let edges = service.get_edges(&viewer, alice.id)?;
    println!("Edges for node {}: {:?}", alice.id, edges);

    Ok(())
}

