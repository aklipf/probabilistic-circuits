use std::collections::HashSet;

use crate::{
    logic::propositional::PropositionalTree,
    tree::{Addr, IndexedMutRef, IndexedRef, LinkingNode, Mapping},
};

use super::{propositional_to_nnf, PLogic, PMut, PRef};
use std::io;
use std::io::prelude::*;

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

fn distribute(node: &mut IndexedMutRef<PropositionalTree>) -> bool {
    let ref_node = node.get_ref();
    match ref_node.as_ref().value {
        PLogic::And => {
            let left = ref_node.left();
            let right = ref_node.right();
            match (left.as_ref().value, right.as_ref().value) {
                (PLogic::Or, _) => {
                    //println!("before left {}", node.get_ref());

                    let left_idx = left.idx;
                    let right_idx = right.idx;
                    let left_left_idx = left.as_ref().node.operands()[0];
                    let left_right_idx = left.as_ref().node.operands()[1];

                    let cloned_right_idx = node.clone_id(right_idx);
                    IndexedMutRef {
                        array: node.array,
                        idx: left_idx,
                    }
                    .into_and(|_| left_left_idx, |_| right_idx);

                    node.into_or(
                        |_| left_idx,
                        |right| right.and(|_| left_right_idx, |_| cloned_right_idx),
                    );

                    //println!("after {}", node.get_ref());
                    //pause();
                    true
                }
                (_, PLogic::Or) => {
                    //println!("before right {}", node.get_ref());

                    let left_idx = left.idx;
                    let right_idx = right.idx;
                    let right_left_idx = right.as_ref().node.operands()[0];
                    let right_right_idx = right.as_ref().node.operands()[1];
                    let cloned_left_idx = node.clone_id(left_idx);

                    IndexedMutRef {
                        array: node.array,
                        idx: right_idx,
                    }
                    .into_and(|_| left_idx, |_| right_left_idx);

                    node.into_or(
                        |_| right_idx,
                        |right| right.and(|_| cloned_left_idx, |_| right_right_idx),
                    );

                    //println!("after {}", node.get_ref());
                    //pause();
                    true
                }
                _ => false,
            }
        }
        _ => false,
    }
}

fn distribute_nodes<'a>(node: &'a mut IndexedMutRef<'a, PropositionalTree>) {
    let childs: Vec<Addr> = node.as_ref().node.operands().iter().copied().collect();
    for addr in childs {
        if addr.is_addr() {
            distribute_nodes(&mut IndexedMutRef {
                array: &mut node.array,
                idx: addr,
            });
        }
    }

    if !distribute(node) {
        return;
    }

    if node.as_ref().node.parent().is_addr() {
        distribute_nodes(&mut IndexedMutRef {
            array: node.array,
            idx: node.idx,
        });
    } else {
        distribute_nodes(node)
    }
}

fn collect_clause(node: &IndexedRef<PropositionalTree>, clause: &mut Vec<Option<bool>>) -> bool {
    match node.as_ref().value {
        PLogic::Variable { id } => {
            if clause[id.addr()] == Some(false) {
                false
            } else {
                clause[id.addr()] = Some(true);
                true
            }
        }
        PLogic::Not => match node.inner().as_ref().value {
            PLogic::Variable { id } => {
                if clause[id.addr()] == Some(true) {
                    false
                } else {
                    clause[id.addr()] = Some(false);
                    true
                }
            }
            _ => panic!(),
        },
        PLogic::And => {
            if !collect_clause(&node.left(), clause) {
                return false;
            }
            if !collect_clause(&node.right(), clause) {
                return false;
            }
            true
        }
        _ => panic!(),
    }
}

fn collect_clauses(node: &IndexedRef<PropositionalTree>, clauses: &mut HashSet<Vec<Option<bool>>>) {
    match node.as_ref().value {
        PLogic::Or => {
            collect_clauses(&node.left(), clauses);
            collect_clauses(&node.right(), clauses);
        }
        _ => {
            let mut clause: Vec<Option<bool>> = (0..node.array.num_named()).map(|_| None).collect();
            if collect_clause(node, &mut clause) {
                clauses.insert(clause);
            }
        }
    }
}

fn print_pattern(a: &Vec<Option<bool>>) {
    for var in a {
        match var {
            Some(true) => {
                print!("1")
            }
            Some(false) => {
                print!("0")
            }
            None => {
                print!("X")
            }
        }
    }
    println!("")
}
fn print_bool(a: &Vec<bool>) {
    for var in a {
        match var {
            true => {
                print!("1")
            }
            false => {
                print!("0")
            }
        }
    }
    println!("")
}

pub fn count_propositional(tree: &PropositionalTree) -> usize {
    let mut nnf = propositional_to_nnf(tree);
    distribute_nodes(&mut nnf.mut_output());
    let mut clauses: HashSet<Vec<Option<bool>>> = Default::default();
    collect_clauses(&nnf.output(), &mut clauses);

    let mut solutions: HashSet<Vec<bool>> = Default::default();

    for clause in clauses {
        let n = clause.iter().filter(|x| x.is_none()).count();

        for i in 0usize..(1 << n) {
            let mut fill = i;
            let current: Vec<bool> = clause
                .iter()
                .map(|x| match x {
                    &Some(a) => a,
                    None => {
                        let current = fill;
                        fill >>= 1;
                        (1 & current) != 0
                    }
                })
                .collect();

            solutions.insert(current);
        }
    }

    solutions.len() as usize
}

pub fn nnf_to_dnf(tree: &PropositionalTree) -> PropositionalTree {
    let mut dnf = tree.clone();
    distribute_nodes(&mut dnf.mut_output());
    let mut clauses: HashSet<Vec<Option<bool>>> = Default::default();
    collect_clauses(&dnf.output(), &mut clauses);

    PropositionalTree::build(|builder| {
        builder.array.copy_named(&dnf);
        builder.disjunction(&mut clauses.iter(), |clause_builder, clause| {
            clause_builder.conjunction(
                &mut clause
                    .iter()
                    .enumerate()
                    .filter_map(|(id, sign)| match sign {
                        &Some(s) => Some((Addr::new(id), s)),
                        None => None,
                    }),
                |var_builder, (var_id, sign)| {
                    if sign {
                        var_builder.var(var_id)
                    } else {
                        var_builder.not(|inner| inner.var(var_id))
                    }
                },
            )
        })
    })
}
