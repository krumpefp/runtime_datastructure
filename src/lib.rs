#[macro_use]
extern crate lazy_static;

extern crate libc;
extern crate regex;

///
/// A module providing some primitive geo types.
///
/// A BoundingBox (BBox) is a 2 dimensional bounding box.
///
/// A Label is a point label with a given 2 dimensional position. It is linked
/// to an osm object via its osm_id and has a certain priority.
///
pub mod primitives;

///
/// A module that implements a 3 dimensional priority search tree on label data.
///
/// The 3 dimensional PST is a priority search tree where the elements are
/// splitted alternating by their x and y coordinate - similar to kd trees.
///
/// The 3d PST allows to find all labels within an half open interval:
///
/// ```text
/// (\infty, t] x [x_min, x_max] x [y_min, y_max]
/// ```
///
pub mod pst_3d;

///
/// A simple module to import data of label elimination sequences.
///
/// The module imports label elimination sequences from files of the form:
///
/// ```text
/// 5
/// lat lon osm_id priority collision_time label_length size_factor label
/// 53.143155300000004 8.9351249 3627273522 1 1.4922737369836614 3300.0 11.0 'Timmersloh'
/// 53.200157000000004 8.528893 253042611 2 1.5769136968447124 1650.0 11.0 'Farge'
/// 53.170524900000004 8.6238803 2147118476 3 2.2440622447579543 2880.0 12.0 'Vegesack'
/// 53.5522264 8.5865509 660314734 4 4.751763965397364 7260.0 22.0 'Bremerhaven'
/// 53.0758196 8.8071646 20982927 5 3686.835042292192 4320.0 24.0 'Bremen'
/// ```
///
/// Where the first line contains the number of elements<br>
/// The second line is a standard header<br>
/// Each of the following lines defines a label:<br>
///  * its position (lat, lon)<br>
///  * its collision time<br>
///  * its length<br>
///  * its size factor<br>
///  * the label string<br>
///
pub mod input;

use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;

use std::error::Error;
use std::io::prelude::*;
use std::fs::File;

#[repr(C)]
pub struct DataStructure {
    pst: Option<pst_3d::Pst3d>,

    last_res: Vec<C_Label>,
}

#[repr(C)]
pub struct C_Label {
    x: f64,
    y: f64,
    t: f64,

    osm_id: i64,
    prio: i32,

    label: *mut c_char,
}

#[repr(C)]
pub struct C_Result {
    size: u64,

    data: *mut C_Label,
}

#[no_mangle]
pub extern "C" fn init(input_path: *const c_char) -> Box<DataStructure> {
    let c_string = unsafe { CStr::from_ptr(input_path) };

    let input_path = match c_string.to_str() {
        Ok(path) => path.to_string(),
        Err(_) => {
            return Box::new(DataStructure {
                                pst: None,
                                last_res: Vec::new(),
                            })
        }
    };

    // debug
    let log_path = "log_ds.txt";
    match File::create(&log_path) {
        Err(why) => println!("couldn't create {}: {}", log_path, why.description()),
        Ok(mut file) => {
            match file.write_all(format!("Reading ds from {}", input_path).as_bytes()) {
                Err(why) => panic!("couldn't write to {}: {}", log_path, why.description()),
                Ok(_) => println!("successfully wrote to {}", log_path),
            };
        }
    }

    let tree: Option<pst_3d::Pst3d> = match input::import_labels(&input_path) {
        Ok(res) => {
            println!("Successfully imported {} labels", res.len());
            Some(pst_3d::Pst3d::new(res))
        }
        Err(e) => {
            println!("Could not read the given input file:{}\n\t{:?}\n",
                     input_path,
                     e);
            None
        }
    };

    Box::new(DataStructure {
                 pst: tree,
                 last_res: Vec::new(),
             })
}

#[no_mangle]
pub extern "C" fn is_good(ds: &mut DataStructure) -> bool {
    return ds.pst.is_some();
}

#[no_mangle]
pub extern "C" fn get_data(ds: &mut DataStructure,
                           min_t: f64,
                           min_x: f64,
                           max_x: f64,
                           min_y: f64,
                           max_y: f64)
                           -> C_Result {
    let pst = match ds.pst {
        Some(ref pst) => pst,
        None => {
            ds.last_res = Vec::new();

            return C_Result {
                       size: ds.last_res.len() as u64,
                       data: ds.last_res.as_mut_ptr(),
                   };
        }
    };

    let bb = primitives::bbox::BBox::new(min_x, min_y, max_x, max_y);
    let r = pst.get(&bb, min_t);

    ds.last_res = Vec::new();
    for e in &r {
        let c_label = CString::new(e.get_label().as_str()).unwrap();
        ds.last_res
            .push(C_Label {
                      x: e.get_x(),
                      y: e.get_y(),
                      t: e.get_t(),
                      osm_id: e.get_osm_id(),
                      prio: e.get_prio(),
                      label: c_label.into_raw(),
                  });
    }
    C_Result {
        size: r.len() as u64,
        data: ds.last_res.as_mut_ptr(),
    }
}
