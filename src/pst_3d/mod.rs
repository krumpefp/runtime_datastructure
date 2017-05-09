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
/// Implements a tree node
///
mod root;

use primitives::label::Label;
use primitives::bbox::BBox;

use self::root::Root;

///
/// A wrapper to the Pst3d providing some additional coordinate range checks and some functions
/// specific for the geographic setting.
///
/// For every provided function dealing with geo-coordinates there are checks of the coordinate
/// values.
///
/// The getter function deals with "wrap arounds", i.e. a bounding box might range from lin -170
/// to 170
///
pub struct GeoPst3d {
    m_pst: Pst3d,
}

impl GeoPst3d {
    ///
    /// Initialize a new 3D PST from the given label vector.
    ///
    /// Take care: The procedure will consume the given vector!
    ///
    /// # Panics
    /// * if lat not in range [-90, 90]
    /// * if lon not in range [-180, 180]
    ///
    /// # Examples
    /// ```
    /// use rt_datastructure::primitives::label;
    /// use rt_datastructure::pst_3d;
    ///
    /// let mut v = Vec::new();
    /// v.push(label::Label::new(-180., -90., 9., 1, 1, 1.5, "T1".to_string()));
    /// v.push(label::Label::new(-140., -70., 8., 2, 1, 1.5, "T2".to_string()));
    /// v.push(label::Label::new(-100., -50., 7., 3, 1, 1.5, "T3".to_string()));
    /// v.push(label::Label::new(-60., -30., 7., 4, 1, 1.5, "T4".to_string()));
    /// v.push(label::Label::new(-20., -10., 6., 5, 1, 1.5, "T5".to_string()));
    /// v.push(label::Label::new(20., 10., 5., 6, 1, 1.5, "T6".to_string()));
    /// v.push(label::Label::new(60., 30., 4., 7, 1, 1.5, "T7".to_string()));
    /// v.push(label::Label::new(100., 50., 3., 8, 1, 1.5, "T8".to_string()));
    /// v.push(label::Label::new(140., 70., 2., 9, 1, 1.5, "T9".to_string()));
    /// v.push(label::Label::new(180., 90., 1., 10, 1, 1.5, "T10".to_string()));
    ///
    /// let _ = pst_3d::GeoPst3d::new(v);
    /// ```
    ///
    /// ```should_panic
    /// use rt_datastructure::primitives::label;
    /// use rt_datastructure::pst_3d;
    ///
    /// let mut v = Vec::new();
    /// v.push(label::Label::new(100., 100., 9., 1, 1, 1.5, "T1".to_string()));
    ///
    /// let _ = pst_3d::GeoPst3d::new(v);
    /// ```
    ///
    /// ```should_panic
    /// use rt_datastructure::primitives::label;
    /// use rt_datastructure::pst_3d;
    ///
    /// let mut v = Vec::new();
    /// v.push(label::Label::new(200., 90., 9., 1, 1, 1.5, "T1".to_string()));
    ///
    /// let _ = pst_3d::GeoPst3d::new(v);
    /// ```
    ///
    pub fn new(mut labels: Vec<Label>) -> GeoPst3d {
        // ensure that each Label has valid coordinates
        let bbox = BBox::new(-180., -90., 180., 90.);
        for l in &labels {
            if !bbox.is_contained(&l) {
                panic!("Label coordinates out of bounds");
            }
        }

        GeoPst3d { m_pst: Pst3d::new(labels) }
    }

