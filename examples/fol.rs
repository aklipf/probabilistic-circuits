use probabilistic_circuits::logic::first_order::*;
use probabilistic_circuits::tree::traits::Mapping;
use probabilistic_circuits::tree::tree::*;
use probabilistic_circuits::*;

fn main() {
    let tree = first_order!(and!(
        exist!(name:"x", not!(pred!(name:"E",name:"x",name:"x"))),
        exist!(name:"x",exist!(name:"y", imply!(pred!(name:"E",name:"x",name:"y"),pred!(name:"E",name:"y",name:"x"))))
    ));
    println!("{tree}");
}
