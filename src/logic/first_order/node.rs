use std::ops::Index;

use crate::{
    logic::fragment::FragmentNode,
    tree::{
        index::Indexing,
        node::{LinkinNode, Node},
        traits::Mapping,
    },
};

use super::FirstOrderLogic;

pub type FirstOrderNode<I> = Node<I, FirstOrderLogic<I>, 2>;

impl<I: Indexing> FragmentNode<I, FirstOrderLogic<I>, 2> for FirstOrderNode<I> {
    fn fmt_display<T: Mapping<I> + Index<I, Output = Self>>(
        &self,
        f: &mut std::fmt::Formatter,
        tree: &T,
    ) -> std::fmt::Result {
        match self.symbol() {
            FirstOrderLogic::Predicate { id } => {
                match tree.get_named(id) {
                    Some(pred_name) => write!(f, "{}(", pred_name),
                    None => write!(f, "Anon{}(", id.addr()),
                }?;

                let mut first_var = true;
                let mut node_id = self.operand()[0];
                if node_id.is_none() {
                    return write!(f, ")");
                }

                while node_id.is_addr() {
                    if first_var {
                        first_var = false;
                    } else {
                        write!(f, ", ")?;
                    }

                    match tree[node_id].symbol() {
                        FirstOrderLogic::Predicate { id: var_id } => match tree.get_named(var_id) {
                            Some(name) => write!(f, "{}", name),
                            None => write!(f, "Anon{}", var_id.addr()),
                        },
                        _ => panic!(),
                    }?;

                    node_id = tree[node_id].operand()[0];
                }
                write!(f, ")")
            }
            FirstOrderLogic::Not => {
                write!(f, "\u{00AC}")?;
                tree[self.operand()[0]].fmt_display(f)
            }
            FirstOrderLogic::And => {
                write!(f, "(")?;
                tree[self.operand()[0]].fmt_display(f)?;
                write!(f, "\u{2227}")?;
                tree[self.operand()[1]].fmt_display(f)?;
                write!(f, ")")
            }
            FirstOrderLogic::Or => {
                write!(f, "(")?;
                tree[self.operand()[0]].fmt_display(f)?;
                write!(f, "\u{2228}")?;
                tree[self.operand()[1]].fmt_display(f)?;
                write!(f, ")")
            }
            FirstOrderLogic::Universal { id } => {
                write!(
                    f,
                    "\u{2200}{}:",
                    tree.get_named(id).unwrap_or(&format!("Anon{}", id.addr()))
                )?;
                tree[self.operand()[0]].fmt_display(f)
            }
            FirstOrderLogic::Existential { id } => {
                write!(
                    f,
                    "\u{2203}{}:",
                    tree.get_named(id).unwrap_or(&format!("Anon{}", id.addr()))
                )?;
                tree[self.operand()[0]].fmt_display(f)
            }
            FirstOrderLogic::None => panic!("Can't display a partially initialised tree."),
        }
    }

    fn arity(&self) -> usize {
        match self.symbol() {
            FirstOrderLogic::Predicate { id: _ } => 0,
            FirstOrderLogic::Universal { id: _ } => 1,
            FirstOrderLogic::Existential { id: _ } => 1,
            FirstOrderLogic::Not => 1,
            FirstOrderLogic::And => 2,
            FirstOrderLogic::Or => 2,
            FirstOrderLogic::None => 0,
        }
    }

    fn new(symbol: FirstOrderLogic<I>, operands: &[I]) -> Self {
        Self::new(symbol, operands)
    }

    fn duplicate(&self, operands: &[I]) -> Self {
        Self::new(self.symbol(), operands)
    }
}
