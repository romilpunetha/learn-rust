use std::fs::File;
use std::io::Write;
use tao_db::db::Database;
use tao_db::service::TaoService;
use tao_db::viewer::ViewerContext;
use serde::Serialize;
use tao_db::service;

#[derive(Serialize)]
struct JsonNode {
    id: i64,
    node_type: String,
    name: Option<String>,
}

#[derive(Serialize)]
struct JsonEdge {
    source: i64,
    target: i64,
    edge_type: String,
    label: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = Database::new("tao.db", 16)?;
    let mut service = TaoService::new(db);
    service.init()?;

    let nodes = service.get_all_nodes()?;
    let mut json_nodes = Vec::new();
    for n in &nodes {
        let data = service::TaoService::decode_node_data(n)?;
        json_nodes.push(JsonNode { id: n.id, node_type: n.node_type.clone(), name: data.and_then(|d| d.name) });
    }

    let mut json_edges = Vec::new();
    for n in &nodes {
        let viewer = ViewerContext::new(n.id);
        let edges = service.get_edges(&viewer, n.id)?;
        for e in edges {
            let data = service::TaoService::decode_edge_data(&e)?;
            json_edges.push(JsonEdge { source: e.source, target: e.target, edge_type: e.edge_type, label: data.and_then(|d| d.label) });
        }
    }

    let graph = serde_json::json!({"nodes": json_nodes, "edges": json_edges});
    let mut f = File::create("frontend/graph.json")?;
    f.write_all(serde_json::to_string_pretty(&graph)?.as_bytes())?;
    Ok(())
}
