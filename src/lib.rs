use std::{ops::Deref, process::Command};

pub struct Targets(Vec<u8>);

impl Targets {
    pub fn iter(&self) -> Result<TargetsIter, std::str::Utf8Error> {
        std::str::from_utf8(&self.0)
            .map(str::lines)
            .map(TargetsIter)
    }

    pub fn collect_strs(&self) -> Result<CollectedTargetsStrs, std::str::Utf8Error> {
        std::str::from_utf8(&self.0)
            .map(str::lines)
            .map(|targets| targets.collect())
            .map(CollectedTargetsStrs::from_vec)
    }
}

pub struct TargetsIter<'a>(std::str::Lines<'a>);

impl<'a> Iterator for TargetsIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

pub struct CollectedTargetsStrs<'a>(Vec<&'a str>);

impl<'a> CollectedTargetsStrs<'a> {
    fn from_vec(vec: Vec<&'a str>) -> Self {
        #[cfg(debug_assertions)]
        for window in vec.windows(2) {
            assert!(
                window[0] < window[1],
                "The targets from `go tool dist list` were expected to be sorted lexicographically."
            );
        }
        Self(vec)
    }
}

impl<'a> CollectedTargetsStrs<'a> {
    pub fn contains(&self, target: &str) -> bool {
        self.0.binary_search(&target).is_ok()
    }
}

impl<'a> Deref for CollectedTargetsStrs<'a> {
    type Target = [&'a str];

    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

/// Returns the list of targets supported by the current go compiler using CLI.
///
/// The list of targets can be iterated over using the `iter` method:
///
/// # Example
///
/// ```rust
#[doc = include_str!("../examples/targets.rs")]
/// ```
pub fn from_cli() -> Result<Targets, std::io::Error> {
    let output = Command::new("go")
        .arg("tool")
        .arg("dist")
        .arg("list")
        .output()?;

    Ok(Targets(output.stdout))
}
