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

use std::f64;
use std::cmp::Ordering;

use primitives::label::Label;
use primitives::bbox::BBox;

///
/// Represent the possible split dimensions.
///
#[derive(PartialEq)]
enum SplitDimension {
    X,
    Y,
    UNDEF,
}

///
/// The struct defines a tree node.
///
/// The tree nodes members are the labels t value, the label itself, the split type (X, Y or UNDEF
/// in case the node is a leaf node).
///
/// The split value indicates the maximum value of the left children in the corresponding
/// dimension. The split value is guaranteed to be less than the corresponding coordinate of the
/// right children.
///
/// Left and right child are some indices, if there is a left or right subtree and none otherwise.
pub struct Root {
    m_t: f64,
    m_data: Label,
    m_type: SplitDimension,
    m_split: f64,
    m_left_child: Option<usize>,
    m_right_child: Option<usize>,
}

impl Root {
    ///
    /// Construct a new root from the given label
    ///
    /// Note: The function only contains the given label. No subtrees of connenctions to other
    /// tree nodes are constructed.
    ///
    /// To construct a single tree from a forest of root nodes use the Root::init_pst3d(...)
    /// function.
    ///
    pub fn new(l: Label) -> Root {
        Root {
            m_t: l.get_t(),
            m_data: l,

            m_type: SplitDimension::UNDEF,
            m_split: f64::NAN,
            m_left_child: None,
            m_right_child: None,
        }
    }

    ///
    /// Initialize a single 3D PST out of a forest of root nodes and return the root node index.
    ///
    /// The function will mutate the given root nodes and set the corresponding split type, split
    /// value and left and right subtree indices.
    ///
    /// The function returns the index of the root node in the data array.
    ///
    pub fn init_pst3d(mut data: &mut Vec<Root>) -> Option<usize> {
        let mut refs: Vec<RootRef> = Vec::with_capacity(data.len());

        data.sort_by(|first, second| if first.m_t < second.m_t {
                         Ordering::Less
                     } else if first.m_t > second.m_t {
            Ordering::Greater
        } else {
            Ordering::Equal
        });
        data.reverse();

        for (idx, d) in data.iter().enumerate() {
            refs.push(RootRef::new(d, idx));
        }

        let initial_dimension = SplitDimension::X;
        create_root(refs, &mut data, &initial_dimension)
    }

    ///
    /// Get a vector of references to the elements in the 3d PST with t >= min_t and that are
    /// contained in bbox.
    ///
    pub fn get<'a>(&'a self, bbox: &BBox, min_t: f64, data: &'a Vec<Root>) -> Vec<&'a Label> {
        let mut r: Vec<&Label> = Vec::new();

        if self.m_t <= min_t {
            return r;
        }

        if bbox.is_contained(&self.m_data) {
            r.push(&self.m_data);
        }

        // append the left child if it exists and is cut by the bounding box
        if let Some(idx) = self.m_left_child {
            let append = match self.m_type {
                SplitDimension::X => bbox.get_min_x() <= self.m_split,
                SplitDimension::Y => bbox.get_min_y() <= self.m_split,
                SplitDimension::UNDEF => false,
            };

            if append {
                assert!(idx < data.len());
                let mut res = data[idx].get(&bbox, min_t, &data);
                r.append(&mut res);
            }
        }
        // append the right child if it exists and is cut by the bounding box
        if let Some(idx) = self.m_right_child {
            let append = match self.m_type {
                SplitDimension::X => bbox.get_max_x() > self.m_split,
                SplitDimension::Y => bbox.get_max_y() > self.m_split,
                SplitDimension::UNDEF => false,
            };

            if append {
                assert!(idx < data.len());
                let mut res = data[idx].get(&bbox, min_t, &data);
                r.append(&mut res);
            }
        }

        r
    }

