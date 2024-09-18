use std::collections::HashMap;

use crate::Tree;

use super::index::Indexing;

pub trait Mapping<IDX: Indexing = u32> {
    fn get_pred(&self, name: &String) -> IDX;
    fn get_var(&self, name: &String) -> IDX;
    fn get_vars(&self, names: &Vec<String>) -> Vec<IDX>;
}

pub(super) struct VerifiedMapping<IDX: Indexing> {
    pub vars: HashMap<String, IDX>,
    pub preds: HashMap<String, IDX>,
}

impl<IDX: Indexing> Mapping<IDX> for VerifiedMapping<IDX> {
    fn get_pred(&self, name: &String) -> IDX {
        *self
            .preds
            .get(name)
            .expect(format!("Unknown predicate {}", name.as_str()).as_str())
    }

    fn get_var(&self, name: &String) -> IDX {
        *self
            .vars
            .get(name)
            .expect(format!("Unknown variable {}", name.as_str()).as_str())
    }

    fn get_vars(&self, names: &Vec<String>) -> Vec<IDX> {
        names.iter().map(|x| self.get_var(x)).collect::<Vec<IDX>>()
    }
}

impl<IDX: Indexing> From<&Tree<IDX>> for VerifiedMapping<IDX> {
    fn from(tree: &Tree<IDX>) -> Self {
        VerifiedMapping {
            vars: tree
                .variables
                .iter()
                .enumerate()
                .map(|(idx, name)| (name.clone(), IDX::from(idx)))
                .collect(),
            preds: tree
                .predicates
                .iter()
                .enumerate()
                .map(|(idx, (name, _))| (name.clone(), IDX::from(idx)))
                .collect(),
        }
    }
}
