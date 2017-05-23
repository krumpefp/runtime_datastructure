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

extern crate rt_datastructure;

use rt_datastructure::input;
use rt_datastructure::primitives;
use rt_datastructure::pst_3d;

use std::env;
use std::error::Error;
use std::process;

fn main() {
    println!("Hallo :-)");

    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
                                                       println!("Problem parsing arguments: {}",
                                                                err);
                                                       process::exit(1);
                                                   });

    println!("Starting runtime datastructure with min_t: {}",
             config.m_min_t);
    if config.m_input_path != "" {
        println!(" and path {}", config.m_input_path);
    }


    let testing = false;
    let labels = match input::import_labels(&config.m_input_path) {
        Ok(res) => {
            println!("Successfully imported {} labels", res.len());
            res
        }
        Err(e) => {
            panic!("Could not read the given input file:\
    {}\n\t{:?}\n",
                   config.m_input_path,
                   e)
        }
    };

    if testing {
        for (idx, l) in labels.iter().enumerate() {
            println!("Parsed label (#{}):\n{}", idx, l.to_string());
        }
    }


    // Testing stuff ...
    if testing {
        let l = primitives::label::Label::new(90., 90., 0.9, 1234567, 16, 1.5, "Test".to_string());

        println!("Test label:\n{}", l.to_string());

        println!("Starting to create a 3d priority search tree ...");

        let mut v = Vec::new();
        v.push(primitives::label::Label::new(1., 2., 9., 1, 1, 1.5, "T1".to_string()));
        v.push(primitives::label::Label::new(2., 3., 8., 2, 1, 1.5, "T2".to_string()));
        v.push(primitives::label::Label::new(3., 4., 7., 3, 1, 1.5, "T3".to_string()));
        v.push(primitives::label::Label::new(3., 5., 7., 4, 1, 1.5, "T4".to_string()));

        v.push(primitives::label::Label::new(4., 6., 6., 5, 1, 1.5, "T5".to_string()));
        v.push(primitives::label::Label::new(5., 7., 5., 6, 1, 1.5, "T6".to_string()));
        v.push(primitives::label::Label::new(6., 8., 4., 7, 1, 1.5, "T7".to_string()));

        v.push(primitives::label::Label::new(7., 9., 3., 8, 1, 1.5, "T8".to_string()));
        v.push(primitives::label::Label::new(8., 10., 2., 9, 1, 1.5, "T9".to_string()));
        v.push(primitives::label::Label::new(9., 11., 1., 10, 1, 1.5, "T10".to_string()));

        let t = pst_3d::Pst3d::new(v.clone());

        println!("... finished!");

        println!("\n####    TREE:\n{}", t.to_string());

        let bb = primitives::bbox::BBox::new(4., 5., 7., 8.);
        let r = t.get(&bb, config.m_min_t);

        println!("Requesting labels with min t: {} in {}",
                 config.m_min_t,
                 bb.to_string());

        println!("\n####    RESULTS:");
        for elem in r {
            println!("Result: {}", elem.to_string());
        }
    }
}

struct Config {
    m_input_path: String,
    m_min_t: f64,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, Box<Error>> {
        let mut min_t = 4.;
        let mut path = "".to_string();

        if args.len() >= 2 {
            min_t = args[1].parse()?;
        }
        if args.len() >= 3 {
            path = args[2].clone();
        }

        Ok(Config {
               m_input_path: path,
               m_min_t: min_t,
           })
    }
}
