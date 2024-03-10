use crate::{city_filter::CityFilter, matchers::Matcher, Record};

/// Match the name field of the record
#[derive(Clone)]
pub struct Name<T: Matcher> {
    m: T,
}

impl<T: Matcher+Clone> Name<T> {
    pub fn new(name: &str) -> Self {
        Self { m: T::new(name) }
    }
}

impl<T: Matcher+Clone> CityFilter for Name<T> {
    fn rec_match(&self, r: &Record) -> bool {
        self.m.match_field(&r.name)
    }
}

/// Match the admin1 field of the record
#[derive(Clone)]
pub struct Admin1<T: Matcher+Clone> {
    m: T,
}

impl<T: Matcher+Clone> Admin1<T> {
    pub fn new(admin1: &str) -> Self {
        Self { m: T::new(admin1) }
    }
}

impl<T: Matcher+Clone> CityFilter for Admin1<T> {
    fn rec_match(&self, r: &Record) -> bool {
        self.m.match_field(&r.admin1)
    }
}

/// Match the admin2 field of the record
#[derive(Clone)]
pub struct Admin2<T: Matcher+Clone> {
    m: T,
}

impl<T: Matcher+Clone> Admin2<T> {
    pub fn new(admin2: &str) -> Self {
        Self { m: T::new(admin2) }
    }
}

impl<T: Matcher+Clone> CityFilter for Admin2<T> {
    fn rec_match(&self, r: &Record) -> bool {
        self.m.match_field(&r.admin2)
    }
}

/// Match the country of the record exactly.
#[derive(Clone)]
pub struct Country {
    country_code: String,
}

impl Country {
    /// Create country filter with a ISO 3166 country code, e.g. "US"
    pub fn new(country_code: &str) -> Self {
        Self {
            country_code: country_code.to_owned(),
        }
    }
}

impl CityFilter for Country {
    fn rec_match(&self, r: &Record) -> bool {
        r.cc == self.country_code
    }
}

#[derive(Clone)]
/// Filtor for T and U
pub struct And<T:CityFilter,U:CityFilter> {
    fa : T,
    fb : U
}

impl<T:CityFilter,U:CityFilter> And<T,U> {
    pub fn new( fa: T, fb: U ) -> Self {
        Self { fa, fb }
    }
}

impl<T:CityFilter,U:CityFilter> CityFilter for And<T,U> {
    fn rec_match(&self, r: &Record) -> bool {
        self.fa.rec_match(r) && self.fb.rec_match(r)
    }
}

#[cfg(test)]
mod filter_tests {
    use crate::{
        filters::{Admin1, Admin2, Country, Name, And},
        matchers::{ExactMatcher, IContainsMatcher},
    };

    #[test]
    fn and() {
        let rc = crate::ReverseGeocoder::new();
        let fa = Name::<ExactMatcher>::new("Paris");
        let fb = Country::new("FR");
        let f = And::new(fa,fb);
        let ci = rc.search_city(f);
        // Only one Paris in France
        assert_eq!(ci.len(),1);
    }

    #[test]
    fn name() {
        let rc = crate::ReverseGeocoder::new();
        let ci = rc.search_city(Name::<ExactMatcher>::new("Chicago"));
        assert_eq!(ci.len(), 1);
        let ([lat, lon], rec) = ci[0];
        assert_eq!(rec.name, "Chicago");
        assert_eq!(rec.cc, "US");
        assert!(*lat > 41.8 && *lat < 41.9);
        assert!(*lon > -87.7 && *lon < -87.6);
    }

    #[test]
    fn name_icontains() {
        let rc = crate::ReverseGeocoder::new();
        let ci = rc.search_city(Name::<IContainsMatcher>::new("roNTig"));
        assert_eq!(ci.len(), 1);
        let ([_lat, _lon], rec) = ci[0];
        assert_eq!(rec.name, "Frontignan");
        assert_eq!(rec.cc, "FR");
    }

    #[test]
    fn admin1() {
        let rc = crate::ReverseGeocoder::new();
        let ci = rc.search_city(Admin1::<ExactMatcher>::new("Bremen"));
        assert_eq!(ci.len(), 2);
        let ([lat, lon], rec) = ci[0];
        assert_eq!(rec.admin1, "Bremen");
        assert_eq!(rec.cc, "DE");
        assert!(*lat > 53.0 && *lat < 54.0);
        assert!(*lon > 8.0 && *lon < 9.0);
    }

    #[test]
    fn admin2() {
        let rc = crate::ReverseGeocoder::new();
        let ci = rc.search_city(Admin2::<ExactMatcher>::new("Goteborg"));
        assert_eq!(ci.len(), 11);
        for cc in ci {
            let ([lat, lon], rec) = cc;
            assert_eq!(rec.admin2, "Goteborg");
            assert_eq!(rec.cc, "SE");
            assert!(*lat > 57.0 && *lat < 58.0);
            assert!(*lon > 11.0 && *lon < 13.0);
        }
    }

    #[test]
    fn country() {
        let rc = crate::ReverseGeocoder::new();
        let ci = rc.search_city(Country::new("MC"));
        assert_eq!(ci.len(), 6);
    }
}
