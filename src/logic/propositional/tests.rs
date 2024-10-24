use crate::tree::Mapping;

use super::*;

#[test]
fn build() {
    assert_eq!(
        format!("{}", PropositionalTree::build(|builder| builder.var("A"))),
        "A"
    );
    assert_eq!(
        format!(
            "{}",
            PropositionalTree::build(|builder| builder.not(|inner| inner.var("A")))
        ),
        "\u{00AC}A"
    );
    assert_eq!(
        format!(
            "{}",
            PropositionalTree::build(|builder| builder.and(
                |left| left.not(|inner| inner.var("A")),
                |right| right.var("B")
            ))
        ),
        "(\u{00AC}A\u{2227}B)"
    );
    assert_eq!(
        format!(
            "{}",
            PropositionalTree::build(|builder| builder.or(
                |left| left.not(|inner| inner.not(|inner| inner.var("A"))),
                |right| right.var("B")
            ))
        ),
        "(\u{00AC}\u{00AC}A\u{2228}B)"
    );
    assert_eq!(
        format!(
            "{}",
            PropositionalTree::build(|builder| builder.or(
                |left| left.var("A"),
                |right| right.and(
                    |left| left.var("B"),
                    |right| right.not(|inner| inner.var("B"))
                )
            ))
        ),
        "(A\u{2228}(B\u{2227}\u{00AC}B))"
    );
}

#[test]
fn eval() {
    let tree = PropositionalTree::build(|builder| {
        builder.or(
            |left| left.var("A"),
            |right| {
                right.and(
                    |left| left.var("B"),
                    |right| right.not(|inner| inner.var("C")),
                )
            },
        )
    });

    for x in 0..8 {
        let assignment = vec![x & 1 != 0, x & 2 != 0, x & 4 != 0];
        let a = assignment[tree.get_id(&"A".to_string()).addr()];
        let b = assignment[tree.get_id(&"B".to_string()).addr()];
        let c = assignment[tree.get_id(&"C".to_string()).addr()];

        assert_eq!(tree.eval(&assignment), a || (b && (!c)));
    }
}
