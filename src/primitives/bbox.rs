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

use primitives::label::Label;


///
/// The struct defines an axis aligned rectangular area in 2 dimension via min and max in each
/// dimension X and Y.
///
#[derive(Debug)]
pub struct BBox {
    m_max_x: f64,
    m_max_y: f64,
    m_min_x: f64,
    m_min_y: f64,
}

impl BBox {
    ///
    /// Initialize a new Bounding Box with the given values.
    ///
    ///
    /// # Examples
    /// ```
    /// use rt_datastructure::primitives::{bbox, label};
    ///
    /// let bb = bbox::BBox::new(0., 0., 1., 1.);
    /// let not_contained = label::Label::new(-1., -1., 0., 0, 0, 1., "Not contained".to_string());
    /// let at_the_border = label::Label::new(0., 0., 0., 0, 0, 1., "Not contained".to_string());
    /// let contained = label::Label::new(0.5, 0.5, 0., 0, 0, 1., "Not contained".to_string());
    ///
    /// assert!(bb.is_contained(&contained));
    /// assert!(bb.is_contained(&at_the_border));
    /// assert!(!bb.is_contained(&not_contained));
    /// ```
    ///
    pub fn new(min_x: f64, min_y: f64, max_x: f64, max_y: f64) -> BBox {
        BBox {
            m_max_x: max_x,
            m_max_y: max_y,
            m_min_x: min_x,
            m_min_y: min_y,
        }
    }

    ///
    /// Initialize a new empty bounding box.
    ///
    /// An empty bounding box might be used to construct a bounding box that spans a set of labels
    /// or boudning boxes or a mixture of both.
    ///
    /// For any Label l, is_contained will return false.
    ///
    ///
    /// # Examples
    /// ```
    /// use rt_datastructure::primitives::{bbox, label};
    ///
    /// let bb = bbox::BBox::new_empty();
    /// let l = label::Label::new(0., 0., 0., 0, 0, 1., "Not contained".to_string());
    ///
    /// assert!(!bb.is_contained(&l));
    /// ```
    ///
    pub fn new_empty() -> BBox {
        BBox {
            m_max_x: f64::NEG_INFINITY,
            m_max_y: f64::NEG_INFINITY,
            m_min_x: f64::INFINITY,
            m_min_y: f64::INFINITY,
        }
    }

    ///
    /// Initialize a new bounding box that contains the given label L
    ///
    /// For any Label not located at L, is_contained will return false.
    ///
    ///
    /// # Examples
    /// ```
    /// use rt_datastructure::primitives::{bbox, label};
    ///
    /// let L = label::Label::new(0., 0., 0., 0, 0, 1., "Defining label".to_string());
    /// let bb = bbox::BBox::new_from_point(&L);
    /// let contained = label::Label::new(0., 0., 0., 0, 0, 1., "Contained".to_string());
    /// let not_contained = label::Label::new(1., 0., 0., 0, 0, 1., "Not contained".to_string());
    ///
    /// assert!(bb.is_contained(&L));
    /// assert!(bb.is_contained(&contained));
    /// assert!(!bb.is_contained(&not_contained));
    /// ```
    ///
    pub fn new_from_point(l: &Label) -> BBox {
        BBox {
            m_max_x: l.get_x(),
            m_max_y: l.get_y(),
            m_min_x: l.get_x(),
            m_min_y: l.get_y(),
        }
    }

    ///
    /// Adapt the box to also contain the given point.
    ///
    ///
    /// # Examples
    /// ```
    /// use rt_datastructure::primitives::{bbox, label};
    ///
    /// let mut bb = bbox::BBox::new(0., 0., 1., 1.);
    /// let tba = label::Label::new(-1., -1., 0., 0, 0, 1., "To be added".to_string());
    /// assert!(!bb.is_contained(&tba));
    /// bb.add_to_box(&tba);
    /// assert!(bb.is_contained(&tba));
    /// ```
    ///
    pub fn add_to_box(&mut self, l: &Label) {
        self.m_max_x = self.m_max_x.max(l.get_x());
        self.m_max_y = self.m_max_y.max(l.get_y());
        self.m_min_x = self.m_max_x.min(l.get_x());
        self.m_min_y = self.m_max_y.min(l.get_y());
    }

    ///
    /// Adapt the box to also contains to also span the given box.
    ///
    ///
    /// # Examples
    /// ```
    /// use rt_datastructure::primitives::{bbox, label};
    ///
    /// let mut bb = bbox::BBox::new(-1., -1., 0., 0.);
    ///
    /// let bb1 = bbox::BBox::new(0., 0., 1., 1.);
    /// let c1 = label::Label::new(0.5, 0.5, 0., 0, 0, 1., "Contained in 1".to_string());
    /// assert!(bb1.is_contained(&c1));
    /// assert!(!bb.is_contained(&c1));
    ///
    /// bb.add_box(&bb1);
    /// assert!(bb.is_contained(&c1));
    /// ```
    ///
    pub fn add_box(&mut self, other_box: &Self) {
        self.m_max_x = self.m_max_x.max(other_box.m_max_x);
        self.m_max_y = self.m_max_y.max(other_box.m_max_y);
        self.m_min_x = self.m_max_x.min(other_box.m_min_x);
        self.m_min_y = self.m_max_y.min(other_box.m_min_y);
    }

