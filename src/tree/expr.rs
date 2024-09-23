use std::collections::{HashMap, HashSet};

use super::{
    builder::Builder,
    index::Indexing,
    mapping::{AddPredicate, Mapping, VerifiedMapping},
    pool::Pool,
    tree::Tree,
};

pub fn var(name: &str) -> Expression {
    Expression::Variable {
        name: name.to_string(),
    }
}

pub fn predicate(name: &str, vars: &[&str]) -> Expression {
    Expression::Predicate {
        name: name.to_string(),
        vars: vars.iter().map(|x| x.to_string()).collect(),
    }
}

pub fn not(expr: Expression) -> Expression {
    Expression::Not {
        expr: Box::new(expr),
    }
}

pub fn and(left: Expression, right: Expression) -> Expression {
    Expression::And {
        left: Box::new(left),
        right: Box::new(right),
    }
}

pub fn or(left: Expression, right: Expression) -> Expression {
    Expression::Or {
        left: Box::new(left),
        right: Box::new(right),
    }
}

pub fn imply(left: Expression, right: Expression) -> Expression {
    or(not(left), right)
}

pub fn equivalent(left: Expression, right: Expression) -> Expression {
    and(or(not(left.clone()), right.clone()), or(left, not(right)))
}

pub fn every(name: &str, expr: Expression) -> Expression {
    Expression::All {
        var: name.to_string(),
        expr: Box::new(expr),
    }
}

pub fn exist(name: &str, expr: Expression) -> Expression {
    Expression::Any {
        var: name.to_string(),
        expr: Box::new(expr),
    }
}

#[derive(Clone, Debug)]
pub enum Expression {
    Variable {
        name: String,
    },
    Predicate {
        name: String,
        vars: Vec<String>,
    },
    Not {
        expr: Box<Expression>,
    },
    And {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Or {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    All {
        var: String,
        expr: Box<Expression>,
    },
    Any {
        var: String,
        expr: Box<Expression>,
    },
}

impl<IDX: Indexing + Default> From<Expression> for Tree<IDX> {
    fn from(expr: Expression) -> Self {
        let mut var_set: HashSet<String> = Default::default();
        let mut pred_map: HashMap<String, usize> = Default::default();

        Expression::recursive_collect(&mut var_set, &mut pred_map, &expr);

        let mut tree: Tree<IDX> = Tree::new(var_set, pred_map);

        let map = VerifiedMapping::<IDX>::from(&tree);

        tree.builder(|builder| Expression::push_recursive(builder, &expr, &map));
        tree
    }
}

impl Expression {
    fn push_recursive<IDX: Indexing, T: Mapping<IDX>, P: Pool<IDX = IDX> + AddPredicate<IDX>>(
        builder: &mut Builder<IDX, P>,
        expr: &Expression,
        map: &T,
    ) -> IDX {
        match expr {
            Expression::Variable { name } => builder.var(map.get_var(name)),
            Expression::Predicate { name, vars } => {
                builder.pred(map.get_pred(name), &map.get_vars(vars)[..])
            }
            Expression::Not { expr } => builder.not(|inner| Self::push_recursive(inner, expr, map)),
            Expression::And { left, right } => builder.and(
                |inner| Self::push_recursive(inner, left, map),
                |inner| Self::push_recursive(inner, right, map),
            ),
            Expression::Or { left, right } => builder.or(
                |inner| Self::push_recursive(inner, left, map),
                |inner| Self::push_recursive(inner, right, map),
            ),
            Expression::All { var, expr } => builder.every(map.get_var(var), |inner| {
                Self::push_recursive(inner, expr, map)
            }),
            Expression::Any { var, expr } => builder.exist(map.get_var(var), |inner| {
                Self::push_recursive(inner, expr, map)
            }),
        }
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

impl PartialEq for Expression {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Variable { name: self_name }, Self::Variable { name: other_name }) => {
                self_name == other_name
            }
            (
                Self::Predicate {
                    name: self_name,
                    vars: self_vars,
                },
                Self::Predicate {
                    name: other_name,
                    vars: other_vars,
                },
            ) => self_name == other_name && self_vars == other_vars,
            (Self::Not { expr: self_expr }, Self::Not { expr: other_expr }) => {
                self_expr.eq(other_expr)
            }
            (
                Self::And {
                    left: self_left,
                    right: self_right,
                },
                Self::And {
                    left: other_left,
                    right: other_right,
                },
            ) => self_left.eq(other_left) && self_right.eq(other_right),
            (
                Self::Or {
                    left: self_left,
                    right: self_right,
                },
                Self::Or {
                    left: other_left,
                    right: other_right,
                },
            ) => self_left.eq(other_left) && self_right.eq(other_right),
            (
                Self::All {
                    var: self_var,
                    expr: self_expr,
                },
                Self::All {
                    var: other_var,
                    expr: other_expr,
                },
            ) => self_var == other_var && self_expr.eq(other_expr),
            (
                Self::Any {
                    var: self_var,
                    expr: self_expr,
                },
                Self::Any {
                    var: other_var,
                    expr: other_expr,
                },
            ) => self_var == other_var && self_expr.eq(other_expr),
            _ => false,
        }
    }
}

impl Eq for Expression {}

#[cfg(test)]
mod tests {
    use super::*;

