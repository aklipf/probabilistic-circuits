use std::time::Instant;

use probabilistic_circuits::{
    and, conjunction, equiv, every, first_order, imply,
    logic::first_order::{first_order_to_propositional, FOMut, FirstOrderLogic},
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

    let start = Instant::now();
    let prop = first_order_to_propositional(
        &input,
        &[Integer {
            vars: vec![
                input.get_id(&"x".to_string()),
                input.get_id(&"y".to_string()),
            ],
            card: 4,
        }],
    );
    let compilation = start.elapsed();
    let count = enumerate(&prop).count();
    let duration = start.elapsed();

    println!("{prop}");
    println!("count: {count}");
    println!("compilation: {compilation:?}");
    println!("total: {duration:?}");
}
