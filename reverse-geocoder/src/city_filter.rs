use crate::Record;


pub trait CityFilter {
    fn rec_match(&self,r:&Record) -> bool;
}