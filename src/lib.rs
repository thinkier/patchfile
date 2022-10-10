extern crate diff;
extern crate ansi_term;

mod model;
mod algo;

use std::cmp::max;

use ansi_term::Colour;
use diff::Result as DiffDelta;
use crate::algo::render_diff;
use crate::model::{DiffU, LineNumber, Range};

pub fn display_diff(name: &str, remote_time: &str, local_time: &str, diff: &Vec<DiffDelta<&str>>) {
    println!("{}", Colour::Red.paint(format!("--- {}\t{}", name, local_time)));
    println!("{}", Colour::Green.paint(format!("+++ {}\t{}", name, remote_time)));

    for x in render_diff(diff) {
        let s = x.to_string();
        println!("{}", match x {
            DiffU::CaretPos { .. } => {
                Colour::Cyan.bold().paint(s)
            }
            DiffU::Addition(_) => {
                Colour::Green.paint(s)
            }
            DiffU::Deletion(_) => {
                Colour::Red.paint(s)
            }
            DiffU::Display(_) => {
                Colour::White.dimmed().paint(s)
            }
        });
    };
}

#[cfg(test)]
mod tests;
