
use crate::{city_filter::CityFilter, Record};

pub struct Name {
    name : String,
}

impl CityFilter for Name {
    fn rec_match( &self, r: &Record ) -> bool {
        r.name == self.name
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
    use crate::filters::{Country, Name};

    #[test]
    fn name() {
        let rc = crate::ReverseGeocoder::new();
        let ci = rc.search_city( &Name{name:"Helmstedt".to_owned()});
        assert_eq!(ci.len(),1);
        let ([lat,lon],rec) = ci[0];
        assert_eq!(rec.name,"Helmstedt");
        assert_eq!(rec.cc,"DE");
        assert!( *lat > 52.2 && *lat<52.3);
        assert!( *lon > 11.0 && *lon<11.1);
    }

    #[test]
    fn country() {
        let rc = crate::ReverseGeocoder::new();
        let ci = rc.search_city( &Country{name:"MC".to_owned()});
        assert_eq!(ci.len(),6);
    }
}