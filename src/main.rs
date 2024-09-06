mod wfol;

use wfol::tree::*;

fn main() {
    let expr = all(
        var("x"),
        any(
            var("y"),
            imply(
                or(not(and(not(var("A")), var("B"))), weight(0.5, var("C"))),
                predi("test", &["x", "y"]),
            ),
        ),
    );

    println!("{expr}");
}
