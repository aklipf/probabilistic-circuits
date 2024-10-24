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
