use probabilistic_circuits::logic::first_order::*;
use probabilistic_circuits::tree::traits::Mapping;
use probabilistic_circuits::tree::tree::*;
use probabilistic_circuits::*;
use solver::domain::Integer;
use solver::naive::enumerate;

use std::time::Instant;

fn main() {
    let tree = first_order!(and!(
        exist!(name:"x", not!(pred!(name:"E",name:"x",name:"x"))),
        exist!(name:"x",exist!(name:"y", imply!(pred!(name:"E",name:"x",name:"y"),pred!(name:"E",name:"y",name:"x"))))
    ));
    println!("{tree}");

    let grounded = tree.ground(&vec![
        Integer::new(&tree, &"x".to_string(), 3),
        Integer::new(&tree, &"y".to_string(), 3),
    ]);

    println!("{grounded}");

    let start = Instant::now();
    for var_id in 0..grounded.num_named() {
        print!("|{: ^5}", grounded.get_named(var_id as u32).unwrap())
    }
    println!("|");
    for _ in 0..grounded.num_named() {
        print!("|=====")
    }
    println!("|");

    for x in enumerate(&grounded) {
        for &v in x.iter() {
            print!("|{: ^5}", if v { 1 } else { 0 })
        }
        println!("|");
    }
    let duration = start.elapsed();
    println!("done in {:?}", duration);
}
