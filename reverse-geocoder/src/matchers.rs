use regex::{Regex, RegexBuilder};

/// How to match a field against a pattern
pub trait Matcher {
    fn match_field(&self, rec_field: &str) -> bool;
    fn new(pat: &str) -> Self;
}

/// The field matches the pattern exactly
#[derive(Clone)]
pub struct ExactMatcher {
    pattern: String,
}

impl Matcher for ExactMatcher {
    fn match_field(&self, rec_field: &str) -> bool {
        self.pattern == rec_field
    }
    fn new(pat: &str) -> Self {
        ExactMatcher {
            pattern: pat.to_owned(),
        }
    }
}

/// The contains the pattern case insensitively
#[derive(Clone)]
pub struct IContainsMatcher {
    pattern: Regex,
}

impl Matcher for IContainsMatcher {
    fn match_field(&self, rec_field: &str) -> bool {
        self.pattern.is_match(rec_field)
    }
    fn new(pat: &str) -> Self {
        IContainsMatcher {
            pattern: RegexBuilder::new(pat)
                .case_insensitive(true)
                .build()
                .unwrap(),
        }
    }
}
