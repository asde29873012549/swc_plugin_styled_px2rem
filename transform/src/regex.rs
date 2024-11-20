use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref PX_REGEX: Regex = Regex::new(r"(\d*\.?\d+)px").unwrap();
}