    ///
    /// Get a human readable string representation of the tree rooted at self.
    ///
    /// A such string will look like:
    ///
    /// ```text
    ///   x-node (split: 4.5): Label [#1]: 'T1' at (1, 2) with prio 1,elim-t: 9 and label factor: \
    ///                                                                                         1.5
    ///   l    y-node (split: 4.5): Label [#2]: 'T2' at (2, 3) with prio 1, elim-t: 8 and label \
    ///                                                                                 factor: 1.5
    ///   l        x-node (split: NaN): Label [#3]: 'T3' at (3, 4) with prio 1, elim-t: 7 and \
    ///                                                                           label factor: 1.5
    /// ```
    ///
    pub fn to_string(&self, level: i32, data: &Vec<Root>) -> String {
        // prefix is level x p
        let p = "    ";
        let mut prefix = String::new();
        for _ in 0..level {
            prefix = format!("{}{}", prefix, p);
        }

        let mut result = match self.m_type {
            SplitDimension::X => {
                format!("{}x-node (split: {}): {}",
                        prefix,
                        self.m_split,
                        self.m_data.to_string())
            }
            SplitDimension::Y => {
                format!("{}y-node (split: {}): {}",
                        prefix,
                        self.m_split,
                        self.m_data.to_string())
            }
            SplitDimension::UNDEF => {
                format!("{}leaf-node (split: {}): {}",
                        prefix,
                        self.m_split,
                        self.m_data.to_string())
            }
        };

        // append the left subtree
        if let Some(idx) = self.m_left_child {
            assert!(idx < data.len());
            result = format!("{}\nl{}", result, data[idx].to_string(level + 1, &data));
        }
        // append the right subtree
        if let Some(idx) = self.m_right_child {
            assert!(idx < data.len());
            result = format!("{}\nr{}", result, data[idx].to_string(level + 1, &data));
        }

        result
    }
}

///
/// The struct represents a reference to a root node and contains all the information required to
/// construct the 3D PST.
///
#[derive(Debug)]
struct RootRef {
    m_x: f64,
    m_y: f64,
    m_t: f64,

    m_idx: usize,
}

impl RootRef {
    ///
    /// Initialize a new root ref
    ///
    fn new(r: &Root, idx: usize) -> RootRef {
        RootRef {
            m_t: r.m_data.get_t(),
            m_x: r.m_data.get_x(),
            m_y: r.m_data.get_y(),

            m_idx: idx,
        }
    }

