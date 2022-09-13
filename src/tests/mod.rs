use crate::render_diff;

const DIFF: &'static str = include_str!("wktest.diff");
const NEW: &'static str = include_str!("wktest.new");
const ORIGINAL: &'static str = include_str!("wktest.original");

#[test]
fn test_diff_u_from_wikipedia() {
    assert_eq!(DIFF, render_diff(&diff::lines(ORIGINAL, NEW))
        .into_iter()
        .map(|d| d.to_string())
        .collect::<Vec<String>>()
        .join("\n"))
}
