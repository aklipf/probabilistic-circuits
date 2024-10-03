use super::index::Indexing;

pub trait Mapping<I: Indexing> {
    fn add_named(&mut self, name: &String) -> I;
    fn add_anon(&mut self) -> I;
    fn get_id(&self, name: &String) -> Option<I>;
    fn get_named(&self, id: I) -> Option<&String>;
}