    ///
    /// Return the set of label in the given bounding box with a t >= min_t.
    ///
    /// The getter supports a wraparound. So a request for a bounding box with min lon = 170 and
    /// max lon = -170 is valid
    ///
    /// # Examples
    /// ```
    /// use rt_datastructure::primitives::{label, bbox};
    /// use rt_datastructure::pst_3d;
    ///
    /// let mut v = Vec::new();
    /// v.push(label::Label::new(1., 2., 10., 1, 1, 1.5, "T1".to_string()));
    /// v.push(label::Label::new(2., 3., 9., 2, 1, 1.5, "T2".to_string()));
    /// v.push(label::Label::new(3., 4., 8., 3, 1, 1.5, "T3".to_string()));
    /// v.push(label::Label::new(4., 5., 7., 4, 1, 1.5, "T4".to_string()));
    /// v.push(label::Label::new(5., 6., 6., 5, 1, 1.5, "T5".to_string()));
    /// v.push(label::Label::new(6., 7., 5., 6, 1, 1.5, "T6".to_string()));
    /// v.push(label::Label::new(7., 8., 4., 7, 1, 1.5, "T7".to_string()));
    /// v.push(label::Label::new(8., 9., 3., 8, 1, 1.5, "T8".to_string()));
    /// v.push(label::Label::new(9., 10., 2., 9, 1, 1.5, "T9".to_string()));
    /// v.push(label::Label::new(10., 11., 1., 10, 1, 1.5, "T10".to_string()));
    ///
    /// let t = pst_3d::GeoPst3d::new(v);
    ///
    /// let bb = bbox::BBox::new(4., 5., 7., 8.);
    /// let r = t.get(&bb, 4.);
    ///
    /// for e in &r {
    ///   println!("{}, ", e.to_string());
    /// }
    ///
    /// // resulting labels are T5 to  T7
    /// assert!(r.len() == 3);
    /// ```
    ///
    /// with wraparound:
    ///
    /// ```
    /// use rt_datastructure::primitives::{label, bbox};
    /// use rt_datastructure::pst_3d;
    ///
    /// let mut v = Vec::new();
    /// v.push(label::Label::new(160., 20., 10., 1, 1, 1.5, "T1".to_string()));
    /// v.push(label::Label::new(170., 30., 9., 2, 1, 1.5, "T2".to_string()));
    /// v.push(label::Label::new(175., -40., 8., 3, 1, 1.5, "T3".to_string()));
    /// v.push(label::Label::new(180., 50., 7., 4, 1, 1.5, "T4".to_string()));
    /// v.push(label::Label::new(-180., 60., 6., 5, 1, 1.5, "T5".to_string()));
    /// v.push(label::Label::new(175., 70., 5., 6, 1, 1.5, "T6".to_string()));
    /// v.push(label::Label::new(170., -80., 4., 7, 1, 1.5, "T7".to_string()));
    /// v.push(label::Label::new(160., 90., 3., 8, 1, 1.5, "T8".to_string()));
    ///
    /// let t = pst_3d::GeoPst3d::new(v);
    ///
    /// let bb = bbox::BBox::new(170., 0., -170., 90.);
    /// let r = t.get(&bb, 1.);
    ///
    /// for e in &r {
    ///   println!("{}, ", e.to_string());
    /// }
    ///
    /// // resulting labels are T2 to  T6 without T3
    /// assert!(r.len() == 4);
    /// ```
    ///
    pub fn get<'a>(&'a self, bbox: &BBox, min_t: f64) -> Vec<&'a Label> {
        assert!(bbox.get_min_y() <= bbox.get_max_y());
        assert!(bbox.get_min_y() >= -90. && bbox.get_max_y() <= 90.);
        assert!(bbox.get_min_x() >= -180. && bbox.get_min_x() <= 180.);
        assert!(bbox.get_max_x() >= -180. && bbox.get_max_x() <= 180.);

        if bbox.get_max_x() < bbox.get_min_x() {
            let mut res = self.m_pst
                .get(&BBox::new(bbox.get_min_x(), bbox.get_min_y(), 180., bbox.get_max_y()),
                     min_t);
            res.append(&mut self.m_pst
                                .get(&BBox::new(-180.,
                                                bbox.get_min_y(),
                                                bbox.get_max_x(),
                                                bbox.get_max_y()),
                                     min_t));

            return res;
        }

        self.m_pst.get(&bbox, min_t)
    }

    ///
    /// Create a human readable string representation of the tree.
    ///
    /// The function returns a multiline string with one row for each tree node. Large trees will
    /// produce a huge multiline string!
    ///
    /// # Examples
    /// ```
    /// use rt_datastructure::primitives::label;
    /// use rt_datastructure::pst_3d;
    ///
    /// let mut v = Vec::new();
    /// v.push(label::Label::new(1., 2., 10., 1, 1, 1.5, "T1".to_string()));
    /// v.push(label::Label::new(2., 3., 9., 2, 1, 1.5, "T2".to_string()));
    /// v.push(label::Label::new(3., 4., 8., 3, 1, 1.5, "T3".to_string()));
    /// v.push(label::Label::new(4., 5., 7., 4, 1, 1.5, "T4".to_string()));
    /// v.push(label::Label::new(5., 6., 6., 5, 1, 1.5, "T5".to_string()));
    /// v.push(label::Label::new(6., 7., 5., 6, 1, 1.5, "T6".to_string()));
    /// v.push(label::Label::new(7., 8., 4., 7, 1, 1.5, "T7".to_string()));
    /// v.push(label::Label::new(8., 9., 3., 8, 1, 1.5, "T8".to_string()));
    /// v.push(label::Label::new(9., 10., 2., 9, 1, 1.5, "T9".to_string()));
    /// v.push(label::Label::new(10., 11., 1., 10, 1, 1.5, "T10".to_string()));
    ///
    /// let t = pst_3d::Pst3d::new(v);
    ///
    /// let res_string = "\
    ///   x-node (split: 5): Label [#1]: 'T1' at (1, 2) with prio 1, elim-t: 10 and label \
    ///                                                                              factor: 1.5\n\
    ///   l    y-node (split: 4): Label [#2]: 'T2' at (2, 3) with prio 1, elim-t: 9 and label \
    ///                                                                              factor: 1.5\n\
    ///   l        x-node (split: NaN): Label [#3]: 'T3' at (3, 4) with prio 1, elim-t: 8 and \
    ///                                                                        label factor: 1.5\n\
    ///   r        x-node (split: 5): Label [#4]: 'T4' at (4, 5) with prio 1, elim-t: 7 and label \
    ///                                                                              factor: 1.5\n\
    ///   l            y-node (split: NaN): Label [#5]: 'T5' at (5, 6) with prio 1, elim-t: 6 and \
    ///                                                                        label factor: 1.5\n\
    ///   r    y-node (split: 9): Label [#6]: 'T6' at (6, 7) with prio 1, elim-t: 5 and label \
    ///                                                                              factor: 1.5\n\
    ///   l        x-node (split: 8): Label [#7]: 'T7' at (7, 8) with prio 1, elim-t: 4 and label \
    ///                                                                              factor: 1.5\n\
    ///   l            y-node (split: NaN): Label [#8]: 'T8' at (8, 9) with prio 1, elim-t: 3 and \
    ///                                                                        label factor: 1.5\n\
    ///   r        x-node (split: 10): Label [#9]: 'T9' at (9, 10) with prio 1, elim-t: 2 and \
    ///                                                                        label factor: 1.5\n\
    ///   l            y-node (split: NaN): Label [#10]: 'T10' at (10, 11) with prio 1, elim-t: 1 \
    ///                                                                      and label factor: 1.5\
    ///   ".to_string();
    ///
    /// println!("Tree after construction:\n{}", t.to_string());
    ///
    /// assert!(t.to_string() == res_string);
    /// ```
    ///
    pub fn to_string(&self) -> String {
        self.m_pst.to_string()
    }
}

