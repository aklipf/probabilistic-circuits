mod wfol;

use wfol::nnf::*;
use wfol::tree::*;

fn main() {
    let expr = all(
        var("x"),
        any(
            var("y"),
            imply(
                or(not(and(not(var("A")), var("B"))), var("C")),
                predi("test", &["x", "y"]),
            ),
        ),
    );

    println!("{expr}");

    let nnf = to_nnf(&expr.into());
    println!("{nnf}");
}

/*
(¬(¬A∧B)∨C)⇒test(x, y)
¬(¬(¬A∧B)∨C)∨test(x, y)
((¬A∧B)∧¬C)∨test(x, y)
*/
