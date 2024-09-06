use super::tree::*;

use std::fmt::Display;
use std::*;

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Node::Var(x) => x.fmt(f),
            Node::Weight(x) => x.fmt(f),
            Node::Not(x) => x.fmt(f),
            Node::And(x) => x.fmt(f),
            Node::Or(x) => x.fmt(f),
            Node::Imply(x) => x.fmt(f),
            Node::Equivalent(x) => x.fmt(f),
            Node::Predicate(x) => x.fmt(f),
            Node::Any(x) => x.fmt(f),
            Node::All(x) => x.fmt(f),
        }
    }
}

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
        write!(f, "\u{00AC}{}", self.child)
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
        write!(f, "{}\u{21D2}{}", self.left, self.right)
    }
}

impl Display for Equivalent {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}\u{21D4}{}", self.left, self.right)
    }
}

impl Display for Predicate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match write!(f, "{}(", self.name) {
            Ok(_) => {}
            Err(e) => return Err(e),
        };

        if self.vars.len() > 0 {
            match self.vars[0].fmt(f) {
                Ok(_) => {}
                Err(e) => return Err(e),
            };
        }

        for var in &self.vars[1..] {
            match write!(f, ", {}", var) {
                Ok(_) => {}
                Err(e) => return Err(e),
            };
        }

        write!(f, ")")
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
