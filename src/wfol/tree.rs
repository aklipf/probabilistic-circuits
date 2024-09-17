use super::expr::Expression;
use super::index::Indexing;

use core::panic;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Variable {
    pub(super) name: String,
}

#[derive(Debug)]
pub struct Predicate {
    pub(super) name: String,
    pub(super) order: usize,
}

#[derive(Clone, Debug)]
pub enum Node<IDX: Indexing> {
    Variable {
        output: IDX,
        var_id: IDX,
    },
    Not {
        output: IDX,
        inputs: (IDX,),
    },
    And {
        output: IDX,
        inputs: (IDX, IDX),
    },
    Or {
        output: IDX,
        inputs: (IDX, IDX),
    },
    Predicate {
        output: IDX,
        pred_id: IDX,
        next_id: IDX,
    },
    PredicateVariable {
        previous_id: IDX,
        var_id: IDX,
        next_id: IDX,
    },
    All {
        output: IDX,
        var_id: IDX,
        inputs: (IDX,),
    },
    Any {
        output: IDX,
        var_id: IDX,
        inputs: (IDX,),
    },
    None,
}

impl<IDX: Indexing> Node<IDX> {
    pub(super) fn replace_input(&mut self, old: IDX, new: IDX) {
        match self {
            Node::Variable { .. } => panic!("Variable nodes have no input"),
            Node::Not { inputs, .. } => {
                assert_eq!(old, inputs.0, "old IDX didn't correspond to any input");
                inputs.0 = new;
            }
            Node::And { inputs, .. } => {
                if old == inputs.0 {
                    inputs.0 = new;
                } else if old == inputs.1 {
                    inputs.1 = new;
                } else {
                    panic!("old IDX didn't correspond to any input")
                }
            }
            Node::Or { inputs, .. } => {
                if old == inputs.0 {
                    inputs.0 = new;
                } else if old == inputs.1 {
                    inputs.1 = new;
                } else {
                    panic!("old IDX didn't correspond to any input")
                }
            }
            Node::Predicate { next_id, .. } => {
                assert_eq!(old, *next_id, "old IDX didn't correspond to any input");
                *next_id = new;
            }
            Node::PredicateVariable { next_id, .. } => {
                assert_eq!(old, *next_id, "old IDX didn't correspond to any input");
                *next_id = new;
            }
            Node::All { inputs, .. } => {
                assert_eq!(old, inputs.0, "old IDX didn't correspond to any input");
                inputs.0 = new;
            }
            Node::Any { inputs, .. } => {
                assert_eq!(old, inputs.0, "old IDX didn't correspond to any input");
                inputs.0 = new;
            }
            Node::None => panic!("None nodes have no input"),
        }
    }

    pub(super) fn get_output(&self) -> IDX {
        match self {
            Node::Variable { output, .. } => *output,
            Node::Not { output, .. } => *output,
            Node::And { output, .. } => *output,
            Node::Or { output, .. } => *output,
            Node::Predicate { output, .. } => *output,
            Node::PredicateVariable { previous_id, .. } => *previous_id,
            Node::All { output, .. } => *output,
            Node::Any { output, .. } => *output,
            Node::None => panic!("None nodes have no input"),
        }
    }
}

#[derive(Default, Debug)]
pub struct Tree<IDX: Indexing = u32> {
    pub(crate) variables: Vec<Variable>,
    pub(crate) predicates: Vec<Predicate>,
    pub(crate) nodes: Vec<Node<IDX>>,
    pub(crate) output: IDX,
}

impl<IDX: Indexing> Tree<IDX> {
    pub(super) fn push(&mut self, node: Node<IDX>) -> IDX {
        let idx = IDX::from(self.nodes.len());
        self.nodes.push(node);
        idx
    }

    pub fn remove(&mut self, idx: IDX) {
        let old_idx = IDX::from(self.nodes.len() - 1);

        let node = self.nodes.pop().expect("The tree is empty");

        self.nodes[node.get_output().addr()].replace_input(old_idx, idx);
        self.nodes[idx.addr()] = node;
        todo!("fix this");
    }

    pub(super) fn allocate(&mut self) -> IDX {
        self.push(Node::None)
    }

    pub(super) fn allocate_n(&mut self, n: usize) -> IDX {
        let idx = IDX::from(self.nodes.len());
        self.nodes.resize(self.nodes.len() + n, Node::None);
        idx
    }

    pub(super) fn replace(&mut self, idx: IDX, node: Node<IDX>) -> IDX {
        self.nodes[idx.addr()] = node;
        idx
    }

    pub(super) fn get(&self, idx: IDX) -> &Node<IDX> {
        &self.nodes[idx.addr()]
    }

    pub(super) fn get_mut(&mut self, idx: IDX) -> &mut Node<IDX> {
        &mut self.nodes[idx.addr()]
    }

