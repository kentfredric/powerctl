#[derive(Debug)]
pub(super) enum OutPairs {
    Pair { key: String, value: String },
    Heading { name: String, source: PathBuf },
    Children(Vec<OutPairs>),
}

use std::{
    io::{Error, Write},
    path::PathBuf,
};
use OutPairs::*;

impl OutPairs {
    fn lead_width(&self) -> usize {
        match self {
            Heading { name: k, .. } => k.len(),
            Pair { key: k, .. } => k.len(),
            Children(v) => v.iter().map(Self::lead_width).max().unwrap_or(0),
        }
    }

    fn __write_into<W>(
        &self, buf: &mut W, pad: &str, width: usize,
    ) -> Result<(), Error>
    where
        W: Write,
    {
        match self {
            Heading { name, source } => writeln!(
                buf,
                "{pad}{name} (via {source:?})",
                pad = pad,
                name = name,
                source = source
            ),
            Pair { key: k, value: v } => writeln!(
                buf,
                "{pad}| {key:>kwidth$}: {value}",
                pad = pad,
                key = k,
                kwidth = width,
                value = v
            ),
            Children(v) => {
                for child in v {
                    child.__write_into(
                        buf,
                        &(pad.to_owned() + "  "),
                        width,
                    )?
                }
                Ok(())
            },
        }
    }

    pub(super) fn write_into<W>(&self, buf: &mut W) -> Result<(), Error>
    where
        W: Write,
    {
        self.__write_into(buf, "", self.lead_width())
    }
}
