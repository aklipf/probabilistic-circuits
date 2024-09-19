use super::index::Indexing;
use super::node::{Node, Symbols};
use super::pool::Pool;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Builder<'a, IDX: Indexing, P: Pool<IDX = IDX>> {
    pub(super) allocator: &'a mut P,
}

impl<'a, IDX: Indexing, P: Pool<IDX = IDX>> Builder<'a, IDX, P> {
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
    pub fn all<F: Fn(&mut Self) -> IDX>(&mut self, var_id: IDX, inner: F) -> IDX {
        self.push_unary(Symbols::All { var_id: var_id }, inner)
    }

    #[inline]
    pub fn any<F: Fn(&mut Self) -> IDX>(&mut self, var_id: IDX, inner: F) -> IDX {
        self.push_unary(Symbols::Any { var_id: var_id }, inner)
    }

    #[inline]
    pub fn connect(&mut self, node_id: IDX) -> IDX {
        node_id
    }
}
