use super::index::Indexing;
use super::mapping::AddPredicate;
use super::node::{Node, Symbols};
use super::pool::Pool;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Builder<'a, IDX: Indexing, P: Pool<IDX = IDX> + AddPredicate<IDX>> {
    pub(super) allocator: &'a mut P,
}

#[macro_export]
macro_rules! var {
    ($id: expr) => {
        |builder| builder.var($id)
    };
}

#[macro_export]
macro_rules! not {
    ($e: expr) => {
        |builder| builder.not($e)
    };
}

#[macro_export]
macro_rules! and {
    ($left: expr,$right: expr) => {
        |builder| builder.and($left, $right)
    };
}

#[macro_export]
macro_rules! or {
    ($left: expr,$right: expr) => {
        |builder| builder.or($left, $right)
    };
}

#[macro_export]
macro_rules! conjunction {
    ($e: expr) => {
        $e
    };
    ($e:expr,$($es:expr),+) => {{
        and!($e, conjunction! ($($es),+))
    }};
}

#[macro_export]
macro_rules! disjunction {
    ($e: expr) => {
        $e
    };
    ($e:expr,$($es:expr),+) => {{
        or!($e, disjunction! ($($es),+))
    }};
}

#[macro_export]
macro_rules! pred {
    ($id: expr,$vars: expr) => {
        |builder| builder.pred($id, $vars)
    };
    ($id: expr) => {
        |builder| builder.pred($id, &[])
    };
}

#[macro_export]
macro_rules! every {
    ($id: expr,$e: expr) => {
        |builder| builder.every($id, $e)
    };
}

#[macro_export]
macro_rules! every_n {
    ($id: expr,$e: expr) => {
        |builder| builder.every_n($id, $e)
    };
}

#[macro_export]
macro_rules! exist {
    ($id: expr,$e: expr) => {
        |builder| builder.exist($id, $e)
    };
}

#[macro_export]
macro_rules! exist_n {
    ($id: expr,$e: expr) => {
        |builder| builder.exist_n($id, $e)
    };
}

#[macro_export]
macro_rules! connect {
    ($e: expr) => {
        |builder| builder.connect($e)
    };
}

#[macro_export]
macro_rules! copy {
    ($e: expr) => {
        |builder| builder.copy($e)
    };
}

impl<'a, IDX: Indexing, P: Pool<IDX = IDX> + AddPredicate<IDX>> Builder<'a, IDX, P> {
    #[inline]
    fn push(&mut self, symbol: Symbols<IDX>) -> IDX {
        self.allocator.push(Node {
            parent: IDX::NONE,
            childs: [IDX::NONE, IDX::NONE],
            symbol: symbol,
        })
    }

    #[inline]
    fn push_unary<F: Fn(&mut Self) -> IDX>(&mut self, symbol: Symbols<IDX>, inner: F) -> IDX {
        let inner_idx = inner(self);

        let idx = self.allocator.push(Node {
            parent: IDX::NONE,
            childs: [inner_idx, IDX::NONE],
            symbol: symbol,
        });

        self.allocator[inner_idx].parent = idx;

        idx
    }

    #[inline]
    fn push_binary<F: Fn(&mut Self) -> IDX, G: Fn(&mut Self) -> IDX>(
        &mut self,
        symbol: Symbols<IDX>,
        left: F,
        right: G,
    ) -> IDX {
        let left_idx = left(self);
        let right_idx = right(self);

        let idx = self.allocator.push(Node {
            parent: IDX::NONE,
            childs: [left_idx, right_idx],
            symbol: symbol,
        });

        self.allocator[left_idx].parent = idx;
        self.allocator[right_idx].parent = idx;

        idx
    }

    #[inline]
    pub fn var(&mut self, var_id: IDX) -> IDX {
        self.push(Symbols::Variable { var_id: var_id })
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
    fn pred_arg(&mut self, vars_id: &[IDX]) -> IDX {
        if vars_id.len() == 1 {
            return self.push(Symbols::Variable { var_id: vars_id[0] });
        }

        self.push_unary(Symbols::Variable { var_id: vars_id[0] }, |builder| {
            builder.pred_arg(&vars_id[1..])
        })
    }

    #[inline]
    pub fn pred(&mut self, pred_id: IDX, vars_id: &[IDX]) -> IDX {
        if vars_id.len() == 0 {
            return self.push(Symbols::Predicate { pred_id: pred_id });
        }

        self.push_unary(Symbols::Predicate { pred_id: pred_id }, |builder| {
            builder.pred_arg(vars_id)
        })
    }

    #[inline]
    pub fn every<F: Fn(&mut Self) -> IDX>(&mut self, var_id: IDX, inner: F) -> IDX {
        self.push_unary(Symbols::Every { var_id: var_id }, inner)
    }

    #[inline]
    pub fn exist<F: Fn(&mut Self) -> IDX>(&mut self, var_id: IDX, inner: F) -> IDX {
        self.push_unary(Symbols::Exist { var_id: var_id }, inner)
    }

    #[inline]
    pub fn every_n<F: Fn(&mut Self) -> IDX>(&mut self, var_id: &[IDX], inner: F) -> IDX {
        let next_idx = inner(self);
        let mut idx = IDX::NONE;

        for &id in var_id.iter().rev() {
            idx = self.allocator.push(Node {
                parent: IDX::NONE,
                childs: [next_idx, IDX::NONE],
                symbol: Symbols::Every { var_id: id },
            });
            self.allocator[next_idx].parent = idx;
        }

        idx
    }

    #[inline]
    pub fn exist_n<F: Fn(&mut Self) -> IDX>(&mut self, var_id: &[IDX], inner: F) -> IDX {
        let next_idx = inner(self);
        let mut idx = IDX::NONE;

        for &id in var_id.iter().rev() {
            idx = self.allocator.push(Node {
                parent: IDX::NONE,
                childs: [next_idx, IDX::NONE],
                symbol: Symbols::Exist { var_id: id },
            });
            self.allocator[next_idx].parent = idx;
        }

        idx
    }

    #[inline]
    pub fn connect(&mut self, node_id: IDX) -> IDX {
        node_id
    }

    #[inline]
    pub fn copy(&mut self, node_id: IDX) -> IDX {
        let node = self.allocator[node_id];
        let idx = self.allocator.push(node);
        for i in 0..2 {
            if node.childs[i].is_addr() {
                let res_idx = self.copy(node.childs[i]);
                self.allocator[res_idx].parent = idx;
            }
        }
        idx
    }
}

impl<'a, IDX: Indexing, P: Pool<IDX = IDX> + AddPredicate<IDX>> AddPredicate<IDX>
    for Builder<'a, IDX, P>
{
    #[inline]
    fn add_anon_predicate(&mut self, n: usize) -> IDX {
        self.allocator.add_anon_predicate(n)
    }
}
