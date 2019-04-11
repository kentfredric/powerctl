#![feature(try_trait)]

mod app;
mod coolingdevice;
mod hwmon;
mod maybe;
mod outpairs;
mod output;
mod thermalzone;

pub use app::App;
pub use maybe::Maybe::{self, *};
