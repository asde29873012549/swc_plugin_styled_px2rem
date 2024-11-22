#![deny(unused)]

use std::{fs::read_to_string, path::PathBuf};
use styled_components_px2rem::{config::Config, visitor::PxToRem};
use swc_ecma_parser::{EsSyntax, Syntax};
use swc_core::{
    common::Mark,
    ecma::visit::visit_mut_pass,
    ecma::transforms::{
        base::resolver,
        testing::{test_fixture, FixtureTestConfig},
    },
};


#[testing::fixture("tests/fixtures/**/**/input.js")]
fn fixture(input: PathBuf) {
    let dir = input.parent().unwrap();

    let config = dir
        .join("config.json")
        .exists()
        .then(|| {
            let config_str = read_to_string(dir.join("config.json"))
                .expect("failed to read config.json");
            println!("---- Config -----\n{}", config_str);
            serde_json::from_str(&config_str).unwrap()
        })
        .unwrap_or_else(|| Config::default());

    test_fixture(
        Syntax::Es(EsSyntax {
            jsx: true,
            ..Default::default()
        }),
        &|_| {
            (
                resolver(Mark::new(), Mark::new(), false),
                visit_mut_pass(PxToRem::new(config.clone())),
            )
        },
        &input,
        &dir.join("output.js"),
        FixtureTestConfig {
            module: Some(true),
            ..Default::default()
        },
    )
}