use std::process::Command;

pub struct Targets(Vec<u8>);

impl Targets {
    pub fn iter(&self) -> Result<TargetsIter, std::str::Utf8Error> {
        std::str::from_utf8(&self.0)
            .map(str::lines)
            .map(TargetsIter)
    }
}

pub struct TargetsIter<'a>(std::str::Lines<'a>);

impl<'a> Iterator for TargetsIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
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