    pub(super) fn push_recursive(
        &mut self,
        expr: &Expression,
        output: IDX,
        var_map: &HashMap<String, IDX>,
        pred_map: &HashMap<String, IDX>,
    ) -> IDX {
        match expr {
            Expression::Variable { name } => self.push(Node::Variable {
                output: output,
                var_id: *var_map.get(name).unwrap(),
            }),
            Expression::Predicate { name, vars } => {
                let pred_idx = *pred_map.get(name).expect("The predicate is not referenced");
                let pred_order = self.predicates[pred_idx.addr()].order;

                let begin_idx = self.allocate_n(pred_order + 1);

                self.nodes[begin_idx.addr()] = Node::Predicate {
                    pred_id: pred_idx,
                    next_id: begin_idx.offset(1),
                    output: output,
                };

                for (idx, var) in vars.iter().enumerate() {
                    self.nodes[begin_idx.offset(idx + 1).addr()] = Node::PredicateVariable {
                        var_id: *var_map.get(var).unwrap(),
                        next_id: if idx + 1 < pred_order {
                            begin_idx.offset(idx + 2)
                        } else {
                            IDX::None
                        },
                        previous_id: begin_idx.offset(idx),
                    };
                }

                begin_idx
            }
            Expression::Not { expr } => {
                let idx = self.allocate();
                let input = self.push_recursive(expr, idx, var_map, pred_map);

                self.nodes[idx.addr()] = Node::Not {
                    inputs: (input,),
                    output: output,
                };

                idx
            }
            Expression::And { left, right } => {
                let idx = self.allocate();

                let left_idx = self.push_recursive(left, idx, var_map, pred_map);
                let right_idx = self.push_recursive(right, idx, var_map, pred_map);

                self.nodes[idx.addr()] = Node::And {
                    inputs: (left_idx, right_idx),
                    output: idx,
                };

                idx
            }
            Expression::Or { left, right } => {
                let idx = self.allocate();

                let left_idx = self.push_recursive(left, idx, var_map, pred_map);
                let right_idx = self.push_recursive(right, idx, var_map, pred_map);

                self.nodes[idx.addr()] = Node::Or {
                    inputs: (left_idx, right_idx),
                    output: idx,
                };

                idx
            }
            Expression::All { var, expr } => {
                let idx = self.allocate();
                let input = self.push_recursive(expr, idx, var_map, pred_map);

                self.nodes[idx.addr()] = Node::All {
                    inputs: (input,),
                    var_id: *var_map.get(var).unwrap(),
                    output: output,
                };

                idx
            }
            Expression::Any { var, expr } => {
                let idx = self.allocate();
                let input = self.push_recursive(expr, idx, var_map, pred_map);

                self.nodes[idx.addr()] = Node::Any {
                    inputs: (input,),
                    var_id: *var_map.get(var).unwrap(),
                    output: output,
                };

                idx
            }
        }
    }

    pub(super) fn init(
        &mut self,
        expr: &Expression,
    ) -> (HashMap<String, IDX>, HashMap<String, IDX>) {
        let mut var_set: HashSet<String> = Default::default();
        let mut pred_map: HashMap<String, usize> = Default::default();

        Self::recursive_collect(&mut var_set, &mut pred_map, expr);

        self.variables = var_set
            .iter()
            .map(|name| Variable { name: name.clone() })
            .collect();

        self.predicates = pred_map
            .iter()
            .map(|(name, order): (&String, &usize)| Predicate {
                name: name.clone(),
                order: *order,
            })
            .collect();

        (
            self.variables
                .iter()
                .enumerate()
                .map(|(idx, var)| (var.name.clone(), IDX::from(idx)))
                .collect(),
            self.predicates
                .iter()
                .enumerate()
                .map(|(idx, var)| (var.name.clone(), IDX::from(idx)))
                .collect(),
        )
    }

    fn recursive_collect(
        var_set: &mut HashSet<String>,
        pred_map: &mut HashMap<String, usize>,
        expression: &Expression,
    ) {
        match expression {
            Expression::Variable { name } => {
                var_set.insert(name.clone());
            }
            Expression::Predicate { name, vars } => {
                match pred_map.get(name) {
                    Some(x) => assert_eq!(
                        vars.len(),
                        *x,
                        "order of the {name} predicate is inconsistent ({} != {})",
                        vars.len(),
                        *x
                    ),
                    None => {
                        pred_map.insert(name.clone(), vars.len());
                    }
                }
                var_set.extend(vars.iter().map(|x| x.clone()));
            }
            Expression::Not { expr } => {
                Self::recursive_collect(var_set, pred_map, expr);
            }
            Expression::And { left, right } => {
                Self::recursive_collect(var_set, pred_map, left);
                Self::recursive_collect(var_set, pred_map, right);
            }
            Expression::Or { left, right } => {
                Self::recursive_collect(var_set, pred_map, left);
                Self::recursive_collect(var_set, pred_map, right);
            }
            Expression::All { var, expr } => {
                var_set.insert(var.clone());
                Self::recursive_collect(var_set, pred_map, expr);
            }
            Expression::Any { var, expr } => {
                var_set.insert(var.clone());
                Self::recursive_collect(var_set, pred_map, expr);
            }
        }
    }
}