    ///
    /// Get the maximum x value of the bbox.
    ///
    ///
    /// # Examples
    /// ```
    /// use rt_datastructure::primitives::bbox;
    ///
    /// let bb = bbox::BBox::new(1., 2., 3., 4.);
    /// assert!(bb.get_max_x() == 3.);
    /// ```
    ///
    pub fn get_max_x(&self) -> f64 {
        self.m_max_x
    }

    ///
    /// Get the maximum y value of the bbox.
    ///
    ///
    /// # Examples
    /// ```
    /// use rt_datastructure::primitives::bbox;
    ///
    /// let bb = bbox::BBox::new(1., 2., 3., 4.);
    /// assert!(bb.get_max_y() == 4.);
    /// ```
    ///
    pub fn get_max_y(&self) -> f64 {
        self.m_max_y
    }

    ///
    /// Get the minimum x value of the bbox.
    ///
    ///
    /// # Examples
    /// ```
    /// use rt_datastructure::primitives::bbox;
    ///
    /// let bb = bbox::BBox::new(1., 2., 3., 4.);
    /// assert!(bb.get_min_x() == 1.);
    /// ```
    ///
    pub fn get_min_x(&self) -> f64 {
        self.m_min_x
    }

    ///
    /// Get the maximum x value of the bbox.
    ///
    ///
    /// # Examples
    /// ```
    /// use rt_datastructure::primitives::bbox;
    ///
    /// let bb = bbox::BBox::new(1., 2., 3., 4.);
    /// assert!(bb.get_min_y() == 2.);
    /// ```
    ///
    pub fn get_min_y(&self) -> f64 {
        self.m_min_y
    }

    ///
    /// Check if a label is contained in the bounding box or not.
    ///
    /// A label is contained in the box if its coordinates are >= min and <= max of the respective
    /// dimension.
    ///
    ///
    /// # Examples
    /// ```
    /// use rt_datastructure::primitives::{bbox, label};
    ///
    /// let bb = bbox::BBox::new(-1., -1., 1., 1.);
    /// let on_border = label::Label::new(-1., -1., 0., 0, 0, 1., "On border".to_string());
    /// let contained = label::Label::new(0., 0., 0., 0, 0, 1., "Contained".to_string());
    /// let not_contained = label::Label::new(-1., -2., 0., 0, 0, 1., "Not contained".to_string());
    ///
    /// assert!(bb.is_contained(&on_border));
    /// assert!(bb.is_contained(&contained));
    /// assert!(!bb.is_contained(&not_contained));
    /// ```
    ///
    pub fn is_contained(&self, l: &Label) -> bool {
        let x_in = l.get_x() <= self.m_max_x && l.get_x() >= self.m_min_x;
        let y_in = l.get_y() <= self.m_max_y && l.get_y() >= self.m_min_y;

        x_in && y_in
    }

    ///
    /// Output the given bounding box to a human readable string
    ///
    ///
    /// # Examples
    /// ```
    /// use rt_datastructure::primitives::bbox;
    ///
    /// let bb = bbox::BBox::new(1., 2., 3., 4.);
    /// let s = bb.to_string();
    ///
    /// assert!(s == "[x: 1 - 3, y: 2 - 4]".to_string());
    /// ```
    ///
    pub fn to_string(&self) -> String {
        format!("[x: {} - {}, y: {} - {}]",
                self.m_min_x,
                self.m_max_x,
                self.m_min_y,
                self.m_max_y)
    }

    ///
    /// Check that the bounding box contains valid a coordinate range
    ///
    /// Valid coordinates have y-values in the range [-90,90] and
    /// x-values in the range [-180,180]
    ///
    /// # Examples
    /// ```
    /// use rt_datastructure::primitives::bbox;
    ///
    /// let bb = bbox::BBox::new(1., 2., 3., 4.);
    /// assert!(bb.check_coordinate_consistency().is_ok());
    ///
    /// let bb = bbox::BBox::new(10., -92., 3., 4.);
    /// assert!(bb.check_coordinate_consistency().is_err());
    /// ```
    pub fn check_coordinate_consistency(&self) -> Result<(), &'static str> {
        // Added .1 to be safe against floating point imprecision
        if self.m_min_y >= self.m_max_y {
            return Err("lower y bound is greater than upper y bound");
        } else if self.m_min_y <= -90.1 || self.m_max_y >= 90.1 {
            return Err("y values have to be in the range [-90.0, 90.0]");
        } else if (self.m_min_x <= -180.1 || self.m_min_x >= 180.1) ||
                  (self.m_max_x <= -180.1 || self.m_max_x >= 180.1) {
            return Err("x values have to be in the range [-180.0, 180.0]");
        }
        Ok(())
    }
}
