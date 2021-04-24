use trees::{tr, Node};
use std::fmt::Display;

fn tree_to_string<T:Display>( node: &Node<T> ) -> String {
    if node.has_no_child() {
        node.data().to_string()
    } else {
        format!("{}( {})", node.data(),
                node.iter().fold(String::new(),
                                 |s, c| format!("{}{} ",
                                                s, tree_to_string(c))))
    }
}

#[cfg(test)]
mod lib_tests {
    use super::*;
    use crate::ServiceMeta;
    use itertools::Itertools;
    use std::borrow::Borrow;

    #[test]
    fn rels_works() -> anyhow::Result<()> {
        let mut meta = ServiceMeta::load()?;
        let ent_name="Person";
        let ent=meta.get_entity_model(ent_name)?;

        let rels = ent.get_relation_entities();
        println!("{:?}", rels);

        let all_rels=meta.entity_reader.get_or_build_relations(ent_name)?;
        if let Some(rs)=all_rels {
            for r in rs {
                println!("{:?}", r);
            }
        }
        Ok(())
    }

    #[test]
    fn entity_type_works() -> anyhow::Result<()> {
        let mut reader=seed::meta::ModelReader::load()?;
        let names=reader.get_all_entity_names();
        let mut index=0;
        for (_i, name) in names.iter().enumerate(){
            let ent=reader.get_entity_model(name.as_str())?;
            let flds=ent.get_field_names();
            if flds.contains(&String::from("parentTypeId"))
                && flds.contains(&String::from("description")){
                println!("{}. + {} ({})", index, name, ent.pks_str());
                index+=1;
            }else if name.ends_with("Type")
                && flds.contains(&String::from("description")){
                println!("{}. - {} ({})", index, name, ent.pks_str());
                index+=1;
            }
        }
        Ok(())
    }

    mod tree_walk {
        use trees::{Tree, TreeWalk, tr, walk::Visit};
        use crate::entity_info::tree_to_string;
        use std::convert::TryFrom;

        #[test]
        fn get() {
            let tree = tr(0) / tr(1) / tr(2) / tr(3);
            let walk = TreeWalk::from(tree);
            assert_eq!(walk.get(), Some(Visit::Begin((tr(0) / tr(1) / tr(2) / tr(3)).root())));
        }

        #[test] fn iter_mut() {
            let mut tree = Tree::<i32>::from_tuple((0, 1, 2, 3));
            tree.iter_mut().for_each(|mut child| *child.data_mut() *= 10);
            assert_eq!(tree.to_string(), "0( 10 20 30 )");
        }

        #[test] fn from_tuple() {
            let tree = Tree::<i32>::from_tuple((0, (1, 2), (3, 4)));
            assert_eq!(tree, tr(0) / (tr(1) / tr(2)) / (tr(3) / tr(4)));
            assert_eq!(tree.to_string(), "0( 1( 2 ) 3( 4 ) )");
        }

        #[test] fn piled_tree_from_tuple() {
            let tuple = ( 0, (1,2,3), (4,5,6) );
            println!("{:?}", tuple);
            let piled = Tree::<i32>::from_tuple( tuple );
            assert_eq!( piled.to_string(), "0( 1( 2 3 ) 4( 5 6 ) )" );
        }

        #[test] fn piled_tree_string_from_tuple() -> anyhow::Result<()>{
            //          |----------------------------------USA----------------------------------|
            //          |                                  |                                    |
            //     Legislature                      ExecutiveJudiciary                      Judiciary
            //    /         \                              |                                    |
            // House      Senate                      WhiteHouse                          SupremeCourt
            //  |            |                             |                                    |
            // Pelosi      Harris                        Biden                               Roberts

            // let tuple = (0, (1, 2, 3), (4, 5, 6));
            // let tuple = ("USA",
            //              ("Legislature", ("House", ("Pelosi"), "Senate", ("Harris")),
            //               "ExecutiveJudiciary", ("WhiteHouse", ("Biden")),
            //               "Judiciary", ("SupremeCourt", ("Roberts"))));
            // let piled = Tree::<i32>::from_tuple(tuple);
            // let tree_string = "   0( 1( 2 3bc) 4( 5 6 ) )  ";
            let tree_string="USA (Legislature (House (Pelosi) Senate \
            (Harris))ExecutiveJudiciary (WhiteHouse (Biden))Judiciary \
            (SupremeCourt (Roberts)))";
            let piled=Tree::try_from(String::from(tree_string))?;
            println!("{}", piled.to_string());
            println!("{:?}", piled.root().locate_first_by_data(&"Legislature".to_string()).unwrap()
                .descendants());
            Ok(())
        }

        #[test]
        fn insert_next_sib_works() -> anyhow::Result<()> {
            use trees::tr;
            let mut tree = tr(0) / tr(1) / tr(2);
            tree.iter_mut().for_each(|mut sub| sub.insert_next_sib(tr(3)));
            assert_eq!(tree.to_string(), "0( 1 3 2 3 )");
            println!("{}", tree);
            Ok(())
        }

        // https://oooutlk.github.io/trees/crud.html
        #[test]
        fn tree_modify_works() -> anyhow::Result<()> {
            let mut tree = Tree::new(9);
            println!("{}", tree);
            let root = tree.root();
            assert!( root.has_no_child() );

            assert_eq!( root.data(), &9 );

            let mut root = tree.root_mut();
            *root.data_mut() = 0;
            tree.push_back( Tree::new(1) );
            tree.push_back( Tree::new(2) );

            let mut iter = tree.iter();
            assert_eq!( iter.next().unwrap().data(), &1 );
            assert_eq!( iter.next().unwrap().data(), &2 );
            assert_eq!( tree.front().unwrap().data(), &1 );
            assert_eq!( tree.back() .unwrap().data(), &2 );

            let mut node_1 = tree.front_mut().unwrap();
            node_1.push_back( Tree::new(3) );
            node_1.push_back( Tree::new(4) );
            node_1.push_back( Tree::new(5) );
            // println!("{}", tree);
            // .............
            // .     0     .
            // .   /   \   .
            // .  1     2  .
            // . /|\       .
            // .3 4 5      .
            // .............
            let _tree_4 = node_1.iter_mut().nth(1).unwrap().detach();
            // .............
            // .     0     .
            // .   /   \   .
            // .  1     2  .
            // . / \       .
            // .3   5      .
            // .............
            // Specially, the first/last child can be removed via pop_front()/pop_back().
            node_1.pop_front();
            // .............
            // .     0     .
            // .   /   \   .
            // .  1     2  .
            // .  |        .
            // .  5        .
            // .............
            println!("{}", tree);

            Ok(())
        }

        #[test]
        fn tree_to_string_works() -> anyhow::Result<()> {
            let tree = tr(0)
                /( tr(1) /tr(2)/tr(3) )
                /( tr(4) /tr(5)/tr(6) );
            assert_eq!( tree_to_string( &tree ), "0( 1( 2 3 ) 4( 5 6 ) )" );

            Ok(())
        }
    }
}