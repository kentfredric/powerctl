#[derive(Debug)]
pub struct App {}

use crate::{
    coolingdevice::CoolingDevice, hwmon::HwMon, output::Output,
    thermalzone::ThermalZone,
};
use std::io::{self, Error, Write};

impl App {
    pub fn new() -> Self { Default::default() }

    pub fn run(&self) -> Result<(), Error> {
        let mut out = io::stdout();
        writeln!(out, "Cooling Devices:")?;
        for i in 0..=4 {
            CoolingDevice::new(i).emit_output()?.write_into(&mut out)?;
        }
        writeln!(out, "Thermal Zones:")?;
        for i in 0..=0 {
            ThermalZone::new(i).emit_output()?.write_into(&mut out)?;
        }
        writeln!(out, "Hardware Monitors:")?;
        for i in 0..=0 {
            HwMon::new(i).emit_output()?.write_into(&mut out)?;
        }
        Ok(())
    }
}
impl Default for App {
    fn default() -> Self { Self {} }
}
