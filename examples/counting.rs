use std::time::Instant;

use probabilistic_circuits::{
    and, conjunction, disjunction, equiv, every, exist, first_order, imply,
    logic::{
        circuit::propositional_to_circuit,
        first_order::{first_order_to_propositional, FOMut, FirstOrderLogic},
        propositional::count_propositional,
        Eval,
    },
    not, or, pred,
    solver::{domain::Integer, naive::enumerate},
    tree::{Mapping, Tree},
};

fn main() {
    let input = first_order!(conjunction!(
        every!("x", not!(pred!("Edge", "x", "x"))),
        every!(
            "x",
            every!(
                "y",
                imply!(pred!("Edge", "x", "y"), pred!("Edge", "y", "x"))
            )
        ),
        every!(
            "x",
            every!(
                "y",
                imply!(
                    pred!("Edge", "x", "y"),
                    not!(equiv!(pred!("Black", "x"), pred!("Black", "y")))
                )
            )
        )
    ));
    let input = first_order!(every!("x", not!(pred!("Edge", "x", "x"))));
    let input = first_order!(every!("x", imply!(pred!("R", "x"), pred!("S", "x"))));
    let input = first_order!(exist!(
        "x",
        exist!(
            "y",
            conjunction!(pred!("R", "x"), pred!("S", "x", "y"), pred!("T", "y"))
        )
    ));
    let input = first_order!(every!(
        "x",
        every!(
            "y",
            every!(
                "z",
                imply!(
                    and!(pred!("E", "x", "y"), pred!("E", "y", "z")),
                    pred!("E", "x", "z")
                )
            )
        )
    ));

    let start = Instant::now();
    let prop = first_order_to_propositional(
        &input,
        &[Integer {
            vars: vec![
                input.get_id(&"x".to_string()),
                input.get_id(&"y".to_string()),
                input.get_id(&"z".to_string()),
            ],
            card: 2,
        }],
    );
    let compilation = start.elapsed();
    let count = enumerate(&prop).count();
    let duration = start.elapsed();

    for i in enumerate(&prop) {
        println!("{i:#?}");
    }

    //let circuit = propositional_to_circuit(&prop);

    println!("{input}");
    println!("{prop}");
    println!("count: {count}");
    println!("compilation: {compilation:?}");
    println!("total: {duration:?}");
    //println!("{circuit}");

    //let count = circuit.eval(&[true; 25].iter().copied().collect());
    //println!("pc: {count}");
}
