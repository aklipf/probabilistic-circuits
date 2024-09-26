mod io;
mod tree;
use std::time::Instant;

use tree::cnf::skolemize;
use tree::expr::*;
use tree::index::Indexing;
use tree::mapping::Mapping;
use tree::nnf::to_nnf;
use tree::node::Symbols;
use tree::tree::*;

fn main() {
    let mut tree = Tree::build(expr!(forall(x): exist(y): WorksFor(x,y) | Boss(x)));
    println!("{tree}");

    let start = Instant::now();
    skolemize(&mut tree);
    let duration = start.elapsed();
    println!("{:?}", duration);
    //println!("{tree:#?}");

    println!("{tree}");

    /*let start = Instant::now();
    let tree = load_file::<u32>("aim-50-1_6-yes1-4.cnf".to_string()).unwrap();
    let duration = start.elapsed();
    println!("{:?}", duration);
    println!("{tree}");*/
}
