use crate::output::Output;
use std::path::{Path, PathBuf};

pub(super) struct CoolingDevice {
    id: usize,
}

impl CoolingDevice {
    pub(super) fn new(id: usize) -> Self { Self { id } }
}

impl Output for CoolingDevice {
    fn heading(&self) -> String {
        format!("Cooling Device #{id}", id = &self.id)
    }

    fn root(&self) -> PathBuf {
        Path::new(&format!(
            "/sys/class/thermal/cooling_device{id}/",
            id = &self.id
        ))
        .to_path_buf()
    }

    fn fields(&self) -> Vec<String> {
        vec!["type", "cur_state", "max_state"]
            .iter()
            .map(|x| x.to_string())
            .collect()
    }
}
