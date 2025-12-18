use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use regex::Regex;

fn normalize(text: &str) -> String {
    text
        // remove LTR / RTL marks
        .replace(['\u{202A}', '\u{202B}', '\u{202C}', '\u{202D}', '\u{202E}'], "")
        // normalize spaces
        .replace('\u{00A0}', " ")
        // normalize hyphens
        .replace(['\u{2010}', '\u{2011}', '\u{2012}', '\u{2013}', '\u{2014}'], "-")
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }

    let input_path = Path::new(&args[1]);

    if !input_path.exists() {
        eprintln!("File not found: {}", input_path.display());
        std::process::exit(1);
    }

    // Output file in same directory
    let output_path = input_path
        .parent()
        .unwrap_or(Path::new("."))
        .join("numbers.txt");

    let input_file = File::open(input_path)?;
    let output_file = File::create(&output_path)?;

    let reader = BufReader::new(input_file);
    let mut writer = std::io::BufWriter::new(output_file);

    let mut seen = HashSet::new();
    let re = Regex::new(r"\+977\s?\d{3}-\d{7}").unwrap();

    for line in reader.lines() {
        let line = normalize(&line?);

        for m in re.find_iter(&line) {
            let number = m.as_str().replace([' ', '-'], "");

            if seen.insert(number.clone()) {
                writeln!(writer, "{}", number)?;
            }
        }
    }

    println!(
        "Saved {} unique numbers to {}",
        seen.len(),
        output_path.display()
    );

    Ok(())
}

