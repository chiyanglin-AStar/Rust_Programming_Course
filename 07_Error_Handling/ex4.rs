// Example 4: Propagating errors with ?
use std::fs::File;
use std::io::{self, Read};
fn read_file_contents(filename: &str) -> io::Result<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