    ///
    /// Compare two Root refs with respect to the t value.
    ///
    fn order_by_t(first: &Self, second: &Self) -> Ordering {
        if first.m_t < second.m_t {
            Ordering::Less
        } else if first.m_t > second.m_t {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }

    ///
    /// Compare two Root refs with respect to the x value.
    ///
    fn order_by_x(first: &Self, second: &Self) -> Ordering {
        if first.m_x < second.m_x {
            Ordering::Less
        } else if first.m_x > second.m_x {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }

    ///
    /// Compare two Root refs with respect to the y value.
    ///
    fn order_by_y(first: &Self, second: &Self) -> Ordering {
        if first.m_y < second.m_y {
            Ordering::Less
        } else if first.m_y > second.m_y {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

///
/// In the RootRef vector find the index of the root with the maximum t value.
///
fn find_root_idx(refs: &mut Vec<RootRef>) -> usize {
    let mut max_t = 0.;
    let mut max_idx = 0;
    for (idx, e) in refs.iter().enumerate() {
        if e.m_t > max_t {
            max_t = e.m_t;
            max_idx = idx;
        }
    }

    let r = refs.swap_remove(max_idx);
    assert!(r.m_t == max_t);

    r.m_idx
}

///
/// From the given RootRef vector construct the subtree and update the corresponding root nodes in
/// the data vector.
///
/// The element with the maximum t value will be set as root with the corresponding split
/// dimension. The remaining elements will sorted by the split dimension. The split value is the
/// corresponding coordinate of item floor(|root_refs| / 2) and the elements are splitted into <=
/// and >.
///
/// From the <= elements the left subtree is constructed recursively with swapped split dimension.
/// Same for the > elements as the right subtree.
///
/// For the nodes in data that are referenced by RootRefs in root_refs the  corresponding Roots are
/// updated accordingly.
///

fn create_root(mut root_refs: Vec<RootRef>,
               mut data: &mut Vec<Root>,
               dim: &SplitDimension)
               -> Option<usize> {
    if root_refs.is_empty() {
        return None;
    }


    let size1 = root_refs.len();

    assert!(*dim != SplitDimension::UNDEF);
    let is_x = *dim == SplitDimension::X;

    // find the element with the maximum t value, remove the corresonding RootRef
    let root_idx = find_root_idx(&mut root_refs);

    // the sub dimension flips from X to Y or from Y to X
    let sub_dim = if is_x {
        SplitDimension::Y
    } else {
        SplitDimension::X
    };

    let mut split_value = f64::NAN;
    let mut left_child_idx: Option<usize> = None;
    let mut right_child_idx: Option<usize> = None;

    let order_asc = if is_x {
        RootRef::order_by_x
    } else {
        RootRef::order_by_y
    };
    if root_refs.len() == 1 {
        split_value = if is_x {
            root_refs[0].m_x
        } else {
            root_refs[0].m_y
        };
        left_child_idx = create_root(root_refs, &mut data, &sub_dim);
    } else if root_refs.len() > 1 {
        root_refs.sort_by(order_asc);

        // take the x value of the median element as the new split value
        let mut median_idx = root_refs.len() / 2 - 1;
        split_value = if is_x {
            root_refs[median_idx].m_x
        } else {
            root_refs[median_idx].m_y
        };

        // ensure that the right children realy have a value > m_split
        if is_x {
            while median_idx < root_refs.len() && root_refs[median_idx].m_x == split_value {
                median_idx = median_idx + 1;
            }
        } else {
            while median_idx < root_refs.len() && root_refs[median_idx].m_y == split_value {
                median_idx = median_idx + 1;
            }
        }

        let size2 = root_refs.len();
        assert!(size1 == size2 + 1);
        // split the data at the median point:
        let last = root_refs.split_off(median_idx);
        assert!(size2 == root_refs.len() + last.len());

        left_child_idx = create_root(root_refs, &mut data, &sub_dim);
        right_child_idx = create_root(last, &mut data, &sub_dim);
    }

    let r = data.get_mut(root_idx)
        .expect("Trying to access element at not existing vector position");

    r.m_type = if is_x {
        SplitDimension::X
    } else {
        SplitDimension::Y
    };
    r.m_split = split_value;
    r.m_left_child = left_child_idx;
    r.m_right_child = right_child_idx;

    Some(root_idx)
}

#[test]
fn test_root_new() {
    let r = Root::new(Label::new(1., 2., 9., 1, 1, 1.5, "A".to_string()));

    assert!(r.m_t == 9.);
    assert!(*r.m_data.get_label() == "A".to_string());
    assert!(r.m_type == SplitDimension::UNDEF);
}

#[test]
fn test_pst_init() {
    let mut f: Vec<Root> = Vec::new();
    f.push(Root::new(Label::new(1., 2., 9., 1, 1, 1.5, "A".to_string())));
    f.push(Root::new(Label::new(2., 3., 8., 2, 1, 1.5, "B".to_string())));
    f.push(Root::new(Label::new(3., 4., 7., 3, 1, 1.5, "C".to_string())));

    let root = Root::init_pst3d(&mut f);
    let root_idx = root.unwrap();
    println!("{}", f[root_idx].to_string(0, &f));
    assert!(root_idx == 0);

    assert!(f[root_idx].m_type == SplitDimension::X);
    assert!(f[root_idx].m_left_child.is_some());
    assert!(f[root_idx].m_right_child.is_some());

    assert!(f[root_idx].m_left_child.unwrap() == 1);
    assert!(f[root_idx].m_right_child.unwrap() == 2);
}
