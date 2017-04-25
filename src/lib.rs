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

#[macro_use]
extern crate lazy_static;

extern crate rand;

extern crate regex;

///
/// A module providing some primitive geo types.
///
/// A BoundingBox (BBox) is a 2 dimensional bounding box.
///
/// A Label is a point label with a given 2 dimensional position. It is linked to an osm object via
/// its osm_id and has a certain priority.
///
pub mod primitives;

///
/// A module that implements a 3 dimensional priority search tree on label data.
///
/// The 3 dimensional PST is a priority search tree where the elements are splitted alternating by
/// their x and y coordinate - similar to kd trees.
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
///
/// The second line is a standard header<br>
///
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

///
/// C respresentation of a pst instance It also contains and owns the data that was returned at the
/// last request.
///
/// After initializing the pst by the C interface, a pointer DataStructure object will be returned
/// caller. The pointer should not be modified from outside!
///
/// To get data, the struct pointer must be given to the corresponding function as an argument.
///
#[repr(C)]
pub struct DataStructure {
    pst: Option<pst_3d::Pst3d>,

    last_res: Vec<C_Label>,
}

///
/// A C representation of a label and its data.
///
/// The result of requests of the data structure will be returned as an c-array of these structs.
///
#[repr(C)]
pub struct C_Label {
    x: f64,
    y: f64,
    t: f64,

    osm_id: i64,
    prio: i32,

    lbl_fac: f64,
    label: *mut c_char,
}

///
/// A struct represents a basic C_Label vector, i.e. its size and the data (the contained C_Label
/// objects).
///
#[repr(C)]
pub struct C_Result {
    size: u64,

    data: *mut C_Label,
}

///
/// Initialize a 3D PST from the file defined by input_path.
///
/// The returned pointer to the DataStructure object can be used to request data from the 3D PST.
///
/// The given file must match the format specified in the [Input Module](input/index.html).
///
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

///
/// Check if the initialization was successfull and the returned DataStructure object is valid.
///
#[no_mangle]
pub extern "C" fn is_good(ds: &mut DataStructure) -> bool {
    return ds.pst.is_some();
}

///
/// Get the labels contained in the specified bounding box with a t value >= min_t.
///
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
                      lbl_fac: e.get_label_factor(),
                      label: c_label.into_raw(),
                  });
    }
    C_Result {
        size: r.len() as u64,
        data: ds.last_res.as_mut_ptr(),
    }
}


#[cfg(test)]
mod tests {
    extern crate rand;
    
    
    const TEST_SIZE: usize = 500;
    const TEST_COUNT: usize = 1;
    
    use rand::{thread_rng, Rng};

    use std::collections::HashSet;

    use super::primitives::{bbox, label};
    use super::pst_3d;

    // create a random floating point number in the range -180 to 180
    fn rand_lat() -> f64 {
        180. * rand::random::<f64>() - 90.
    }

    // create a random floating point number in the range -90 to 90
    fn rand_lon() -> f64 {
        360. * rand::random::<f64>() - 180.
    }

    // create a random level instance of count many elements
    fn random_label_instance(count: usize) -> Vec<label::Label> {
        let mut v: Vec<label::Label> = Vec::new();

        for counter in 1..count {
            let lat = rand_lat();
            let lon = rand_lon();
            let t = rand::random::<f64>();

            v.push(label::Label::new(lon,
                                     lat,
                                     t,
                                     counter as i64,
                                     counter as i32,
                                     1.0,                  // label factor is not of interesst
                                     format!("T {}", counter)));
        }

        v
    }

    // get a hash set of ids of the labels in the label list
    fn get_id_set(v: &Vec<&label::Label>) -> HashSet<i64> {
        let mut res = HashSet::new();

        for id in v.iter().map(|l| l.get_osm_id()) {
            res.insert(id);
        }

        res
    }

    // get a hash set of ids of the labels in the label list
    fn get_id_set_filtered(v: &Vec<label::Label>, bbox: &bbox::BBox, t: f64) -> HashSet<i64> {
        let mut res = HashSet::new();

        for id in v.iter()
                .filter(|l| l.get_t() >= t)
                .filter(|l| bbox.is_contained(l))
                .map(|l| l.get_osm_id()) {
            res.insert(id);
        }

        res
    }

    #[test]
    fn randomized_test() {
        let instance = random_label_instance(TEST_SIZE);

        let mut data_box = bbox::BBox::new_empty();
        for l in &instance {
            data_box.add_to_box(l);
        }

        let pskdt = pst_3d::Pst3d::new(instance.clone());



        let mut rng = rand::thread_rng();

        for i in 0..TEST_COUNT {
            let t = rand::random::<f64>();

            let min_x = rng.gen_range(data_box.get_min_x(), data_box.get_max_x());
            let max_x = rng.gen_range(min_x, data_box.get_max_x());
            let min_y = rng.gen_range(data_box.get_min_y(), data_box.get_max_y());
            let max_y = rng.gen_range(min_y, data_box.get_max_y());

            let bbox = bbox::BBox::new(min_x, min_y, max_x, max_y);

            let res = pskdt.get(&bbox, t);
            
            assert!(get_id_set(&res) == get_id_set_filtered(&instance, &bbox, t));
        }
    }
}
