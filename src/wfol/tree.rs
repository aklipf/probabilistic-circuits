use rc::Rc;
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
    pub(super) child: Rc<Node>,
}

#[derive(Clone)]
pub struct Not {
    pub(super) child: Rc<Node>,
}

#[derive(Clone)]
pub struct And {
    pub(super) left: Rc<Node>,
    pub(super) right: Rc<Node>,
}

#[derive(Clone)]
pub struct Or {
    pub(super) left: Rc<Node>,
    pub(super) right: Rc<Node>,
}

#[derive(Clone)]
pub struct Imply {
    pub(super) left: Rc<Node>,
    pub(super) right: Rc<Node>,
}

#[derive(Clone)]
pub struct Any {
    pub(super) var: Rc<Var>,
    pub(super) expr: Rc<Node>,
}

#[derive(Clone)]
pub struct All {
    pub(super) var: Rc<Var>,
    pub(super) expr: Rc<Node>,
}

#[derive(Clone)]
pub enum Node {
    Var(Var),
    Weight(Weight),
    Not(Not),
    And(And),
    Or(Or),
    Imply(Imply),
    Any(Any),
    All(All)
}



impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Node::Var(x) => x.fmt(f),
            Node::Weight(x) => x.fmt(f),
            Node::Not(x) => x.fmt(f),
            Node::And(x) => x.fmt(f),
            Node::Or(x) => x.fmt(f),
            Node::Imply(x) => x.fmt(f),
            Node::Any(x) => x.fmt(f),
            Node::All(x) => x.fmt(f),
        }
    }
}

impl Expr for Var {}
impl Expr for Weight {}
impl Expr for Not {}
impl Expr for And {}
impl Expr for Or {}
impl Expr for Imply {}
impl Expr for Any {}
impl Expr for All {}

impl Display for Var {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Display for Weight {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}\u{2219}{}", self.weight, self.child)
    }
}

impl Display for Not {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "/{}", self.child)
    }
}

impl Display for And {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}\u{2227}{})", self.left, self.right)
    }
}

impl Display for Or {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}\u{2228}{})", self.left, self.right)
    }
}

impl Display for Imply {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}=>{}", self.left, self.right)
    }
}

impl Display for Any {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "\u{2203}{}:{}", self.var, self.expr)
    }
}

impl Display for All {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "\u{2200}{}:{}", self.var, self.expr)
    }
}

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
        child: Rc::new(expr.into()),
    }
}

pub fn not<T: Into<Node>>(expr: T) -> Not {
    Not {
        child: Rc::new(expr.into()),
    }
}

pub fn and<T: Into<Node>, U: Into<Node>>(left: T, right: U) -> And {
    And {
        left: Rc::new(left.into()),
        right: Rc::new(right.into()),
    }
}

pub fn or<T: Into<Node>, U: Into<Node>>(left: T, right: U) -> Or {
    Or {
        left: Rc::new(left.into()),
        right: Rc::new(right.into()),
    }
}

pub fn imply<T: Into<Node>, U: Into<Node>>(left: T, right: U) -> Imply {
    Imply {
        left: Rc::new(left.into()),
        right: Rc::new(right.into()),
    }
}

pub fn any<T: Into<Node>>(var: Var, expr: T) -> Any {
    Any {
        var: Rc::new(var),
        expr: Rc::new(expr.into()),
    }
}

pub fn all<T: Into<Node>>(var: Var, expr: T) -> All {
    All {
        var: Rc::new(var),
        expr: Rc::new(expr.into()),
    }
}
