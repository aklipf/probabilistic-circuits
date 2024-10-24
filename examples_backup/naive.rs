use probabilistic_circuits::logic::propositional::*;
use probabilistic_circuits::solver::naive::enumerate;
use probabilistic_circuits::tree::traits::Mapping;
use probabilistic_circuits::tree::tree::{ExpressionTree, Tree};
use probabilistic_circuits::*;
use std::time::Instant;

fn main() {
    let tree = propositional!(conjunction!(
        var!(name:"B"),
        var!(name:"D"),
        var!(name:"E"),
        var!(name:"F"),
        var!(name:"G"),
        var!(name:"H"),
        var!(name:"I"),
        var!(name:"K"),
        var!(name:"J"),
        var!(name:"K"),
        var!(name:"L"),
        var!(name:"M"),
        var!(name:"N"),
        var!(name:"O"),
        var!(name:"P"),
        var!(name:"Q"),
        var!(name:"R"),
        var!(name:"S"),
        var!(name:"T"),
        var!(name:"U"),
        var!(name:"V"),
        var!(name:"W"),
        var!(name:"X"),
        var!(name:"Y"),
        var!(name:"Z"),
        or!(not!(var!(name:"A")), var!(name:"C"))
    ));
    println!("{tree}");

    let start = Instant::now();
    for var_id in 0..tree.num_named() {
        print!("|{: ^5}", tree.get_named(var_id as u32).unwrap())
    }
    println!("|");
    for _ in 0..tree.num_named() {
        print!("|=====")
    }
    println!("|");

    for x in enumerate(&tree) {
        for &v in x.iter() {
            print!("|{: ^5}", if v { 1 } else { 0 })
        }
        println!("|");
    }
    let duration = start.elapsed();
    println!("done in {:?}", duration);
}
