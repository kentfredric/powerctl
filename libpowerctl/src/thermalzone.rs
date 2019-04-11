use crate::output::Output;
use std::path::{Path, PathBuf};

pub(super) struct ThermalZone {
    id: usize,
}

impl ThermalZone {
    pub(super) fn new(id: usize) -> Self { Self { id } }
}
impl Output for ThermalZone {
    fn heading(&self) -> String {
        format!("Thermal Zone #{id}", id = &self.id)
    }

    fn root(&self) -> PathBuf {
        Path::new(&format!(
            "/sys/class/thermal/thermal_zone{id}/",
            id = &self.id
        ))
        .to_path_buf()
    }

    fn fields(&self) -> Vec<String> {
        vec![
            "type",
            "available_policies",
            "integral_cutoff",
            "k_d",
            "k_i",
            "k_po",
            "k_pu",
            "offset",
            "policy",
            "slope",
            "sustainable_power",
            "temp",
            "trip_point_0_temp",
            "trip_point_0_type",
            "trip_point_1_temp",
            "trip_point_1_type",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect()
    }
}
