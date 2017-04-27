///
/// A module to parse lines strings and create a corresponding label object.
///
/// The strings must be of the form defined in the [Module description](../index.html)
/// 
pub mod parse;

use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use primitives::label::Label;

///
/// import the label elimination data given by the file at 'path' into a vector
///
/// # Errors
///   * if the file path does not match any file in the file system
///   * if the number of labels does not match the specified number of labels
///
pub fn import_labels(path: &String) -> Result<Vec<Label>, Box<Error>> {
    let mut result: Vec<Label> = Vec::new();

    let input_file = File::open(path)?;
    let reader = BufReader::new(input_file);

    let mut total: usize = 0;
    for (idx, line_res) in reader.lines().enumerate() {
        let line = line_res.unwrap().to_string();
        if idx == 0 {
            total = line.parse()
                .expect("Could not parse number of labels in the file");
            println!("Reading {} labels from the file", total);
            continue;
        } else if idx == 1 {
            //             println!("The file header line is:\n\t{}", line);
            continue;
        }

        result.push(parse::parse_label(&line));
    }

    if (total != result.len()) {
        return Err(From::from("Specified number of labels does not match real label size!"));
    }


    Ok(result)
}
