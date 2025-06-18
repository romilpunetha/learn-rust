use rusqlite::Result;

use crate::db::{Database, Edge, Node};
use crate::viewer::ViewerContext;
use crate::tao::{NodeData, EdgeData};
use crate::thrift_utils;

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

    pub fn create_node(&self, node_type: &str, data: Option<&[u8]>) -> Result<Node> {
        self.db.create_node(node_type, data)
    }

    pub fn create_node_thrift(&self, node_type: &str, data: Option<&NodeData>) -> Result<Node, Box<dyn std::error::Error>> {
        let bytes = if let Some(d) = data {
            Some(thrift_utils::serialize(d)?)
        } else {
            None
        };
        Ok(self.db.create_node(node_type, bytes.as_deref())?)
    }

    pub fn create_edge(
        &self,
        source: i64,
        target: i64,
        edge_type: &str,
        data: Option<&[u8]>,
    ) -> Result<Edge> {
        self.db.create_edge(source, target, edge_type, data)
    }

    pub fn create_edge_thrift(
        &self,
        source: i64,
        target: i64,
        edge_type: &str,
        data: Option<&EdgeData>,
    ) -> Result<Edge, Box<dyn std::error::Error>> {
        let bytes = if let Some(d) = data {
            Some(thrift_utils::serialize(d)?)
        } else {
            None
        };
        Ok(self.db.create_edge(source, target, edge_type, bytes.as_deref())?)
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

    pub fn decode_node_data(node: &Node) -> thrift::Result<Option<NodeData>> {
        match &node.data {
            Some(bytes) => Ok(Some(thrift_utils::deserialize(bytes)?)),
            None => Ok(None),
        }
    }

    pub fn decode_edge_data(edge: &Edge) -> thrift::Result<Option<EdgeData>> {
        match &edge.data {
            Some(bytes) => Ok(Some(thrift_utils::deserialize(bytes)?)),
            None => Ok(None),
        }
    }

    pub fn get_all_nodes(&self) -> Result<Vec<Node>> {
        self.db.get_all_nodes()
    }
}

