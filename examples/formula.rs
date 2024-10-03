use probabilistic_circuits::logic::propositional::*;
use probabilistic_circuits::tree::mapping::*;
use probabilistic_circuits::tree::tree::*;
use probabilistic_circuits::{and, not, or, propositional, var};
use std::time::Instant;

fn main() {
    //expr!(forall(x): exist(y): WorksFor(x,y) | Boss(x))
    let tree = propositional!(and!(
        var!(name:"B"),
        or!(not!(var!(name:"A")), var!(name:"C"))
    ));
    println!("{tree}");

    let start = Instant::now();
    // skolemize(&mut tree);
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
