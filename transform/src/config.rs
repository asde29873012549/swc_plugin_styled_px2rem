use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    #[serde(default = "default_root_value")]
    pub root_value: f64,
    #[serde(default = "default_unit_precision")]
    pub unit_precision: usize,
    #[serde(default = "default_min_pixel_value")]
    pub min_pixel_value: f64,
    #[serde(default = "default_multiplier")]
    pub multiplier: f64,

    #[serde(default = "default_tags")]
    pub tags: Vec<String>,
    #[serde(default)]
    pub media_query: bool,
    #[serde(default = "default_transform_interpolation")]
    pub transform_interpolation: bool,
    #[serde(default)]
    pub transform_jsx_attributes: bool
}

fn default_root_value() -> f64 { 3.75 }
fn default_unit_precision() -> usize { 3 }
fn default_min_pixel_value() -> f64 { 0.0 }
fn default_multiplier() -> f64 { 1.0 }
fn default_transform_interpolation() -> bool { true }
fn default_tags() -> Vec<String> {
    vec![
        "styled".to_string(),
        "css".to_string(),
        "createGlobalStyle".to_string(),
        "keyframes".to_string(),
    ]
}

impl Default for Config {
    fn default() -> Self {
        Self {
            root_value: default_root_value(),
            unit_precision: default_unit_precision(),
            min_pixel_value: default_min_pixel_value(),
            multiplier: default_multiplier(),

            tags: default_tags(),
            media_query: false,
            transform_interpolation: default_transform_interpolation(),
            transform_jsx_attributes: false,
        }
    }
}