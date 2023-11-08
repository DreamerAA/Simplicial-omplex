pub trait IComplex{
    pub fn cells(&self) -> vec<i64>&;
    pub fn dimension(&self, id:i64) -> ui32;
    pub fn facets(&self, id:i64) -> vect<i64>&;
    pub fn cofacets(&self, id:i64) -> vect<i64>&;
    pub fn deleteCells(&self, ids:vec<i64>) -> vect<i64>&;
    pub fn addCell(&self, facets: vec<i64>) -> void
    pub fn fillDataStructure(&self) -> void;
    pub fn isValid(&self) -> bool{
       ....
    }
    fn new() -> IComplex{
       ...
       complex.fillDataStructure();
       complex.isValid();
       return complex;
    }
  }
  
pub trait MorseFunction{
fn value(&self, id:i64) -> f32;
}

pub trait VectorField{
fn vector(&self, id:i64) -> Pair<i64,i64>;
}

pub struct Complex{
pub cells: ...,
pub graph: Network,
}

impl IComplex for Complex{
pub fn new() -> Complex{
    
}
}


#[cfg(test)]
mod test_complex {
    use crate::complex;
    #[test]
    fn it_works() {
        assert_eq!(complex::add(2,2), 4);
    }
}