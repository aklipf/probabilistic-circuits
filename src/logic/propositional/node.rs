use crate::tree::{
    index::Indexing,
    node::{LinkinNode, Node},
    traits::Mapping,
    tree::Tree,
    view::NodeView,
};

use super::{super::fragment::FragmentNode, PropositionalLogic};

pub type PropositionalNode<I> = Node<I, PropositionalLogic<I>, 2>;

/*impl<I: Indexing> Deref for PropositionalNode<I> {
    type Target = PropositionalLogic<I>;

    fn deref(&self) -> &Self::Target {
        &self.symbol
    }
}*/

impl<I: Indexing> FragmentNode<I, PropositionalLogic<I>, 2> for PropositionalNode<I> {
    fn fmt_display(
        f: &mut std::fmt::Formatter,
        node: &NodeView<PropositionalLogic<I>, I, 2>,
    ) -> std::fmt::Result {
        match node.symbol() {
            PropositionalLogic::Variable { id } => {
                write!(
                    f,
                    "{}",
                    node.get_named(id).unwrap_or(&format!("Anon{}", id.addr()))
                )
            }
            PropositionalLogic::Not => {
                write!(f, "\u{00AC}")?;
                Self::fmt_display(f, &node.input())
            }
            PropositionalLogic::And => {
                write!(f, "(")?;
                Self::fmt_display(f, &node.left())?;
                write!(f, "\u{2227}")?;
                Self::fmt_display(f, &node.right())?;
                write!(f, ")")
            }
            PropositionalLogic::Or => {
                write!(f, "(")?;
                Self::fmt_display(f, &node.left())?;
                write!(f, "\u{2228}")?;
                Self::fmt_display(f, &node.right())?;
                write!(f, ")")
            }
            PropositionalLogic::None => panic!("Can't display a partially initialised tree."),
        }
    }

    fn arity(&self) -> usize {
        match self.symbol() {
            PropositionalLogic::Variable { id: _ } => 0,
            PropositionalLogic::Not => 1,
            PropositionalLogic::And => 2,
            PropositionalLogic::Or => 2,
            PropositionalLogic::None => 0,
        }
    }
}
