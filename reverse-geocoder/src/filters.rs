
use crate::{city_filter::CityFilter, matchers::Matcher, Record};

/// Match the name field of the record
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

/// Match the country of the record exactly.
pub struct Country {
    country_code : String,
}

impl Country {
    /// Create country filter with a ISO 3166 country code, e.g. "US"
    pub fn new( country_code:&str ) -> Self {
        Self { country_code: country_code.to_owned() }
    }
}

impl CityFilter for Country {
    fn rec_match( &self, r: &Record ) -> bool {
        r.cc == self.country_code
    }
}

#[cfg(test)]
mod filter_tests {
    use crate::{filters::{Country, Name}, matchers::{ExactMatcher, IContainsMatcher}};

    #[test]
    fn name() {
        let rc = crate::ReverseGeocoder::new();
        let ci = rc.search_city( &Name::<ExactMatcher>::new("Chicago"));
        assert_eq!(ci.len(),1);
        let ([lat,lon],rec) = ci[0];
        assert_eq!(rec.name,"Chicago");
        assert_eq!(rec.cc,"US");
        assert!( *lat > 41.8 && *lat<41.9);
        assert!( *lon > -87.7 && *lon< -87.6);
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
        let ci = rc.search_city( &Country::new("MC"));
        assert_eq!(ci.len(),6);
    }
}