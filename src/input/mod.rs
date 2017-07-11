/*
    The library provides a simple datastructure to access geolocated labels with an additional
    elimination time t and a label size factor. The library provides method to query a set of
    such labels with a bounding box and a minimum elimination time.

    Copyright (C) {2017}  {Filip Krumpe <filip.krumpe@fmi.uni-stuttgart.de}

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

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
/// import the label elimination data given by the file at 'path' into a vector.
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
            total = line.parse()?;
            println!("Reading {} labels from the file", total);
            continue;
        } else if idx == 1 {
            // skip the header line
            continue;
        }

        match parse::parse_label(&line) {
            Ok(label) => result.push(label),
            Err(e) => {
                println!("Line {} could not be parsed!\nRepored error was: {}", line, e);
                continue;
            },
        }
    }

    if total != result.len() {
        return Err(From::from("Specified number of labels does not match real label size!"));
    }


    Ok(result)
}
