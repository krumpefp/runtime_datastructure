// extern crate runtime_datastructure;
//
// use runtime_datastructure::input;
// use runtime_datastructure::primitives;
// use runtime_datastructure::pst_3d;

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


    //     let testing = false;
    //     let labels = match input::import_labels(&config.m_input_path) {
    //         Ok(res) => {
    //             println!("Successfully imported {} labels", res.len());
    //             res
    //         },
    //         Err(e) => panic!("Could not read the given input file:\
    // {}\n\t{:?}\n", config.m_input_path, e),
    //     };
    //
    //     if testing {
    //         for (idx, l) in labels.iter().enumerate() {
    //             println!("Parsed label (#{}):\n{}", idx, l.to_string());
    //         }
    //     }
    //
    //     let mut tree = pst_3d::PST_3D::new(labels.clone());
    //
    //     // Testing stuff ...
    //     if testing {
    //         let l = primitives::label::Label::new(90., 90., 0.9, 1234567,
    // 16, "Test".to_string());
    //
    //         println!("Test label:\n{}", l.to_string());
    //
    //         println!("Starting to create a 3d priority search tree ...");
    //
    //         let mut v = Vec::new();
    //         v.push(primitives::label::Label::new(1., 2., 9., 1, 1, "T1".to_string()));
    //         v.push(primitives::label::Label::new(2., 3., 8., 2, 1, "T2".to_string()));
    //         v.push(primitives::label::Label::new(3., 4., 7., 3, 1, "T3".to_string()));
    //         v.push(primitives::label::Label::new(3., 5., 7., 4, 1, "T4".to_string()));
    //
    //         v.push(primitives::label::Label::new(4., 6., 6., 5, 1, "T5".to_string()));
    //         v.push(primitives::label::Label::new(5., 7., 5., 6, 1, "T6".to_string()));
    //         v.push(primitives::label::Label::new(6., 8., 4., 7, 1, "T7".to_string()));
    //
    //         v.push(primitives::label::Label::new(7., 9., 3., 8, 1, "T8".to_string()));
    //         v.push(primitives::label::Label::new(8., 10., 2., 9, 1, "T9".to_string()));
    //         v.push(primitives::label::Label::new(9., 11., 1., 10, 1, "T10".to_string()));
    //
    //         let t = pst_3d::PST_3D::new(v.clone());
    //
    //         println!("... finished!");
    //
    //         println!("\n####    TREE:\n{}", t.to_string());
    //
    //         let bb = primitives::bbox::BBox::new(4., 5., 7., 8.);
    //         let r = t.get(&bb, config.m_min_t);
    //
    //         println!("Requesting labels with min t: {} in {}",
    //config.m_min_t, bb.to_string());
    //
    //         println!("\n####    RESULTS:");
    //         for elem in r {
    //             println!("Result: {}", elem.to_string());
    //         }
    //     }
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
