use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn read_file(filename: String) -> anyhow::Result<Vec<String>> {
    let input = File::open(filename)?;
    let buffered = BufReader::new(input);

    Ok(buffered
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>())
}
