use probabilistic_circuits::logic::circuit::*;
use probabilistic_circuits::tree::traits::Mapping;
use probabilistic_circuits::tree::tree::*;
use probabilistic_circuits::*;

fn main() {
    let tree = circuit!(prod!(
        var!(name:"B"),
        sum!(not!(var!(name:"A")), var!(name:"C"))
    ));
    println!("{tree}");
}
