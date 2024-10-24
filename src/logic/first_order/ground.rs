use crate::{
    logic::propositional::PLogic,
    solver::domain::Integer,
    tree::{builder::Builder, index::Indexing, tree::Tree},
};

use super::FirstOrderLogic;
/*
pub trait Ground<I: Indexing> {
    type Grounded;
    type Domain;
    fn ground(&self, domain: &Vec<Self::Domain>) -> Self::Grounded;
}

fn ground_not<I: Indexing>(
    fol: &Tree<FirstOrderLogic<I>, I, 2>,
    node_id: I,
    domain: &Vec<Integer<I>>,
) -> impl Fn(&mut Builder<Tree<PLogic<I>, I, 2>, 2>) -> I + 'static {
    |builder| builder.not(recursive_ground::<I>(fol, node_id, domain))
}

fn recursive_ground<I: Indexing>(
    fol: &Tree<FirstOrderLogic<I>, I, 2>,
    node_id: I,
    domain: &Vec<Integer<I>>,
) -> impl Fn(&mut Builder<Tree<PLogic<I>, I, 2>, 2>) -> I {
    let node = fol[node_id];
    match node.symbol() {
        FirstOrderLogic::Predicate { id } => todo!(),
        FirstOrderLogic::Universal { id } => todo!(),
        FirstOrderLogic::Existential { id } => todo!(),
        FirstOrderLogic::Not => {
            let inner_id = node.operands_iter().next().unwrap();
            move |builder: &mut Builder<Tree<PLogic<I>, I, 2>, 2>| {
                builder.not(recursive_ground(fol, inner_id, domain))
            }
        }
        FirstOrderLogic::And => todo!(),
        FirstOrderLogic::Or => todo!(),
        FirstOrderLogic::None => todo!(),
    }
}

impl<I> Ground<I> for Tree<FirstOrderLogic<I>, I, 2>
where
    I: Indexing,
{
    type Grounded = Tree<PLogic<I>, I, 2>;
    type Domain = Integer<I>;

    fn ground(&self, domain: &Vec<Self::Domain>) -> Self::Grounded {
        let mut grounded: Self::Grounded = Default::default();

        grounded.builder(recursive_ground(self.output(), domain));

        grounded
    }
}
 */
