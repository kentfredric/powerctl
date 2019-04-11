use crate::outpairs::OutPairs;
use std::{
    fs,
    io::Error,
    path::{Path, PathBuf},
};

pub(super) trait Output {
    fn heading(&self) -> String;
    fn fields(&self) -> Vec<String>;
    fn root(&self) -> PathBuf;
    fn extras(&self) -> Option<Vec<OutPairs>> { None }

    fn field_values(&self) -> Result<Vec<OutPairs>, Error> {
        let mut out = Vec::new();
        for field in self.fields() {
            out.push(self.read_pair(&self.root().join(&field), &field)?)
        }
        Ok(out)
    }

    fn children(&self) -> Result<Vec<OutPairs>, Error> {
        let mut children = Vec::new();
        children.push(OutPairs::Heading {
            name:   self.heading().to_owned(),
            source: self.root().to_owned(),
        });
        children.push(OutPairs::Children(self.field_values()?));
        if let Some(entries) = self.extras() {
            for child in entries {
                children.push(child)
            }
        }
        Ok(children)
    }

    fn emit_output(&self) -> Result<OutPairs, Error> {
        Ok(OutPairs::Children(self.children()?))
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
