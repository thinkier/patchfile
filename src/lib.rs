extern crate diff;

mod model;
mod algo;

use diff::Result as DiffDelta;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use crate::algo::render_diff;
use crate::model::{DiffU, LineNumber, Range};
use std::io::Result as IoResult;

/// Creates a builder for converting two files into a patchfile
pub struct Patchfile<'a> {
    remote_name: &'a str,
    local_name: &'a str,
    remote_time: Option<&'a str>,
    local_time: Option<&'a str>,
}

impl<'a> Patchfile<'a> {
    /// Sets the same name for both files
    pub fn new(name: &'a str) -> Self {
        Patchfile {
            remote_name: name,
            local_name: name,
            remote_time: None,
            local_time: None,
        }
    }

    /// Sets different names for both files
    pub fn with_names(remote_name: &'a str, local_name: &'a str) -> Self {
        Patchfile {
            remote_name,
            local_name,
            remote_time: None,
            local_time: None,
        }
    }

    /// Set the remote file's timestamp
    pub fn remote_time(&'a mut self, remote_time: &'a str) -> &'a mut Self {
        self.remote_time = Some(remote_time);

        self
    }

    /// Set the local file's timestamp
    pub fn local_time(&'a mut self, local_time: &'a str) -> &'a mut Self {
        self.local_time = Some(local_time);

        self
    }

    /// Write patch formatted text to stdout
    pub fn print<'b>(&'a self, remote: &'b str, local: &'b str) -> IoResult<()> {
        self.write_impl(StandardStream::stdout(ColorChoice::Auto), remote, local)
    }

    /// Write patch formatted text to stderr
    pub fn eprint<'b>(&'a self, remote: &'b str, local: &'b str) -> IoResult<()> {
        self.write_impl(StandardStream::stderr(ColorChoice::Auto), remote, local)
    }

    fn write_impl<W: WriteColor>(&self, mut w: W, remote: &str, local: &str) -> IoResult<()> {
        let diff = diff::lines(local, remote);

        w.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
        writeln!(w, "--- {}\t{}", self.local_name, self.local_time.unwrap_or(""))?;
        w.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
        writeln!(w, "+++ {}\t{}", self.remote_name, self.remote_time.unwrap_or(""))?;

        for d in render_diff(&diff) {
            let s = d.to_string();
            match d {
                DiffU::CaretPos { .. } => {
                    w.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))
                }
                DiffU::Addition(_) => {
                    w.set_color(ColorSpec::new().set_fg(Some(Color::Green)))
                }
                DiffU::Deletion(_) => {
                    w.set_color(ColorSpec::new().set_fg(Some(Color::Red)))
                }
                DiffU::Display(_) => {
                    w.set_color(ColorSpec::new().set_fg(None))
                }
            }?;

            writeln!(w, "{}", s)?;
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests;
