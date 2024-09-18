mod wfol;

use wfol::expr::*;
use wfol::tree::*;

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
    let tree: Tree = expr.into();

    println!("{tree:#?}");
    println!("{tree}");

    /*tree.remove(8);

    println!("{tree:#?}");
    println!("{tree}");

    to_nnf(&mut tree);*/
}
