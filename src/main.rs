mod wfol;

use wfol::tree::*;

fn main() {
    println!("Hello, world!");

    let expr = all(var("b"),any(var("a"),or(not(and(not(var("a")), var("b"))), weight(0.5,var("c")))));

    println!("{expr}");
}
