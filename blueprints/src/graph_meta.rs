#[cfg(test)]
mod lib_tests {
    use super::*;
    use petgraph::{EdgeType, Graph, Directed};
    use petgraph::Direction::{Outgoing, Incoming};
    use petgraph::prelude::*;

    fn make_edge_iterator_graph<Ty: EdgeType>() -> Graph<String, f64, Ty> {
        let mut gr = Graph::default();
        let a = gr.add_node("a".to_string());
        let b = gr.add_node("b".to_string());
        let c = gr.add_node("c".to_string());
        let d = gr.add_node("d".to_string());
        let e = gr.add_node("e".to_string());
        let f = gr.add_node("f".to_string());
        let g = gr.add_node("g".to_string());
        gr.add_edge(a, b, 7.0);
        gr.add_edge(a, d, 5.);
        gr.add_edge(d, b, 9.);
        gr.add_edge(b, c, 8.);
        gr.add_edge(b, e, 7.);
        gr.add_edge(c, c, 8.);
        gr.add_edge(c, e, 5.);
        gr.add_edge(d, e, 15.);
        gr.add_edge(d, f, 6.);
        gr.add_edge(f, e, 8.);
        gr.add_edge(f, g, 11.);
        gr.add_edge(e, g, 9.);

        gr
    }

    #[test]
    fn iter_outgoing_works() -> anyhow::Result<()> {
        let mut deps = Graph::<&str, &str>::new();
        let pg = deps.add_node("petgraph");
        let fb = deps.add_node("fixedbitset");
        let qc = deps.add_node("quickcheck");
        let rand = deps.add_node("rand");
        let libc = deps.add_node("libc");
        deps.extend_with_edges(&[
            (pg, fb, "1"), (pg, qc, "2"),
            (qc, rand, "3"), (rand, libc, "4"), (qc, libc, "5"),
        ]);

        let g_str=serde_json::to_string(&deps)?;
        println!("{}", g_str);

        // let mut dfs = Dfs::new(&deps, pg);
        // while let Some(node) = dfs.next(&deps) {
        for node in deps.node_indices() {
            let weight=deps.node_weight(node);
            if Some(&"quickcheck")==weight{
                let mut edges = deps.neighbors_directed(node, Incoming).detach();
                while let Some(edge) = edges.next_edge(&deps) {
                    let edge_payload=deps.edge_weight(edge);
                    println!("income .. {:?}", edge_payload);
                }

                let mut edges = deps.neighbors_directed(node, Outgoing).detach();
                while let Some(edge) = edges.next_edge(&deps) {
                    let edge_payload=deps.edge_weight(edge);
                    let (source, target) = deps.edge_endpoints(edge).unwrap();
                    assert_eq!(source, node);
                    println!("outgoing .. {:?} -> {:?}", edge_payload,
                             deps.node_weight(target));
                }
            }
        }

        type DiGraphStrI32 = DiGraph<String, String>;
        let gr:DiGraphStrI32 = serde_json::from_str(g_str.as_str())?;
        println!("{:?}", gr);

        Ok(())
    }
}
