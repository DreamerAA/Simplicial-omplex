use uuid::Uuid;
pub trait IComplex{
    fn cells(&self) -> &[Uuid];
    fn dimension(&self, id:Uuid) -> u32;
    fn facets(&self, id:Uuid) -> &[Uuid];
    fn cofacets(&self, id:Uuid) -> &[Uuid];

    fn delete_cells(&self, ids: Vec<Uuid>);
    fn add_cell(&self, facets: Vec<Uuid>);

    fn is_valid(&self) -> bool{
        true
    }
  }
  
// pub trait MorseFunction{
// fn value(&self, id:i64) -> f32;
// }

// pub trait VectorField{
// fn vector(&self, id:i64) -> Pair<i64,i64>;
// }
