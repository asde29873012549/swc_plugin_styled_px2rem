use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[derive(Clone)]  // Add this line
pub struct Config {
    pub root_value: f64,
    pub unit_precision: usize,
    pub min_pixel_value: f64,
    pub multiplier: f64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            root_value: 3.75,
            unit_precision: 3,
            min_pixel_value: 0.0,
            multiplier: 1.0,
        }
    }
}