///
/// A struct to store the 3d PST and provide a basic interface
///
pub struct Pst3d {
    m_bbox: BBox,

    m_data: Vec<Root>,
    m_root_idx: Option<usize>,
}

impl Pst3d {
    ///
    /// Initialize a new 3D PST from the given label vector.
    ///
    /// Take care: The procedure will consume the given vector!
    ///
    /// # Examples
    /// ```
    /// use rt_datastructure::primitives::label;
    /// use rt_datastructure::pst_3d;
    ///
    /// let mut v = Vec::new();
    /// v.push(label::Label::new(1., 2., 9., 1, 1, 1.5, "T1".to_string()));
    /// v.push(label::Label::new(2., 3., 8., 2, 1, 1.5, "T2".to_string()));
    /// v.push(label::Label::new(3., 4., 7., 3, 1, 1.5, "T3".to_string()));
    /// v.push(label::Label::new(3., 5., 7., 4, 1, 1.5, "T4".to_string()));
    /// v.push(label::Label::new(4., 6., 6., 5, 1, 1.5, "T5".to_string()));
    /// v.push(label::Label::new(5., 7., 5., 6, 1, 1.5, "T6".to_string()));
    /// v.push(label::Label::new(6., 8., 4., 7, 1, 1.5, "T7".to_string()));
    /// v.push(label::Label::new(7., 9., 3., 8, 1, 1.5, "T8".to_string()));
    /// v.push(label::Label::new(8., 10., 2., 9, 1, 1.5, "T9".to_string()));
    /// v.push(label::Label::new(9., 11., 1., 10, 1, 1.5, "T10".to_string()));
    ///
    /// let t = pst_3d::Pst3d::new(v.clone());
    /// ```
    ///
    pub fn new(mut labels: Vec<Label>) -> Pst3d {
        labels.sort_by(Label::order_t);
        labels.reverse();

        let mut v: Vec<Root> = Vec::with_capacity(labels.len());
        let mut bbox = BBox::new_empty();

        for mut l in labels {
            bbox.add_to_box(&mut l);

            v.push(Root::new(l));
        }

        let tree_root = Root::init_pst3d(&mut v);

        Pst3d {
            m_bbox: bbox,

            m_data: v,
            m_root_idx: tree_root,
        }
    }

