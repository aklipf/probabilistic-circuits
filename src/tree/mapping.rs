use crate::Tree;

use super::index::Indexing;

pub trait Mapping<IDX: Indexing = u32> {
    fn add_named(&mut self, name: &String) -> IDX;
    fn add_anon(&mut self) -> IDX;
    fn get_id(&self, name: &String) -> Option<IDX>;
    fn get_named(&self, id: IDX) -> Option<&String>;
}

impl<IDX: Indexing> Mapping<IDX> for Tree<IDX> {
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
