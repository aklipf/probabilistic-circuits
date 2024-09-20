mod io;
mod tree;

use std::time::Instant;

use io::cnf::load;
use tree::expr::*;
use tree::nnf::to_nnf;
use tree::tree::*;

fn main() {
    let expr = not(all(
        "x",
        any(
            "y",
            imply(
                or(not(and(not(var("A")), var("B"))), var("C")),
                predicate("test", &["x", "y"]),
            ),
        ),
    ));
    let mut tree: Tree = expr.into();

    println!("{tree:#?}");
    println!("{tree}");

    to_nnf(&mut tree);

    println!("{tree}");

    let start = Instant::now();
    let tree = load::<u32>("aim-50-1_6-yes1-4.cnf".to_string()).unwrap();
    let duration = start.elapsed();
    println!("{:?}", duration);
    let start = Instant::now();
    let tree = load::<u32>("aim-100-1_6-no-1.cnf".to_string()).unwrap();
    let duration = start.elapsed();
    println!("{:?}", duration);
    println!("{tree}");
}
