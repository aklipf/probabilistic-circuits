use crate::{
    solver::{domain::Integer, naive::enumerate},
    tree::Mapping,
};

use super::*;

#[test]
fn build() {
    assert_eq!(
        format!(
            "{}",
            FirstOrderTree::build(|builder| builder.pred("A", &[]))
        ),
        "A()"
    );
    assert_eq!(
        format!(
            "{}",
            FirstOrderTree::build(|builder| builder.exist("x", |inner| inner.pred("A", &["x"])))
        ),
        "∃x:A(x)"
    );
    assert_eq!(
        format!(
            "{}",
            FirstOrderTree::build(|builder| builder.and(
                |left| left.every("x", |inner| inner
                    .not(|inner| inner.pred("Edge", &["x", "x"]))),
                |right| right.every("x", |inner| inner.every("y", |inner| inner.or(
                    |left| left.not(|inner| inner.pred("Edge", &["x", "y"])),
                    |right| right.pred("Edge", &["y", "x"])
                )))
            ))
        ),
        "(∀x:¬Edge(x, x)∧∀x:∀y:(¬Edge(x, y)∨Edge(y, x)))"
    );
}

#[test]
fn compilation_fol() {
    let input = FirstOrderTree::build(|builder| {
        builder.and(
            |left| {
                left.every("x", |inner| {
                    inner.not(|inner| inner.pred("Edge", &["x", "x"]))
                })
            },
            |right| {
                right.every("x", |inner| {
                    inner.every("y", |inner| {
                        inner.or(
                            |left| left.not(|inner| inner.pred("Edge", &["x", "y"])),
                            |right| right.pred("Edge", &["y", "x"]),
                        )
                    })
                })
            },
        )
    });

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
    println!("{prop}");
    println!("counting: {}", enumerate(&prop).count());
}
