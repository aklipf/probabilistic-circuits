use std::collections::HashMap;

use crate::{
    logic::propositional::{PMut, PropositionalTree},
    solver::domain::Integer,
    tree::{Addr, IndexedMutRef, IndexedRef},
};

use super::{ground::Grounded, Args, FOLogic, FORef, FirstOrderTree};

fn fo2p_recusive(
    src: IndexedRef<FirstOrderTree>,
    dst: &mut IndexedMutRef<PropositionalTree>,
    grounded: &Vec<Grounded>,
    domains: &[Integer],
    values: &HashMap<Addr, usize>,
) -> Addr {
    match src.as_ref().value {
        FOLogic::Not => {
            dst.not(|inner| fo2p_recusive(src.inner().unwrap(), inner, grounded, domains, values))
        }
        FOLogic::And => dst.and(
            |left| fo2p_recusive(src.left().unwrap(), left, grounded, domains, values),
            |right| fo2p_recusive(src.right().unwrap(), right, grounded, domains, values),
        ),
        FOLogic::Or => dst.or(
            |left| fo2p_recusive(src.left().unwrap(), left, grounded, domains, values),
            |right| fo2p_recusive(src.right().unwrap(), right, grounded, domains, values),
        ),
        FOLogic::Predicate { id } => {
            let ground = grounded.iter().find(|&g| g.id == id).unwrap();
            let addr = ground.get_id(src.args().map(|addr| values.get(&addr).unwrap()));
            dst.var(addr)
        }
        FOLogic::Universal { id } => {
            let domain = domains.iter().find(|&x| x.vars.contains(&id)).unwrap();
            dst.conjunction(&mut (0..domain.card), |inner, value| {
                let mut current_values = values.clone();
                current_values.insert(id, value);

                fo2p_recusive(
                    src.inner().unwrap(),
                    inner,
                    grounded,
                    domains,
                    &current_values,
                )
            })
        }
        FOLogic::Existential { id } => {
            let domain = domains.iter().find(|&x| x.vars.contains(&id)).unwrap();
            dst.disjunction(&mut (0..domain.card), |inner, value| {
                let mut current_values = values.clone();
                current_values.insert(id, value);

                fo2p_recusive(
                    src.inner().unwrap(),
                    inner,
                    grounded,
                    domains,
                    &current_values,
                )
            })
        }
    }
}

pub fn first_order_to_propositional(
    tree: &FirstOrderTree,
    domains: &[Integer],
) -> PropositionalTree {
    tree.compile(|src, dst| {
        let grounded = Grounded::ground(tree, dst.array, domains).unwrap();
        fo2p_recusive(src, dst, &grounded, domains, &Default::default())
    })
}
