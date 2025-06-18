use tao_db::service::TaoService;
use tao_db::db::Database;
use tao_db::viewer::ViewerContext;
use tao_db::tao::{NodeData, EdgeData};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = Database::new("tao.db", 16)?;
    let mut service = TaoService::new(db);
    service.init()?;

    let alice_data = NodeData::new(Some("Alice".into()));
    let bob_data = NodeData::new(Some("Bob".into()));
    let alice = service.create_node_thrift("user", Some(&alice_data))?;
    let bob = service.create_node_thrift("user", Some(&bob_data))?;

    let edge_data = EdgeData::new(Some("friend".into()));
    service.create_edge_thrift(alice.id, bob.id, "friend", Some(&edge_data))?;

    let viewer = ViewerContext::new(alice.id);
    let edges = service.get_edges(&viewer, alice.id)?;
    for edge in &edges {
        let data = TaoService::decode_edge_data(edge)?;
        println!("Edge {} -> {}: {:?}", edge.source, edge.target, data);
    }

    Ok(())
}

