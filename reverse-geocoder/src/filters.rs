
use crate::{city_filter::CityFilter, matchers::Matcher, Record};

pub struct Name<T:Matcher> {
    m : T,
}

impl<T:Matcher> Name<T> {
    pub fn new(name:&str) -> Self {
        Self{ m: T::new(name) }
    }
}

impl<T:Matcher> CityFilter for Name<T> {
    fn rec_match( &self, r: &Record ) -> bool {
        self.m.match_field( &r.name)
    }
}

pub struct Country {
    name : String,
}

impl CityFilter for Country {
    fn rec_match( &self, r: &Record ) -> bool {
        r.cc == self.name
    }
}

#[cfg(test)]
mod filter_tests {
    use crate::{filters::{Country, Name}, matchers::{ExactMatcher, IContainsMatcher}};

    #[test]
    fn name() {
        let rc = crate::ReverseGeocoder::new();
        let ci = rc.search_city( &Name::<ExactMatcher>::new("Helmstedt"));
        assert_eq!(ci.len(),1);
        let ([lat,lon],rec) = ci[0];
        assert_eq!(rec.name,"Helmstedt");
        assert_eq!(rec.cc,"DE");
        assert!( *lat > 52.2 && *lat<52.3);
        assert!( *lon > 11.0 && *lon<11.1);
    }

    #[test]
    fn name_icontains() {
        let rc = crate::ReverseGeocoder::new();
        let ci = rc.search_city( &Name::<IContainsMatcher>::new("roNTig"));
        assert_eq!(ci.len(),1);
        let ([_lat,_lon],rec) = ci[0];
        assert_eq!(rec.name,"Frontignan");
        assert_eq!(rec.cc,"FR");
    }

    #[test]
    fn country() {
        let rc = crate::ReverseGeocoder::new();
        let ci = rc.search_city( &Country{name:"MC".to_owned()});
        assert_eq!(ci.len(),6);
    }
}