use std::{collections::HashMap, fmt::Debug};

use crate::{
    solver::domain::Integer,
    tree::{Addr, IndexedRef, LinkingNode, Mapping, Tree},
};

use super::{FOLogic, FORef, FirstOrderTree};

pub struct Grounded {
    pub id: Addr,
    pub domains: Vec<Integer>,
    pub grounded: Vec<Addr>,
}

impl Grounded {
    pub fn len(&self) -> usize {
        self.domains.iter().map(|d| d.card).product()
    }

    pub fn get_id<'a, T: Iterator<Item = &'a usize>>(&self, indices: T) -> Addr {
        let addr = [Integer {
            vars: Default::default(),
            card: 1,
        }]
        .iter()
        .chain(self.domains.iter().rev())
        .zip(indices)
        .fold(0usize, |addr, (dom, &idx)| addr * dom.card + idx);

        self.grounded[addr]
    }

    fn format(&self, radical: &String, idx: usize) -> String {
        let mut vars: Vec<usize> = Default::default();
        let mut current_idx = idx;
        for domain in self.domains.iter().rev() {
            vars.push(current_idx % domain.card);
            current_idx = current_idx / domain.card;
        }
        format!(
            "{radical}({})",
            vars.iter()
                .rev()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }

    pub fn ground<U: Copy + Debug + PartialEq, const N: usize>(
        source: &FirstOrderTree,
        target: &mut Tree<U, N>,
        domains: &[Integer],
    ) -> Result<Vec<Grounded>, &'static str> {
        let mut ground: HashMap<Addr, Grounded> = Default::default();
        let node = source.output();
        Self::ground_recursive(node, &mut ground, domains)?;
        for g in ground.values_mut() {
            let radical = source.fmt_named(g.id);
            for idx in 0..g.len() {
                let grounded_name = target.add_named(&g.format(&radical, idx));
                g.grounded.push(grounded_name);
            }
        }
        Ok(ground.into_values().collect())
    }

    fn ground_recursive<'a>(
        node: IndexedRef<'a, FirstOrderTree>,
        ground: &mut HashMap<Addr, Grounded>,
        domains: &[Integer],
    ) -> Result<(), &'static str> {
        if let FOLogic::Predicate { .. } = node.as_ref().value {
            Self::ground_predicate(node, ground, domains)?;
        } else {
            for &child_id in node.as_ref().node.operands() {
                if child_id.is_addr() {
                    Self::ground_recursive(
                        IndexedRef {
                            array: node.array,
                            idx: child_id,
                        },
                        ground,
                        domains,
                    )?;
                }
            }
        }
        Ok(())
    }

    fn ground_predicate<'a>(
        predicate: IndexedRef<'a, FirstOrderTree>,
        ground: &mut HashMap<Addr, Grounded>,
        domains: &[Integer],
    ) -> Result<(), &'static str> {
        if let FOLogic::Predicate { id } = predicate.as_ref().value {
            match ground.get(&id) {
                Some(g) => Self::check_predicate(predicate, g, domains),
                None => {
                    ground.insert(id, Self::add_predicate(predicate, domains)?);
                    Ok(())
                }
            }
        } else {
            panic!()
        }
    }

    fn get_domain(var: Addr, domains: &[Integer]) -> Option<&Integer> {
        for domain in domains {
            if domain.vars.contains(&var) {
                return Some(domain);
            }
        }
        None
    }

    fn check_predicate<'a>(
        predicate: IndexedRef<'a, FirstOrderTree>,
        ground: &Grounded,
        domains: &[Integer],
    ) -> Result<(), &'static str> {
        let grounede_domains = &ground.domains;
        let mut count = 0;
        let mut next = predicate.inner();
        while let Some(variable) = next {
            if let FOLogic::Predicate { id } = variable.as_ref().value {
                let domain = Self::get_domain(id, domains).ok_or("Unkown domain")?;
                if grounede_domains[count] != *domain {
                    return Err("Domains didn't match");
                }
            } else {
                return Err("Invalid predicate");
            }
            next = variable.inner();
            count += 1;
        }
        if count != grounede_domains.len() {
            Err("Arguments don't match")
        } else {
            Ok(())
        }
    }

    fn add_predicate<'a>(
        predicate: IndexedRef<'a, FirstOrderTree>,
        domains: &[Integer],
    ) -> Result<Grounded, &'static str> {
        let pred_id = if let FOLogic::Predicate { id } = predicate.as_ref().value {
            id
        } else {
            return Err("Invalid predicate");
        };

        let mut grounded_domains: Vec<Integer> = Default::default();
        let mut next = predicate.inner();
        while let Some(variable) = next {
            if let FOLogic::Predicate { id } = variable.as_ref().value {
                let domain = Self::get_domain(id, domains).ok_or("Unkown domain")?;
                grounded_domains.push(domain.clone());
            } else {
                return Err("Invalid predicate");
            }
            next = variable.inner();
        }

        Ok(Grounded {
            id: pred_id,
            domains: grounded_domains,
            grounded: Default::default(),
        })
    }
}
