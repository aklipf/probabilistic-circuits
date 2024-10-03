pub trait Mapping {
    type IDX;
    fn add_named(&mut self, name: &String) -> Self::IDX;
    fn add_anon(&mut self) -> Self::IDX;
    fn get_id(&self, name: &String) -> Option<Self::IDX>;
    fn get_named(&self, id: Self::IDX) -> Option<&String>;
}