    ///
    /// Return the set of label in the given bounding box with a t >= min_t.
    ///
    /// # Examples
    /// ```
    /// use rt_datastructure::primitives::{label, bbox};
    /// use rt_datastructure::pst_3d;
    ///
    /// let mut v = Vec::new();
    /// v.push(label::Label::new(1., 2., 10., 1, 1, 1.5, "T1".to_string()));
    /// v.push(label::Label::new(2., 3., 9., 2, 1, 1.5, "T2".to_string()));
    /// v.push(label::Label::new(3., 4., 8., 3, 1, 1.5, "T3".to_string()));
    /// v.push(label::Label::new(4., 5., 7., 4, 1, 1.5, "T4".to_string()));
    /// v.push(label::Label::new(5., 6., 6., 5, 1, 1.5, "T5".to_string()));
    /// v.push(label::Label::new(6., 7., 5., 6, 1, 1.5, "T6".to_string()));
    /// v.push(label::Label::new(7., 8., 4., 7, 1, 1.5, "T7".to_string()));
    /// v.push(label::Label::new(8., 9., 3., 8, 1, 1.5, "T8".to_string()));
    /// v.push(label::Label::new(9., 10., 2., 9, 1, 1.5, "T9".to_string()));
    /// v.push(label::Label::new(10., 11., 1., 10, 1, 1.5, "T10".to_string()));
    ///
    /// let t = pst_3d::Pst3d::new(v);
    ///
    /// let bb = bbox::BBox::new(4., 5., 7., 8.);
    /// let r = t.get(&bb, 4.);
    ///
    /// for e in &r {
    ///   println!("{}, ", e.to_string());
    /// }
    ///
    /// // resulting labels are T5 to  T7
    /// assert!(r.len() == 3);
    /// ```
    ///
    pub fn get<'a>(&'a self, bbox: &BBox, min_t: f64) -> Vec<&'a Label> {
        match self.m_root_idx {
            Some(idx) => self.m_data[idx].get(&bbox, min_t, &self.m_data),
            None => Vec::new(),
        }
    }

    ///
    /// Create a human readable string representation of the tree.
    ///
    /// The function returns a multiline string with one row for each tree node. Large trees will
    /// produce a huge multiline string!
    ///
    /// # Examples
    /// ```
    /// use rt_datastructure::primitives::label;
    /// use rt_datastructure::pst_3d;
    ///
    /// let mut v = Vec::new();
    /// v.push(label::Label::new(1., 2., 10., 1, 1, 1.5, "T1".to_string()));
    /// v.push(label::Label::new(2., 3., 9., 2, 1, 1.5, "T2".to_string()));
    /// v.push(label::Label::new(3., 4., 8., 3, 1, 1.5, "T3".to_string()));
    /// v.push(label::Label::new(4., 5., 7., 4, 1, 1.5, "T4".to_string()));
    /// v.push(label::Label::new(5., 6., 6., 5, 1, 1.5, "T5".to_string()));
    /// v.push(label::Label::new(6., 7., 5., 6, 1, 1.5, "T6".to_string()));
    /// v.push(label::Label::new(7., 8., 4., 7, 1, 1.5, "T7".to_string()));
    /// v.push(label::Label::new(8., 9., 3., 8, 1, 1.5, "T8".to_string()));
    /// v.push(label::Label::new(9., 10., 2., 9, 1, 1.5, "T9".to_string()));
    /// v.push(label::Label::new(10., 11., 1., 10, 1, 1.5, "T10".to_string()));
    ///
    /// let t = pst_3d::Pst3d::new(v);
    ///
    /// let res_string = "\
    ///   x-node (split: 5): Label [#1]: 'T1' at (1, 2) with prio 1, elim-t: 10 and label \
    ///                                                                              factor: 1.5\n\
    ///   l    y-node (split: 4): Label [#2]: 'T2' at (2, 3) with prio 1, elim-t: 9 and label \
    ///                                                                              factor: 1.5\n\
    ///   l        x-node (split: NaN): Label [#3]: 'T3' at (3, 4) with prio 1, elim-t: 8 and \
    ///                                                                        label factor: 1.5\n\
    ///   r        x-node (split: 5): Label [#4]: 'T4' at (4, 5) with prio 1, elim-t: 7 and label \
    ///                                                                              factor: 1.5\n\
    ///   l            y-node (split: NaN): Label [#5]: 'T5' at (5, 6) with prio 1, elim-t: 6 and \
    ///                                                                        label factor: 1.5\n\
    ///   r    y-node (split: 9): Label [#6]: 'T6' at (6, 7) with prio 1, elim-t: 5 and label \
    ///                                                                              factor: 1.5\n\
    ///   l        x-node (split: 8): Label [#7]: 'T7' at (7, 8) with prio 1, elim-t: 4 and label \
    ///                                                                              factor: 1.5\n\
    ///   l            y-node (split: NaN): Label [#8]: 'T8' at (8, 9) with prio 1, elim-t: 3 and \
    ///                                                                        label factor: 1.5\n\
    ///   r        x-node (split: 10): Label [#9]: 'T9' at (9, 10) with prio 1, elim-t: 2 and \
    ///                                                                        label factor: 1.5\n\
    ///   l            y-node (split: NaN): Label [#10]: 'T10' at (10, 11) with prio 1, elim-t: 1 \
    ///                                                                      and label factor: 1.5\
    ///   ".to_string();
    ///
    /// println!("Tree after construction:\n{}", t.to_string());
    ///
    /// assert!(t.to_string() == res_string);
    /// ```
    ///
    pub fn to_string(&self) -> String {
        match self.m_root_idx {
            Some(idx) => self.m_data[idx].to_string(0, &self.m_data),
            None => "PSKdT is empty!".to_string(),
        }
    }
}
