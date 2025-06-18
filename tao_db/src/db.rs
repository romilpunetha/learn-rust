use rusqlite::{params, Connection, Result};

use crate::cache::Cache;

#[derive(Debug, Clone)]
pub struct Node {
    pub id: i64,
    pub node_type: String,
    pub data: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Edge {
    pub id: i64,
    pub source: i64,
    pub target: i64,
    pub edge_type: String,
    pub data: Option<String>,
}

pub struct Database {
    conn: Connection,
    cache: Cache<i64, Vec<Edge>>, // cache edges by source node id
}

impl Database {
    pub fn new(path: &str, cache_capacity: usize) -> Result<Self> {
        let conn = Connection::open(path)?;
        Ok(Database {
            conn,
            cache: Cache::new(cache_capacity),
        })
    }

    pub fn init(&self) -> Result<()> {
        crate::schema::init_db(&self.conn)
    }

    pub fn create_node(&self, node_type: &str, data: Option<&str>) -> Result<Node> {
        self.conn.execute(
            "INSERT INTO nodes (node_type, data) VALUES (?1, ?2)",
            params![node_type, data],
        )?;
        let id = self.conn.last_insert_rowid();
        Ok(Node {
            id,
            node_type: node_type.to_string(),
            data: data.map(|s| s.to_string()),
        })
    }

    pub fn create_edge(
        &self,
        source: i64,
        target: i64,
        edge_type: &str,
        data: Option<&str>,
    ) -> Result<Edge> {
        self.conn.execute(
            "INSERT INTO edges (source, target, edge_type, data) VALUES (?1, ?2, ?3, ?4)",
            params![source, target, edge_type, data],
        )?;
        let id = self.conn.last_insert_rowid();
        Ok(Edge {
            id,
            source,
            target,
            edge_type: edge_type.to_string(),
            data: data.map(|s| s.to_string()),
        })
    }

    pub fn get_edges_for_node(&mut self, source: i64) -> Result<Vec<Edge>> {
        if let Some(edges) = self.cache.get(&source).cloned() {
            return Ok(edges);
        }
        let mut stmt = self.conn.prepare(
            "SELECT id, source, target, edge_type, data FROM edges WHERE source = ?1",
        )?;
        let edges_iter = stmt.query_map(params![source], |row| {
            Ok(Edge {
                id: row.get(0)?,
                source: row.get(1)?,
                target: row.get(2)?,
                edge_type: row.get(3)?,
                data: row.get(4)?,
            })
        })?;
        let edges: Vec<Edge> = edges_iter.map(|r| r.unwrap()).collect();
        self.cache.insert(source, edges.clone());
        Ok(edges)
    }
}

