use std::fmt::Display;
use std::*;

pub trait Expr: Clone + Into<Node> + Display {}

#[derive(Clone)]
pub struct Var {
    pub(super) name: String,
}

#[derive(Clone)]
pub struct Weight {
    pub(super) weight: f32,
    pub(super) child: Box<Node>,
}

#[derive(Clone)]
pub struct Not {
    pub(super) child: Box<Node>,
}

#[derive(Clone)]
pub struct And {
    pub(super) left: Box<Node>,
    pub(super) right: Box<Node>,
}

#[derive(Clone)]
pub struct Or {
    pub(super) left: Box<Node>,
    pub(super) right: Box<Node>,
}

#[derive(Clone)]
pub struct Imply {
    pub(super) left: Box<Node>,
    pub(super) right: Box<Node>,
}

#[derive(Clone)]
pub struct Equivalent {
    pub(super) left: Box<Node>,
    pub(super) right: Box<Node>,
}

#[derive(Clone)]
pub struct Predicate {
    pub(super) name: String,
    pub(super) vars: Vec<Var>,
}

#[derive(Clone)]
pub struct Any {
    pub(super) var: Box<Var>,
    pub(super) expr: Box<Node>,
}

#[derive(Clone)]
pub struct All {
    pub(super) var: Box<Var>,
    pub(super) expr: Box<Node>,
}

#[derive(Clone)]
pub enum Node {
    Var(Var),
    Weight(Weight),
    Not(Not),
    And(And),
    Or(Or),
    Imply(Imply),
    Equivalent(Equivalent),
    Predicate(Predicate),
    Any(Any),
    All(All),
}

impl Expr for Var {}
impl Expr for Weight {}
impl Expr for Not {}
impl Expr for And {}
impl Expr for Or {}
impl Expr for Imply {}
impl Expr for Equivalent {}
impl Expr for Predicate {}
impl Expr for Any {}
impl Expr for All {}

impl From<Var> for Node {
    fn from(var: Var) -> Self {
        Node::Var(var)
    }
}

impl From<Weight> for Node {
    fn from(weigth: Weight) -> Self {
        Node::Weight(weigth)
    }
}

impl From<Not> for Node {
    fn from(not: Not) -> Self {
        Node::Not(not)
    }
}

impl From<And> for Node {
    fn from(and: And) -> Self {
        Node::And(and)
    }
}

impl From<Or> for Node {
    fn from(or: Or) -> Self {
        Node::Or(or)
    }
}

impl From<Imply> for Node {
    fn from(imply: Imply) -> Self {
        Node::Imply(imply)
    }
}

impl From<Equivalent> for Node {
    fn from(equiv: Equivalent) -> Self {
        Node::Equivalent(equiv)
    }
}

impl From<Predicate> for Node {
    fn from(pred: Predicate) -> Self {
        Node::Predicate(pred)
    }
}

impl From<Any> for Node {
    fn from(any: Any) -> Self {
        Node::Any(any)
    }
}

impl From<All> for Node {
    fn from(all: All) -> Self {
        Node::All(all)
    }
}

pub fn var(name: &str) -> Var {
    Var {
        name: name.to_string(),
    }
}

pub fn weight<T: Into<Node>>(weight: f32, expr: T) -> Weight {
    Weight {
        weight: weight,
        child: Box::new(expr.into()),
    }
}

pub fn not<T: Into<Node>>(expr: T) -> Not {
    Not {
        child: Box::new(expr.into()),
    }
}

pub fn and<T: Into<Node>, U: Into<Node>>(left: T, right: U) -> And {
    And {
        left: Box::new(left.into()),
        right: Box::new(right.into()),
    }
}

pub fn or<T: Into<Node>, U: Into<Node>>(left: T, right: U) -> Or {
    Or {
        left: Box::new(left.into()),
        right: Box::new(right.into()),
    }
}

pub fn imply<T: Into<Node>, U: Into<Node>>(left: T, right: U) -> Imply {
    Imply {
        left: Box::new(left.into()),
        right: Box::new(right.into()),
    }
}

pub fn equiv<T: Into<Node>, U: Into<Node>>(left: T, right: U) -> Equivalent {
    Equivalent {
        left: Box::new(left.into()),
        right: Box::new(right.into()),
    }
}

pub fn predi(name: &str, vars: &[&str]) -> Predicate {
    Predicate {
        name: name.to_string(),
        vars: vars.iter().map(|x| var(x)).collect(),
    }
}

pub fn any<T: Into<Node>>(var: Var, expr: T) -> Any {
    Any {
        var: Box::new(var),
        expr: Box::new(expr.into()),
    }
}

pub fn all<T: Into<Node>>(var: Var, expr: T) -> All {
    All {
        var: Box::new(var),
        expr: Box::new(expr.into()),
    }
}
