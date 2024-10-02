use crate::{
    logic::fragment::{Fragment, Symbols},
    tree::tree::Tree,
};

use super::{index::Indexing, node::Node};

pub trait Mapping {
    type IDX;
    fn add_named(&mut self, name: &String) -> Self::IDX;
    fn add_anon(&mut self) -> Self::IDX;
    fn get_id(&self, name: &String) -> Option<Self::IDX>;
    fn get_named(&self, id: Self::IDX) -> Option<&String>;
}

impl<S: Symbols, IDX: Indexing, const MAX_CHILDS: usize> Mapping for Tree<S, IDX, MAX_CHILDS>
where
    Node<IDX, S, MAX_CHILDS>: Fragment<IDX>,
{
    type IDX = IDX;
    fn add_named(&mut self, name: &String) -> IDX {
        if let Some(id) = self.get_id(name) {
            id
        } else {
            let id = IDX::from(self.named.len());
            self.named.push(Some(name.to_owned()));
            self.mapping.insert(name.to_owned(), id);
            id
        }
    }

    fn add_anon(&mut self) -> IDX {
        let id = IDX::from(self.named.len());
        self.named.push(None);
        id
    }

    fn get_id(&self, name: &String) -> Option<IDX> {
        self.mapping.get(name).cloned()
    }

    fn get_named(&self, id: IDX) -> Option<&String> {
        if id.addr() < self.named.len() {
            self.named[id.addr()].as_ref()
        } else {
            None
        }
    }
}