    mod eq {
        use super::*;
        #[test]
        fn test_var() {
            assert_eq!(
                Expression::Variable {
                    name: "A".to_string()
                },
                Expression::Variable {
                    name: "A".to_string()
                }
            );
            assert_ne!(
                Expression::Variable {
                    name: "A".to_string()
                },
                Expression::Variable {
                    name: "B".to_string()
                }
            );
            assert_ne!(
                Expression::Variable {
                    name: "A".to_string()
                },
                Expression::Not {
                    expr: Box::new(Expression::Variable {
                        name: "A".to_string()
                    })
                }
            );
        }

        #[test]
        fn test_pred() {
            assert_eq!(
                Expression::Predicate {
                    name: "test".to_string(),
                    vars: vec!["A".to_string(), "B".to_string()]
                },
                Expression::Predicate {
                    name: "test".to_string(),
                    vars: vec!["A".to_string(), "B".to_string()]
                },
            );
            assert_ne!(
                Expression::Predicate {
                    name: "test".to_string(),
                    vars: vec!["A".to_string(), "B".to_string()]
                },
                Expression::Predicate {
                    name: "test2".to_string(),
                    vars: vec!["A".to_string(), "B".to_string()]
                },
            );
            assert_ne!(
                Expression::Predicate {
                    name: "test".to_string(),
                    vars: vec!["A".to_string(), "B".to_string()]
                },
                Expression::Predicate {
                    name: "test".to_string(),
                    vars: vec!["A".to_string()]
                },
            );
            assert_ne!(
                Expression::Predicate {
                    name: "test".to_string(),
                    vars: vec!["A".to_string(), "B".to_string()]
                },
                Expression::Predicate {
                    name: "test".to_string(),
                    vars: vec!["B".to_string(), "A".to_string()]
                },
            );
        }

        #[test]
        fn test_not() {
            assert_eq!(
                Expression::Not {
                    expr: Box::new(Expression::Variable {
                        name: "A".to_string()
                    })
                },
                Expression::Not {
                    expr: Box::new(Expression::Variable {
                        name: "A".to_string()
                    })
                }
            );
            assert_ne!(
                Expression::Not {
                    expr: Box::new(Expression::Variable {
                        name: "A".to_string()
                    })
                },
                Expression::Not {
                    expr: Box::new(Expression::Variable {
                        name: "B".to_string()
                    })
                }
            );
        }

        #[test]
        fn test_and() {
            assert_eq!(
                Expression::And {
                    left: Box::new(Expression::Variable {
                        name: "A".to_string()
                    }),
                    right: Box::new(Expression::Variable {
                        name: "B".to_string()
                    })
                },
                Expression::And {
                    left: Box::new(Expression::Variable {
                        name: "A".to_string()
                    }),
                    right: Box::new(Expression::Variable {
                        name: "B".to_string()
                    })
                }
            );
            assert_ne!(
                Expression::And {
                    left: Box::new(Expression::Variable {
                        name: "A".to_string()
                    }),
                    right: Box::new(Expression::Variable {
                        name: "B".to_string()
                    })
                },
                Expression::And {
                    left: Box::new(Expression::Variable {
                        name: "B".to_string()
                    }),
                    right: Box::new(Expression::Variable {
                        name: "A".to_string()
                    })
                }
            );
            assert_ne!(
                Expression::And {
                    left: Box::new(Expression::Variable {
                        name: "A".to_string()
                    }),
                    right: Box::new(Expression::Variable {
                        name: "B".to_string()
                    })
                },
                Expression::Or {
                    left: Box::new(Expression::Variable {
                        name: "B".to_string()
                    }),
                    right: Box::new(Expression::Variable {
                        name: "A".to_string()
                    })
                }
            );
        }

