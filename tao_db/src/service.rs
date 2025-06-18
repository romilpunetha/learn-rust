use rusqlite::Result;

use crate::db::{Database, Edge, Node};
use crate::viewer::ViewerContext;

pub struct TaoService {
    db: Database,
}

impl TaoService {
    pub fn new(db: Database) -> Self {
        TaoService { db }
    }

    pub fn init(&self) -> Result<()> {
        self.db.init()
    }

    pub fn create_node(&self, node_type: &str, data: Option<&str>) -> Result<Node> {
        self.db.create_node(node_type, data)
    }

    pub fn create_edge(
        &self,
        source: i64,
        target: i64,
        edge_type: &str,
        data: Option<&str>,
    ) -> Result<Edge> {
        self.db.create_edge(source, target, edge_type, data)
    }

    pub fn get_edges(
        &mut self,
        viewer: &ViewerContext,
        node_id: i64,
    ) -> Result<Vec<Edge>> {
        // In real system viewer permissions would be checked here
        let _uid = viewer.user_id;
        self.db.get_edges_for_node(node_id)
    }
}

