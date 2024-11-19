use swc_core::ecma::{
    ast::*,
    atoms::JsWord,
    visit::{ VisitMut, VisitMutWith },
};
use swc_core::common::DUMMY_SP;
use crate::{
    config::Config,
    visitor::PxToRem,
    regex::PX_REGEX,
    helpers::is_styled_components,
};

impl PxToRem {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    fn process_tagged_tpl(&mut self, n: &mut TaggedTpl) {
        if is_styled_components(n) {
            if n.tpl.exprs.is_empty() {
                let css = n.tpl.quasis.iter().map(|q| &*q.raw).collect::<String>();
                let transformed_css = self.transform_css(&css);
                n.tpl.quasis = vec![TplElement {
                    cooked: Some(JsWord::from(transformed_css.as_str())),
                    raw: JsWord::from(transformed_css.as_str()),
                    span: DUMMY_SP,
                    tail: true,
                }];
                n.tpl.exprs.clear();
            }
        } else {
            n.visit_mut_children_with(self);
        }
    }

    fn transform_css(&self, css: &str) -> String {
        let root_font_size = self.config.root_value;
        let unit_precision = self.config.unit_precision;
        
        let result = PX_REGEX.replace_all(css, |caps: &regex::Captures| {
            let px_value: f64 = caps[1].parse().unwrap_or(0.0);
            if px_value < self.config.min_pixel_value {
                return format!("{}px", px_value);
            }
            let rem_value = (px_value * self.config.multiplier) / root_font_size;
            format!("{:.precision$}rem", rem_value, precision = unit_precision)
        });

        result.to_string()
    }
}

impl VisitMut for PxToRem {
    fn visit_mut_tagged_tpl(&mut self, n: &mut TaggedTpl) {
        self.process_tagged_tpl(n);
    }
}