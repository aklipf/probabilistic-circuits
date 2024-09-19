mod wfol;

use wfol::expr::*;
use wfol::nnf::to_nnf;
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
    let mut tree: Tree = expr.into();

    println!("{tree:#?}");
    println!("{tree}");

    to_nnf(&mut tree);

    println!("{tree}");
}
