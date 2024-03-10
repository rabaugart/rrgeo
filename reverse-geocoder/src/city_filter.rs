use crate::Record;

/// A filter trait to search for cities based on different criteria
pub trait CityFilter {
    fn rec_match(&self,r:&Record) -> bool;
}