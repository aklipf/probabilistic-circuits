use super::index::Indexing;
use super::tree::*;

use std::fmt::Display;

impl<IDX: Indexing> Node<IDX> {
    pub(super) fn fmt_recursive(
        &self,
        tree: &Tree,
        f: &mut std::fmt::Formatter,
    ) -> std::fmt::Result {
        match self {
            Node::Variable { var_id, .. } => write!(f, "{}", tree.variables[var_id.addr()].name),
            Node::Not { inputs, .. } => {
                write!(f, "\u{00AC}")?;
                tree.nodes[inputs[0].addr()].fmt_recursive(tree, f)
            }
            Node::And { inputs, .. } => {
                write!(f, "(")?;
                tree.nodes[inputs[0].addr()].fmt_recursive(tree, f)?;
                write!(f, "\u{2227}")?;
                tree.nodes[inputs[1].addr()].fmt_recursive(tree, f)?;
                write!(f, ")")
            }
            Node::Or { inputs, .. } => {
                write!(f, "(")?;
                tree.nodes[inputs[0].addr()].fmt_recursive(tree, f)?;
                write!(f, "\u{2228}")?;
                tree.nodes[inputs[1].addr()].fmt_recursive(tree, f)?;
                write!(f, ")")
            }
            Node::Predicate {
                pred_id, next_id, ..
            } => {
                write!(f, "{}(", tree.predicates[pred_id.addr()].name)?;
                if next_id.is_none() {
                    write!(f, ")")
                } else {
                    tree.nodes[next_id.addr()].fmt_recursive(tree, f)
                }
            }
            Node::PredicateVariable {
                var_id, next_id, ..
            } => {
                if next_id.is_none() {
                    write!(f, "{})", tree.variables[var_id.addr()].name)
                } else {
                    write!(f, "{}, ", tree.variables[var_id.addr()].name)?;
                    tree.nodes[next_id.addr()].fmt_recursive(tree, f)
                }
            }
            Node::All { var_id, inputs, .. } => {
                write!(f, "\u{2200}{}:(", tree.variables[var_id.addr()].name)?;
                tree.nodes[inputs[0].addr()].fmt_recursive(tree, f)?;
                write!(f, ")")
            }
            Node::Any { var_id, inputs, .. } => {
                write!(f, "\u{2203}{}:(", tree.variables[var_id.addr()].name)?;
                tree.nodes[inputs[0].addr()].fmt_recursive(tree, f)?;
                write!(f, ")")
            }
            Node::None => panic!("Unkown node None"),
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
    }
}
