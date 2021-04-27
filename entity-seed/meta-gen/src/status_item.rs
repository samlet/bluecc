use crate::DynamicValue;
use crate::params::Object;
use crate::{SrvDeles, SrvResp};

use chrono::{DateTime, Utc};
use serde_json::json;
use petgraph::Graph;
use petgraph::dot::{Dot, Config};
use std::collections::HashMap;
use petgraph::graph::{NodeIndex, Edge, Node};
use itertools::Itertools;

/// $ meta-cli entity StatusItem
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct StatusItem{
    #[serde(flatten)]
    pub id: StatusItemId,
    // fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_type_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct StatusItemId {
    pub status_id: Option<String>,
}

impl Object for StatusItem{
    type Id = StatusItemId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "status_item"
    }
}

/// $ meta-cli entity StatusValidChange
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct StatusValidChange{
    #[serde(flatten)]
    pub id: StatusValidChangeId,
    // fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition_expression: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transition_name: Option<String>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct StatusValidChangeId {
    pub status_id: Option<String>,
    pub status_id_to: Option<String>,
}

impl Object for StatusValidChange{
    type Id = StatusValidChangeId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "status_valid_change"
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GenericValues {
    result: Vec<DynamicValue>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StatusValidChanges {
    result: Vec<StatusValidChange>,
}

pub struct StateGraph{
    graph: Graph::<String, String>,
    id_map: HashMap<String, NodeIndex>,
}

impl StateGraph{
    pub fn wrap(graph: Graph<String, String>) -> Self {
        StateGraph { graph, id_map: Default::default() }
    }
    pub fn new() -> Self{
        StateGraph {
            graph: Graph::<String, String>::new(),
            id_map: Default::default()
        }
    }

    pub async fn add_start_status(&mut self, dele: &SrvDeles, start_st: &str) -> crate::Result<()> {
        #[derive(Serialize, Deserialize, Clone, Debug)]
        #[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
        pub struct StatusTarget {
            pub status_id_to: String,
            pub description: String,
            pub transition_name: String,
        }
        #[derive(Serialize, Deserialize, Clone, Debug)]
        pub struct Details {
            #[serde(rename = "statusValidChangeToDetails", default)]
            rs: Vec<StatusTarget>,
        }

        let values: DynamicValue = serde_json::from_value(json!({
            "statusId":start_st
        }))?;
        let ret: SrvResp<Details> = dele.srv("getStatusValidChangeToDetails", &values).await?;
        // println!("{}", ret.pretty_str()?);
        assert!(ret.is_ok());

        // let mut id_map:HashMap<String, NodeIndex> =HashMap::new();
        // let mut deps = Graph::<String, String>::new();
        // let mut deps=self.graph;
        let left =
            if !self.id_map.contains_key(start_st) {
                let left = self.graph.add_node(start_st.to_string());
                self.id_map.insert(start_st.to_string(), left);
                left
            } else {
                self.id_map.get(start_st).unwrap().to_owned()
            };
        let rs = ret.data.unwrap().rs;
        // add nodes
        for v in rs.iter() {
            if !self.id_map.contains_key(&v.status_id_to) {
                let i = self.graph.add_node(v.status_id_to.to_owned());
                self.id_map.insert(v.status_id_to.to_owned(), *&i);
            }
        }

        // add edges
        for node in rs.iter() {
            let target = self.id_map.get(node.status_id_to.as_str()).unwrap();
            self.graph.add_edge(left, *target, node.transition_name.to_string());
        }

        Ok(())
    }

    pub async fn get_status_items(&self, dele: &SrvDeles, status_type: &str) -> crate::Result<Vec<String>>{
        let values: DynamicValue = serde_json::from_value(json!({
            "statusTypeId":status_type
        }))?;

        #[derive(Serialize, Deserialize, Clone, Debug)]
        pub struct StatusItems {
            #[serde(rename = "statusItems", default)]
            result: Vec<StatusItem>,
        }
        let ret: SrvResp<StatusItems> = dele.srv("getStatusItemsForType", &values).await?;
        // println!("{}", ret.pretty_str()?);
        assert!(ret.is_ok());
        let rs=ret.data.unwrap().result;
        let items=rs.iter()
            .map(|s|s.id.status_id.as_ref().unwrap().to_owned())
            .collect_vec();

        Ok(items)
    }

    pub async fn build_status_type(&mut self, dele: &SrvDeles, status_type: &str) -> crate::Result<bool>{
        let items= self.get_status_items(&dele, status_type).await?;
        debug!("{:?}", items);
        for item in &items {
            self.add_start_status(&dele, item.as_str()).await?;
        }

        if items.is_empty() {Ok(false)} else {Ok(true)}
    }

    pub fn draw(&self) -> crate::Result<()>{
        use tempfile::tempdir;
        use std::fs::File;
        use std::io::{self, Write};
        use std::process::Command;

        // let dot_graph=Dot::with_config(&self.graph, &[Config::EdgeNoLabel]);
        let dot_graph=Dot::with_config(&self.graph, &[Config::EdgeIndexLabel]);
        let dot = format!("{:?}", dot_graph);
        debug!("{}", dot);

        let dir = tempdir()?;
        let file_path = dir.path().join("tmp-graph.dot");
        let mut file = File::create(file_path.as_path())?;
        writeln!(file, "{}", dot)?;

        // execute: $ graph-easy graph-example.dot --from=dot --as_boxart
        let output = Command::new("graph-easy")
            .arg(&*file_path.to_string_lossy())
            .arg("--from=dot")
            .arg("--as_boxart")
            .output()
            .expect("failed to execute process");
        // let out = output.stdout;
        // println!("status: {}", output.status);
        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();

        // By closing the `TempDir` explicitly, we can check that it has
        // been deleted successfully. If we don't close it explicitly,
        // the directory will still be deleted when `dir` goes out
        // of scope, but we won't know whether deleting the directory
        // succeeded.
        drop(file);
        dir.close()?;

        Ok(())
    }

    pub fn edges(&self) -> &[Edge<String, u32>] {
        self.graph.raw_edges()
    }

    pub fn node(&self, index: NodeIndex<u32>) -> &Node<String, u32> {
        &self.graph.raw_nodes()[index.index()]
    }

    pub fn topo(&self) -> Vec<String>{
        let order = petgraph::algo::toposort(&self.graph, None)
            .expect("topo sort fail");
        let order_ids:Vec<String>=order.iter().map(|n:&NodeIndex<u32>|{
            let src= self.node(*n);
            src.weight.to_string()
        }).collect();
        order_ids
    }
}

#[cfg(test)]
mod lib_tests {
    use super::*;
    use itertools::Itertools;
    use inflector::Inflector;

    #[tokio::test]
    async fn perform_find_status_item_works() -> crate::Result<()> {
        let mut dele = SrvDeles::new();
        dele.use_default_token().await?;
        println!("tok {}", dele.access_token);

        let values: DynamicValue = serde_json::from_value(json!({
            "entityName":"StatusItem",
            "maxRows": 10000
        }))?;

        let ret: SrvResp<GenericValues> = dele.srv("findCc", &values).await?;
        println!("{}", ret.pretty_str()?);
        assert!(ret.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn find_status_valid_change_works() -> crate::Result<()> {
        let mut dele = SrvDeles::new();
        dele.use_default_token().await?;
        println!("tok {}", dele.access_token);

        let values: DynamicValue = serde_json::from_value(json!({
            "entityName":"StatusValidChange",
            "maxRows": 10000
        }))?;

        let ret: SrvResp<StatusValidChanges> = dele.srv("findCc", &values).await?;
        println!("{}", ret.pretty_str()?);
        assert!(ret.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn find_status_items_works() -> crate::Result<()> {
        let mut dele = SrvDeles::new();
        dele.use_default_token().await?;
        let status_type="ORDER_ITEM_STATUS";
        let mut stg=StateGraph::new();
        let items= stg.get_status_items(&dele, status_type).await?;
        println!("{:?}", items);

        for item in items {
            stg.add_start_status(&dele, item.as_str()).await?;
        }
        stg.draw()?;
        Ok(())
    }

    #[tokio::test]
    async fn build_status_type_works() -> crate::Result<()> {
        let mut dele = SrvDeles::new();
        dele.use_default_token().await?;
        let status_type="ORDER_ITEM_STATUS";
        let mut stg=StateGraph::new();
        stg.build_status_type(&dele, status_type).await?;
        stg.draw()?;
        Ok(())
    }

    #[test]
    fn petgraph_works() -> anyhow::Result<()> {
        let mut deps = Graph::<String, String>::new();
        let pg = deps.add_node("petgraph".to_string());
        let fb = deps.add_node("fixedbitset".to_string());
        let qc = deps.add_node("quickcheck".to_string());
        let rand = deps.add_node("rand".to_string());
        let libc = deps.add_node("libc".to_string());
        deps.extend_with_edges(&[
            (pg, fb), (pg, qc),
            (qc, rand), (rand, libc), (qc, libc),
        ]);

        let stg=StateGraph::wrap(deps);
        stg.draw()?;
        println!("{:?}", stg.topo());

        Ok(())
    }

    #[tokio::test]
    async fn status_change_works() -> crate::Result<()> {
        let mut dele = SrvDeles::new();
        dele.use_default_token().await?;

        let mut stg=StateGraph::new();
        let start_st="ITEM_CREATED";
        stg.add_start_status(&dele, start_st).await?;
        stg.add_start_status(&dele, "ITEM_APPROVED").await?;
        stg.draw()?;

        for edge in stg.edges() {
            let src= stg.node(edge.source());
            let target=stg.node(edge.target());
            // "ITEM_CREATED" -> "ITEM_APPROVED": Approve Item
            println!("{:?} -> {:?}: {}", src.weight, target.weight, edge.weight.to_pascal_case());
        }

        let order_ids=stg.topo();
        // ["ITEM_CREATED", "ITEM_REJECTED", "ITEM_APPROVED", "ITEM_CANCELLED", "ITEM_COMPLETED"]
        println!("{:?}", order_ids);

        Ok(())
    }
}

