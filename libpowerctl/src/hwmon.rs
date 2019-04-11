use crate::{outpairs::OutPairs, output::Output, Maybe};
use std::{
    io::Error,
    path::{Path, PathBuf},
};

pub(super) struct HwMon {
    id: usize,
}

impl HwMon {
    pub(super) fn new(id: usize) -> Self { Self { id } }

    fn temp(&self, id: usize) -> Temp<'_> { Temp { id, mon: self } }
}

struct Temp<'a> {
    id:  usize,
    mon: &'a HwMon,
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
        vec!["name"].iter().map(|x| x.to_string()).collect()
    }

    fn extras(&self) -> Maybe<Vec<OutPairs>, Error> {
        Maybe::Some(vec![
            self.temp(1).emit_output()?,
            self.temp(2).emit_output()?,
            self.temp(3).emit_output()?,
        ])
    }
}

impl Output for Temp<'_> {
    fn heading(&self) -> String {
        format!("Temperature #{id}", id = &self.id)
    }

    fn root(&self) -> PathBuf { self.mon.root() }

    fn root_string(&self) -> String {
        match self.root().to_str() {
            Some(s) => format!("\"{}temp{}_\"*", s, &self.id),
            None => format!("IO {:?}", self.root()),
        }
    }

    fn fields(&self) -> Vec<String> {
        vec![
            format!("temp{id}_crit", id = &self.id),
            format!("temp{id}_crit_alarm", id = &self.id),
            format!("temp{id}_input", id = &self.id),
            format!("temp{id}_label", id = &self.id),
            format!("temp{id}_max", id = &self.id),
        ]
    }

    fn field_rename(&self, name: &str) -> String {
        let prefix = format!("temp{id}_", id = &self.id);
        name.trim_start_matches(&prefix).to_owned()
    }
}
