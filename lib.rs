extern crate diff;
use std::cmp::max;

use ansi_term::Colour;
use diff::Result as DiffDelta;

#[derive(Debug, Clone)]
struct LineNumber {
    left: usize,
    right: usize,
}

#[derive(Debug, Clone)]
struct Range {
    start: usize,
    count: usize,
}

#[derive(Debug, Clone)]
enum DiffU<'a> {
    CaretPos {
        left: Range,
        right: Range,
    },
    Addition(&'a str),
    Deletion(&'a str),
    Display(&'a str),
}

impl<'a> ToString for DiffU<'a> {
    fn to_string(&self) -> String {
        match self {
            DiffU::CaretPos { left, right } => format!("@@ -{},{} +{},{} @@", left.start, left.count, right.start, right.count),
            DiffU::Addition(s) => format!("+{}", s),
            DiffU::Deletion(s) => format!("-{}", s),
            DiffU::Display(s) => format!(" {}", s),
        }
    }
}

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

const SELECT_CLEARANCE: usize = 3;
const LOOKBACK_RANGE: usize = SELECT_CLEARANCE * 2 - 1;

fn render_diff<'a>(diff: &Vec<DiffDelta<&'a str>>) -> Vec<DiffU<'a>> {
    let mut buf = vec![];

    let mut zipped_lines = vec![];
    {
        let mut left = 0;
        let mut right = 0;
        for d in diff {
            let content = match d {
                DiffDelta::Left(s) => {
                    left += 1;
                    DiffU::Deletion(*s)
                }
                DiffDelta::Both(_, s) => {
                    left += 1;
                    right += 1;
                    DiffU::Display(*s)
                }
                DiffDelta::Right(s) => {
                    right += 1;
                    DiffU::Addition(*s)
                }
            };

            let ln = LineNumber { left: max(left, 1), right: max(right, 1) };
            zipped_lines.push((ln, content));
        }
    }

    let mut sync_pos = vec![];
    {
        let mut delta_start = None;
        let mut consecutive_sames = 0;
        for (i, (_, d)) in zipped_lines.iter().enumerate().skip(1) {
            match d {
                DiffU::CaretPos { .. } | DiffU::Display(_) => {
                    consecutive_sames += 1;

                    if consecutive_sames >= LOOKBACK_RANGE {
                        if let Some(mut start) = delta_start {
                            delta_start = None;

                            if start > SELECT_CLEARANCE {
                                start -= SELECT_CLEARANCE;
                            } else {
                                start = 0;
                            }
                            sync_pos.push((start, i + 1 - SELECT_CLEARANCE));
                        }
                    }
                    continue;
                }
                DiffU::Addition(_) | DiffU::Deletion(_) => {
                    consecutive_sames = 0;

                    if delta_start.is_none() {
                        delta_start = Some(i);
                    }
                }
            }
        }

        if let Some(mut start) = delta_start {
            if start > SELECT_CLEARANCE {
                start -= SELECT_CLEARANCE;
            } else {
                start = 0;
            }
            sync_pos.push((start, zipped_lines.len() - 1));
        }
    }

    if sync_pos.is_empty() {
        sync_pos.push((0, diff.len() - 1));
    }

    for (lower, upper) in sync_pos {
        let left_line = zipped_lines[lower].0.left;
        let left_count = zipped_lines[upper].0.left - left_line + 1;
        let right_line = zipped_lines[lower].0.right;
        let right_count = zipped_lines[upper].0.right - right_line + 1;
        buf.push(DiffU::CaretPos {
            left: Range { start: left_line, count: left_count },
            right: Range { start: right_line, count: right_count },
        });

        for i in lower..=upper {
            buf.push(zipped_lines[i].1.clone());
        }
    }

    return buf;
}

#[cfg(test)]
mod tests;
