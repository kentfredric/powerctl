use crate::outpairs::OutPairs;
use std::{
    fs,
    io::Error,
    path::{Path, PathBuf},
};

pub(super) trait Output {
    fn heading(&self) -> String;
    fn fields(&self) -> &[&str];
    fn root(&self) -> PathBuf;
    fn extras(&self) -> Option<Vec<OutPairs>> { None }

    fn emit_output(&self) -> Result<OutPairs, Error> {
        let mut children = vec![
            OutPairs::Heading {
                name:   self.heading().to_owned(),
                source: self.root().to_owned(),
            },
            OutPairs::Children(
                self.fields()
                    .iter()
                    .map(|field| {
                        self.read_pair(&self.root().join(field), field)
                            .unwrap()
                    })
                    .collect::<Vec<OutPairs>>(),
            ),
        ];
        if let Some(entries) = self.extras() {
            for child in entries {
                children.push(child)
            }
        }

        Ok(OutPairs::Children(children))
    }

    fn read_pair(&self, path: &Path, name: &str) -> Result<OutPairs, Error> {
        match fs::read_to_string(path) {
            Ok(s) => Ok(OutPairs::Pair {
                key:   name.to_owned(),
                value: s.trim_end().to_owned(),
            }),
            Err(e) => Err(e),
        }
    }
}
