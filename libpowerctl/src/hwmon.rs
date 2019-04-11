use crate::output::Output;
use std::path::{Path, PathBuf};

pub(super) struct HwMon {
    id: usize,
}

impl HwMon {
    pub(super) fn new(id: usize) -> Self { Self { id } }
}

impl Output for HwMon {
    fn heading(&self) -> String {
        format!("Hardware Monitor #{id}", id = &self.id)
    }

    fn root(&self) -> PathBuf {
        Path::new(&format!("/sys/class/hwmon/hwmon{id}/", id = &self.id))
            .to_path_buf()
    }

    fn fields(&self) -> Vec<String> {
        vec![
            "name",
            "temp1_crit",
            "temp1_crit_alarm",
            "temp1_input",
            "temp1_label",
            "temp1_max",
            "temp2_crit",
            "temp2_crit_alarm",
            "temp2_input",
            "temp2_label",
            "temp2_max",
            "temp3_crit",
            "temp3_crit_alarm",
            "temp3_input",
            "temp3_label",
            "temp3_max",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect()
    }
}
