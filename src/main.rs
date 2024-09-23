mod io;
mod tree;

use std::array::IntoIter;
use std::time::Instant;

use io::cnf::*;
use tree::cnf::skolemize;
use tree::expr::*;
use tree::nnf::to_nnf;
use tree::tree::*;

fn main() {
    let expr = not(every(
        "x",
        exist(
            "y",
            imply(
                or(not(and(not(var("A")), var("B"))), var("C")),
                predicate("test", &["x", "y"]),
            ),
        ),
    ));
    let mut tree: Tree = expr.into();

    let expr = every(
        "x",
        exist(
            "y",
            or(
                predicate("WorksFor", &["x", "y"]),
                predicate("Boss", &["x"]),
            ),
        ),
    );
    let mut tree: Tree = expr.into();

    //println!("{tree:#?}");
    println!("{tree}");

    let start = Instant::now();
    skolemize(&mut tree);
    let duration = start.elapsed();
    println!("{:?}", duration);

    println!("{tree}");

    /*let start = Instant::now();
    let tree = load_file::<u32>("aim-50-1_6-yes1-4.cnf".to_string()).unwrap();
    let duration = start.elapsed();
    println!("{:?}", duration);
    println!("{tree}");*/
}
