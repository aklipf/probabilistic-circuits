use super::{index::Indexing, tree::Tree};

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

pub fn all(name: &str, expr: Expression) -> Expression {
    Expression::All {
        var: name.to_string(),
        expr: Box::new(expr),
    }
}

pub fn imply(left: Expression, right: Expression) -> Expression {
    or(not(left), right)
}

pub fn equivalent(left: Expression, right: Expression) -> Expression {
    and(or(not(left.clone()), right.clone()), or(left, not(right)))
}

pub fn any(name: &str, expr: Expression) -> Expression {
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
        let mut tree: Tree<IDX> = Default::default();
        let (var_map, pred_map) = tree.init(&expr);
        tree.output = tree.push_recursive(&expr, IDX::None, &var_map, &pred_map);
        tree
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
        fn test_all() {
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
        fn test_any() {
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
            any("y", all("x", or(and(not(var("y")), var("x")), var("A")))),
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