        #[test]
        fn test_or() {
            assert_eq!(
                Expression::Or {
                    left: Box::new(Expression::Variable {
                        name: "A".to_string()
                    }),
                    right: Box::new(Expression::Variable {
                        name: "B".to_string()
                    })
                },
                Expression::Or {
                    left: Box::new(Expression::Variable {
                        name: "A".to_string()
                    }),
                    right: Box::new(Expression::Variable {
                        name: "B".to_string()
                    })
                }
            );
            assert_ne!(
                Expression::Or {
                    left: Box::new(Expression::Variable {
                        name: "A".to_string()
                    }),
                    right: Box::new(Expression::Variable {
                        name: "B".to_string()
                    })
                },
                Expression::Or {
                    left: Box::new(Expression::Variable {
                        name: "B".to_string()
                    }),
                    right: Box::new(Expression::Variable {
                        name: "A".to_string()
                    })
                }
            );
            assert_ne!(
                Expression::Or {
                    left: Box::new(Expression::Variable {
                        name: "A".to_string()
                    }),
                    right: Box::new(Expression::Variable {
                        name: "B".to_string()
                    })
                },
                Expression::And {
                    left: Box::new(Expression::Variable {
                        name: "A".to_string()
                    }),
                    right: Box::new(Expression::Variable {
                        name: "B".to_string()
                    })
                }
            );
        }

        #[test]
        fn test_every() {
            assert_eq!(
                Expression::All {
                    var: "x".to_string(),
                    expr: Box::new(Expression::Variable {
                        name: "x".to_string()
                    })
                },
                Expression::All {
                    var: "x".to_string(),
                    expr: Box::new(Expression::Variable {
                        name: "x".to_string()
                    })
                }
            );
            assert_ne!(
                Expression::All {
                    var: "x".to_string(),
                    expr: Box::new(Expression::Variable {
                        name: "x".to_string()
                    })
                },
                Expression::All {
                    var: "y".to_string(),
                    expr: Box::new(Expression::Variable {
                        name: "x".to_string()
                    })
                }
            );
            assert_ne!(
                Expression::All {
                    var: "x".to_string(),
                    expr: Box::new(Expression::Variable {
                        name: "x".to_string()
                    })
                },
                Expression::All {
                    var: "x".to_string(),
                    expr: Box::new(Expression::Variable {
                        name: "y".to_string()
                    })
                }
            );
        }

        #[test]
        fn test_exist() {
            assert_eq!(
                Expression::Any {
                    var: "x".to_string(),
                    expr: Box::new(Expression::Variable {
                        name: "x".to_string()
                    })
                },
                Expression::Any {
                    var: "x".to_string(),
                    expr: Box::new(Expression::Variable {
                        name: "x".to_string()
                    })
                }
            );
            assert_ne!(
                Expression::Any {
                    var: "x".to_string(),
                    expr: Box::new(Expression::Variable {
                        name: "x".to_string()
                    })
                },
                Expression::Any {
                    var: "y".to_string(),
                    expr: Box::new(Expression::Variable {
                        name: "x".to_string()
                    })
                }
            );
            assert_ne!(
                Expression::Any {
                    var: "x".to_string(),
                    expr: Box::new(Expression::Variable {
                        name: "x".to_string()
                    })
                },
                Expression::Any {
                    var: "x".to_string(),
                    expr: Box::new(Expression::Variable {
                        name: "y".to_string()
                    })
                }
            );
        }
    }

    #[test]
    fn test_api() {
        assert_eq!(
            var("A"),
            Expression::Variable {
                name: "A".to_string()
            }
        );

        assert_eq!(
            not(var("A")),
            Expression::Not {
                expr: Box::new(Expression::Variable {
                    name: "A".to_string()
                })
            }
        );

        assert_eq!(
            imply(var("A"), var("B")),
            Expression::Or {
                left: Box::new(Expression::Not {
                    expr: Box::new(Expression::Variable {
                        name: "A".to_string()
                    })
                }),
                right: Box::new(Expression::Variable {
                    name: "B".to_string()
                })
            }
        );

        assert_eq!(
            equivalent(var("A"), var("B")),
            Expression::And {
                left: Box::new(Expression::Or {
                    left: Box::new(Expression::Not {
                        expr: Box::new(Expression::Variable {
                            name: "A".to_string()
                        })
                    }),
                    right: Box::new(Expression::Variable {
                        name: "B".to_string()
                    })
                }),
                right: Box::new(Expression::Or {
                    left: Box::new(Expression::Variable {
                        name: "A".to_string()
                    }),
                    right: Box::new(Expression::Not {
                        expr: Box::new(Expression::Variable {
                            name: "B".to_string()
                        })
                    })
                })
            }
        );

        assert_eq!(
            exist("y", every("x", or(and(not(var("y")), var("x")), var("A")))),
            Expression::Any {
                var: "y".to_string(),
                expr: Box::new(Expression::All {
                    var: "x".to_string(),
                    expr: Box::new(Expression::Or {
                        left: Box::new(Expression::And {
                            left: Box::new(Expression::Not {
                                expr: Box::new(Expression::Variable {
                                    name: "y".to_string()
                                })
                            }),
                            right: Box::new(Expression::Variable {
                                name: "x".to_string()
                            })
                        }),
                        right: Box::new(Expression::Variable {
                            name: "A".to_string()
                        })
                    })
                })
            }
        );
    }
}
