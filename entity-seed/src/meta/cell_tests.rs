use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::rc::Rc;
struct Graph {
    edges: Vec<(i32, i32)>,
    span_tree_cache: RefCell<Option<Vec<(i32, i32)>>>
}

impl Graph {
    fn minimum_spanning_tree(&self) -> Vec<(i32, i32)> {
        self.span_tree_cache.borrow_mut()
            .get_or_insert_with(|| self.calc_span_tree())
            .clone()
    }

    fn calc_span_tree(&self) -> Vec<(i32, i32)> {
        // Expensive computation goes here
        vec![]
    }
}

/// ref: https://skyao.io/learning-rust/std/cell.html
#[test]
fn shared_map_works() {
    let shared_map: Rc<RefCell<_>> = Rc::new(RefCell::new(HashMap::new()));
    // Create a new block to limit the scope of the dynamic borrow
    {
        let mut map: RefMut<_> = shared_map.borrow_mut();
        map.insert("africa", 92388);
        map.insert("kyoto", 11837);
        map.insert("piccadilly", 11826);
        map.insert("marbles", 38);
    }

    // Note that if we had not let the previous borrow of the cache fall out
    // of scope then the subsequent borrow would cause a dynamic thread panic.
    // This is the major hazard of using `RefCell`.
    let total: i32 = shared_map.borrow().values().sum();
    println!("{}", total);
}

#[test]
fn rc_ref_cell_works() -> anyhow::Result<()> {
    #[derive(Debug)]
    struct Node {
        value: i32,
        next: Box<RefCell<Option<Node>>>,
    }

    #[derive(Debug)]
    struct SharedNode {
        value: i32,

        // 用Rc而不是Box, 是因为Rc支持共享;
        // 用Rc<RefCell>形式是因为SharedNode必须用Rc封装才能够知道长度, 这里是一个嵌套结构;
        // RefCell是局部修改, 即使SharedNode不以mut形式传入, 也可以对next字段进行修改;
        next: Rc<RefCell<Option<SharedNode>>>,
    }

    println!("Mutating node");
    let node_a = Node {
        value: 5,
        next: Box::new(RefCell::new(None)),
    };
    let a = Box::new(RefCell::new(Some(node_a)));
    let b = Node { value: 10, next: a };
    println!("Before mutation b is {:?}", b);

    if let Some(ref mut x) = *b.next.borrow_mut() {
        (*x).value += 10;
    }
    println!("After mutation b is {:?}", b);

    println!("Mutating shared node ...");
    let node_a = SharedNode {
        value: 5,
        next: Rc::new(RefCell::new(None)),
    };
    let a = Rc::new(RefCell::new(Some(node_a)));

    let b = SharedNode {
        value: 10,
        next: Rc::clone(&a),
    };
    let c = SharedNode {
        value: 20,
        next: Rc::clone(&a),
    };
    println!("Before mutation a is {:?}", a);
    println!("Before mutation b is {:?}", b);
    println!("Before mutation c is {:?}", c);

    if let Some(ref mut x) = *a.borrow_mut() {
        (*x).value += 10;
    }

    println!("After mutation a = {:?}", a);
    println!("After mutation b = {:?}", b);
    println!("After mutation c = {:?}", c);

    Ok(())
}


