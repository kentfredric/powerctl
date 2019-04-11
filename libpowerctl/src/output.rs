use crate::{outpairs::OutPairs, Maybe};

use std::{
    fs,
    io::Error,
    path::{Path, PathBuf},
};

pub(super) trait Output {
    fn heading(&self) -> String;
    fn fields(&self) -> Vec<String>;
    fn root(&self) -> PathBuf;
    fn extras(&self) -> Maybe<Vec<OutPairs>, Error> { Maybe::None }

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
            source: self.root_string(),
        });
        children.push(OutPairs::Children(self.field_values()?));
        match self.extras() {
            Maybe::Err(e) => return Err(e),
            Maybe::Some(entries) => {
                for child in entries {
                    children.push(child)
                }
            },
            Maybe::None => (),
        }
        Ok(children)
    }

    fn field_rename(&self, field: &str) -> String { field.to_owned() }
    fn root_string(&self) -> String { format!("{:?}", self.root()) }
    fn emit_output(&self) -> Result<OutPairs, Error> {
        Ok(OutPairs::Children(self.children()?))
    }

    fn read_pair(&self, path: &Path, name: &str) -> Result<OutPairs, Error> {
        match fs::read_to_string(path) {
            Ok(s) => Ok(OutPairs::Pair {
                key:   self.field_rename(name),
                value: s.trim_end().to_owned(),
            }),
            Err(e) => Err(e),
        }
    }
}
