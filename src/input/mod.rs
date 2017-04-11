pub mod parse;

use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use primitives::label::Label;


pub fn import_labels(path : &String) -> Result<Vec<Label>, Box<Error>> {
    let mut result : Vec<Label> = Vec::new();
    
    let mut input_file =File::open(path)?;
    let mut reader = BufReader::new(input_file);
    
    let mut total : usize = 0;
    for (idx, line_res) in reader.lines().enumerate() {
        let line = line_res.unwrap().to_string();
        if idx == 0 {
            total = line.parse().expect("Could not parse number of labels in the file");
            println!("Reading {} labels from the file", total);
            continue;
        } else if idx == 1 {
//             println!("The file header line is:\n\t{}", line);
            continue;
        }
        
        result.push(parse::parse_label(&line));
    }
    
    Ok(result)
}
