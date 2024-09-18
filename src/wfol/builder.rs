use super::expr::Expression;
use super::index::Indexing;
use super::node::{Node, Symbols};
use super::tree::Tree;

use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Builder<'a, IDX: Indexing = u32> {
    pub(super) tree: &'a mut Tree<IDX>,
    pub(super) parent: IDX,
}

impl<'a, IDX: Indexing> Builder<'a, IDX> {
    #[inline]
    fn push_unary<F: Fn(&mut Self) -> IDX>(&mut self, symbol: Symbols<IDX>, inner: F) -> IDX {
        let parent_idx = self.parent;
        let current_idx = self.tree.allocate();

        self.parent = current_idx;

        let inner_idx = inner(self);

        self.tree.nodes[current_idx.addr()] = Node {
            parent: parent_idx,
            childs: [inner_idx, IDX::None],
            symbol: symbol,
        };
        current_idx
    }

    #[inline]
    fn push_binary<F: Fn(&mut Self) -> IDX, G: Fn(&mut Self) -> IDX>(
        &mut self,
        symbol: Symbols<IDX>,
        left: F,
        right: G,
    ) -> IDX {
        let parent_idx = self.parent;
        let current_idx = self.tree.allocate();

        self.parent = current_idx;

        let left_idx = left(self);
        let right_idx = right(self);

        self.tree.nodes[current_idx.addr()] = Node {
            parent: parent_idx,
            childs: [left_idx, right_idx],
            symbol: symbol,
        };
        current_idx
    }

    #[inline]
    pub fn var(&mut self, var_id: IDX) -> IDX {
        self.tree.push(Node {
            parent: self.parent,
            childs: [IDX::None, IDX::None],
            symbol: Symbols::Variable { var_id: var_id },
        })
    }

    #[inline]
    pub fn not<F: Fn(&mut Self) -> IDX>(&mut self, inner: F) -> IDX {
        self.push_unary(Symbols::Not, inner)
    }

    #[inline]
    pub fn and<F: Fn(&mut Self) -> IDX, G: Fn(&mut Self) -> IDX>(
        &mut self,
        left: F,
        right: G,
    ) -> IDX {
        self.push_binary(Symbols::And, left, right)
    }

    #[inline]
    pub fn or<F: Fn(&mut Self) -> IDX, G: Fn(&mut Self) -> IDX>(
        &mut self,
        left: F,
        right: G,
    ) -> IDX {
        self.push_binary(Symbols::Or, left, right)
    }

    #[inline]
    fn pred_arg(&mut self, current_idx: IDX, var_id: IDX) {
        self.tree.nodes[current_idx.addr()] = Node {
            parent: current_idx.offset(-1),
            childs: [current_idx.offset(1), IDX::None],
            symbol: Symbols::Variable { var_id: var_id },
        }
    }

    #[inline]
    pub fn pred(&mut self, pred_id: IDX, vars_id: &[IDX]) -> IDX {
        let begin_idx = self.tree.allocate_n(vars_id.len() + 1);

        self.tree.nodes[begin_idx.addr()] = Node {
            parent: self.parent,
            childs: [begin_idx.offset(1), IDX::None],
            symbol: Symbols::Predicate { pred_id: pred_id },
        };

        for (idx, var_id) in vars_id.iter().enumerate() {
            self.pred_arg(begin_idx.offset(idx + 1), *var_id);
        }

        self.tree.nodes[begin_idx.offset(vars_id.len()).addr()].childs[0] = IDX::None;
        begin_idx
    }

    #[inline]
    pub fn all<F: Fn(&mut Self) -> IDX>(&mut self, var_id: IDX, inner: F) -> IDX {
        self.push_unary(Symbols::All { var_id: var_id }, inner)
    }

    #[inline]
    pub fn any<F: Fn(&mut Self) -> IDX>(&mut self, var_id: IDX, inner: F) -> IDX {
        self.push_unary(Symbols::Any { var_id: var_id }, inner)
    }

    pub fn from_expr(&mut self, expr: &Expression) -> IDX {
        let map = self.init_var_pred(expr);
        self.push_recursive(expr, &map)
    }

    fn push_recursive<T: StringToIdx<IDX>>(&mut self, expr: &Expression, map: &T) -> IDX {
        match expr {
            Expression::Variable { name } => self.var(map.get_var(name)),
            Expression::Predicate { name, vars } => {
                self.pred(map.get_pred(name), &map.get_vars(vars)[..])
            }
            Expression::Not { expr } => self.not(|builder| builder.push_recursive(expr, map)),
            Expression::And { left, right } => self.and(
                |builder| builder.push_recursive(left, map),
                |builder| builder.push_recursive(right, map),
            ),
            Expression::Or { left, right } => self.or(
                |builder| builder.push_recursive(left, map),
                |builder| builder.push_recursive(right, map),
            ),
            Expression::All { var, expr } => self.all(map.get_var(var), |builder| {
                builder.push_recursive(expr, map)
            }),
            Expression::Any { var, expr } => self.any(map.get_var(var), |builder| {
                builder.push_recursive(expr, map)
            }),
        }
    }

    fn init_var_pred(&mut self, expr: &Expression) -> (HashMap<String, IDX>, HashMap<String, IDX>) {
        let mut var_set: HashSet<String> = Default::default();
        let mut pred_map: HashMap<String, usize> = Default::default();

        Self::recursive_collect(&mut var_set, &mut pred_map, expr);

        self.tree.variables = var_set.iter().cloned().collect();

        self.tree.predicates = pred_map
            .iter()
            .map(|(name, order): (&String, &usize)| (name.clone(), *order))
            .collect();

        (
            self.tree
                .variables
                .iter()
                .enumerate()
                .map(|(idx, name)| (name.clone(), IDX::from(idx)))
                .collect(),
            self.tree
                .predicates
                .iter()
                .enumerate()
                .map(|(idx, (name, _))| (name.clone(), IDX::from(idx)))
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

trait StringToIdx<IDX: Indexing = u32> {
    fn get_pred(&self, name: &String) -> IDX;
    fn get_var(&self, name: &String) -> IDX;
    fn get_vars(&self, names: &Vec<String>) -> Vec<IDX>;
}

impl<IDX: Indexing> StringToIdx<IDX> for (HashMap<String, IDX>, HashMap<String, IDX>) {
    fn get_pred(&self, name: &String) -> IDX {
        *self
            .1
            .get(name)
            .expect(format!("Unknown predicate {}", name.as_str()).as_str())
    }

    fn get_var(&self, name: &String) -> IDX {
        *self
            .0
            .get(name)
            .expect(format!("Unknown variable {}", name.as_str()).as_str())
    }

    fn get_vars(&self, names: &Vec<String>) -> Vec<IDX> {
        names.iter().map(|x| self.get_var(x)).collect::<Vec<IDX>>()
    }
}
