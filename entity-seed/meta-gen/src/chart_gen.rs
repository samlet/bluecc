use crate::{SrvDeles, StateGraph};
use inflector::Inflector;
use std::collections::HashMap;
use tera::{Tera, Context};
use petgraph::graph::NodeIndex;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ChartState{
    pub src: String,
    pub targets: Vec<ChartStateTarget>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ChartStateTarget {
    pub transition: String,
    pub dest: String,
}

#[tokio::test]
async fn chart_gen_works() -> crate::Result<()> {
    let status_type="ORDER_ITEM_STATUS";

    let mut dele = SrvDeles::new();
    dele.use_default_token().await?;
    let mut stg=StateGraph::new();
    stg.build_status_type(&dele, status_type).await?;
    // stg.draw()?;

    // let order_ids=stg.topo();
    // assert!(!order_ids.is_empty());
    // let start_st= order_ids.get(0).unwrap();
    let start_st=stg.node(NodeIndex::new(0));
    let mut trans_map:HashMap<String, Vec<ChartStateTarget>>=HashMap::new();
    for edge in stg.edges() {
        let src= stg.node(edge.source());
        let target=stg.node(edge.target());
        // "ITEM_CREATED" -> "ITEM_APPROVED": Approve Item
        // println!("{:?} -> {:?}: {}", src.weight, target.weight, edge.weight.to_pascal_case());
        let v=trans_map.entry(src.weight.to_pascal_case())
            .or_insert(Vec::new());
        v.push(ChartStateTarget{
            transition: edge.weight.to_pascal_case(),
            dest: target.weight.to_pascal_case()
        });
    }

    let mut tera = Tera::default();
    tera.add_raw_template("state_chart", include_str!("incls/state_chart.j2"))?;
    let chart_name= status_type.to_pascal_case();
    let mut ctx=Context::new();
    ctx.insert("chart_name", &chart_name);
    ctx.insert("start", &start_st.weight.to_pascal_case());
    ctx.insert("trans_map", &trans_map);
    let result=tera.render("state_chart", &ctx)?;
    println!("{}", result);

    Ok(())
}

