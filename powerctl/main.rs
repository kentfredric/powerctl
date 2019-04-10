#![feature(termination_trait_lib)]

use libpowerctl::App;
use std::process::Termination;

fn main() -> impl Termination { App::new().run() }
