use petgraph::prelude::{StableGraph, Dfs};

#[test]
fn dfs_works() -> anyhow::Result<()> {
    use petgraph::visit::Dfs;

    let mut gr = StableGraph::<_, _>::with_capacity(0, 0);
    let a = gr.add_node("a");
    let b = gr.add_node("b");
    let c = gr.add_node("c");
    let d = gr.add_node("d");
    gr.add_edge(a, b, 1);
    gr.add_edge(a, c, 2);
    gr.add_edge(b, c, 3);
    gr.add_edge(b, d, 4);
    gr.add_edge(c, d, 5);
    gr.add_edge(d, b, 6);
    gr.add_edge(c, b, 7);
    println!("{:?}", gr);

    let mut dfs = Dfs::new(&gr, a);
    while let Some(next) = dfs.next(&gr) {
        println!("dfs visit => {:?}, weight={:?}", next, &gr[next]);
    }

    Ok(())
}

#[test]
fn stable_graph() {
    let mut gr = StableGraph::<_, _>::with_capacity(0, 0);

    let a = gr.add_node(0);
    let b = gr.add_node(1);
    let c = gr.add_node(2);
    let _ed = gr.add_edge(a, b, 1);
    println!("{:?}", gr);
    gr.remove_node(b);
    println!("{:?}", gr);
    let d = gr.add_node(3);
    println!("{:?}", gr);
    gr.remove_node(a);
    gr.remove_node(c);
    println!("{:?}", gr);
    gr.add_edge(d, d, 2);
    println!("{:?}", gr);

    let e = gr.add_node(4);
    gr.add_edge(d, e, 3);
    println!("{:?}", gr);
    for neigh in gr.neighbors(d) {
        println!("edge {:?} -> {:?}", d, neigh);
    }
}


