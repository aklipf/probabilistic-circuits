use super::index::Indexing;
use super::node::{Node, Symbols};
use super::tree::*;

use std::fmt::Display;

impl<IDX: Indexing> Node<IDX> {
    pub(super) fn fmt_recursive(
        &self,
        tree: &Tree,
        f: &mut std::fmt::Formatter,
    ) -> std::fmt::Result {
        match self.symbol() {
            Symbols::Variable { var_id } => write!(f, "{}", tree.variables[var_id.addr()]),
            Symbols::Not => {
                write!(f, "\u{00AC}")?;
                tree.nodes[self.childs()[0].addr()].fmt_recursive(tree, f)
            }
            Symbols::And => {
                write!(f, "(")?;
                tree.nodes[self.childs()[0].addr()].fmt_recursive(tree, f)?;
                write!(f, "\u{2227}")?;
                tree.nodes[self.childs()[1].addr()].fmt_recursive(tree, f)?;
                write!(f, ")")
            }
            Symbols::Or => {
                write!(f, "(")?;
                tree.nodes[self.childs()[0].addr()].fmt_recursive(tree, f)?;
                write!(f, "\u{2228}")?;
                tree.nodes[self.childs()[1].addr()].fmt_recursive(tree, f)?;
                write!(f, ")")
            }
            Symbols::Predicate { pred_id } => {
                write!(f, "{}(", tree.predicates[pred_id.addr()].0)?;
                if self.num_childs() == 0 {
                    write!(f, ")")
                } else {
                    let mut node = &tree.nodes[self.childs()[0].addr()];
                    node.fmt_recursive(tree, f)?;
                    while node.num_childs() != 0 {
                        node = &tree.nodes[node.childs()[0].addr()];
                        write!(f, ", ")?;
                        node.fmt_recursive(tree, f)?;
                    }
                    write!(f, ")")
                }
            }
            Symbols::All { var_id } => {
                write!(f, "\u{2200}{}:(", tree.variables[var_id.addr()])?;
                tree.nodes[self.childs()[0].addr()].fmt_recursive(tree, f)?;
                write!(f, ")")
            }
            Symbols::Any { var_id } => {
                write!(f, "\u{2203}{}:(", tree.variables[var_id.addr()])?;
                tree.nodes[self.childs()[0].addr()].fmt_recursive(tree, f)?;
                write!(f, ")")
            }
            Symbols::None => panic!("Unkown node None"),
        }
    }
}

impl Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.nodes[self.output.addr()].fmt_recursive(&self, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::expr::*;

    #[test]
    fn test_fmt() {
        let tree: Tree = all("x", any("y", and(or(not(var("A")), var("x")), var("y")))).into();
        assert_eq!(format!("{tree}"), "∀x:(∃y:(((¬A∨x)∧y)))");
        let tree: Tree = all(
            "x",
            any(
                "y",
                and(
                    predicate("pred_x", &["x"]),
                    predicate("pred_xy", &["x", "y"]),
                ),
            ),
        )
        .into();
        assert_eq!(format!("{tree}"), "∀x:(∃y:((pred_x(x)∧pred_xy(x, y))))");
    }
}