use probabilistic_circuits::logic::propositional::*;
use probabilistic_circuits::tree::mapping::Mapping;
use probabilistic_circuits::tree::tree::Tree;
use std::time::Instant;

fn main() {
    //expr!(forall(x): exist(y): WorksFor(x,y) | Boss(x))
    let mut tree = Tree::<PropSymbols>::build(|builder| {
        builder.and(
            |builder| {
                let var_id = builder.add_named(&"A".to_string());
                builder.var(var_id)
            },
            |builder| {
                builder.not(|builder| {
                    let var_id = builder.add_named(&"B".to_string());
                    builder.var(var_id)
                })
            },
        )
    });
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
