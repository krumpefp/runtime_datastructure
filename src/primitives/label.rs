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

use std::cmp::Ordering;

///
/// The label class.
///
/// For more information about the members compare the [module description](../index.html)
///
pub struct Label {
    m_x: f64,
    m_y: f64,
    m_t: f64,

    m_osm_id: i64,
    m_prio: i32,

    m_lbl_fac: f64,
    m_label: String,
}

impl Label {
    ///
    /// Initialize a new label object with the given data.
    ///
    ///
    /// # Examples
    /// ```
    /// use rt_datastructure::primitives::label;
    ///
    /// let l = label::Label::new(90., 90., 0.9, 1234567, 16, 1.5, "Test".to_string());
    /// ```
    ///
    pub fn new(x: f64, y: f64, t: f64, osm_id: i64, prio: i32, lbl_f: f64, label: String) -> Label {
        Label {
            m_x: x,
            m_y: y,
            m_t: t,
            m_osm_id: osm_id,
            m_prio: prio,
            m_lbl_fac: lbl_f,
            m_label: label,
        }
    }

    ///
    /// Get the label factor.
    ///
    ///
    /// # Examples
    /// ```
    /// use rt_datastructure::primitives::label;
    ///
    /// let l = label::Label::new(90., 45., 0.9, 1234567, 16, 1.5, "Test".to_string());
    ///
    /// assert!(l.get_label_factor() == 1.5);
    /// ```
    ///
    pub fn get_label_factor(&self) -> f64 {
        self.m_lbl_fac
    }

    ///
    /// Get the label string.
    ///
    ///
    /// # Examples
    /// ```
    /// use rt_datastructure::primitives::label;
    ///
    /// let l = label::Label::new(90., 45., 0.9, 1234567, 16, 1.5, "Test".to_string());
    ///
    /// assert!(*l.get_label() == "Test".to_string());
    /// ```
    ///
    pub fn get_label(&self) -> &String {
        &self.m_label
    }


    ///
    /// Get the osm_id.
    ///
    ///
    /// # Examples
    /// ```
    /// use rt_datastructure::primitives::label;
    ///
    /// let l = label::Label::new(90., 45., 0.9, 1234567, 16, 1.5, "Test".to_string());
    ///
    /// assert!(l.get_osm_id() == 1234567);
    /// ```
    ///
    pub fn get_osm_id(&self) -> i64 {
        self.m_osm_id
    }


    ///
    /// Get the label priority.
    ///
    ///
    /// # Examples
    /// ```
    /// use rt_datastructure::primitives::label;
    ///
    /// let l = label::Label::new(90., 45., 0.9, 1234567, 16, 1.5, "Test".to_string());
    ///
    /// assert!(l.get_prio() == 16);
    /// ```
    ///
    pub fn get_prio(&self) -> i32 {
        self.m_prio
    }


    ///
    /// Get the t value.
    ///
    ///
    /// # Examples
    /// ```
    /// use rt_datastructure::primitives::label;
    ///
    /// let l = label::Label::new(90., 45., 0.9, 1234567, 16, 1.5, "Test".to_string());
    ///
    /// assert!(l.get_t() == 0.9);
    /// ```
    ///
    pub fn get_t(&self) -> f64 {
        self.m_t
    }


    ///
    /// Get the x coordinate.
    ///
    ///
    /// # Examples
    /// ```
    /// use rt_datastructure::primitives::label;
    ///
    /// let l = label::Label::new(90., 45., 0.9, 1234567, 16, 1.5, "Test".to_string());
    ///
    /// assert!(l.get_x() == 90.);
    /// ```
    ///
    pub fn get_x(&self) -> f64 {
        self.m_x
    }


    ///
    /// Get the y coordinate.
    ///
    ///
    /// # Examples
    /// ```
    /// use rt_datastructure::primitives::label;
    ///
    /// let l = label::Label::new(90., 45., 0.9, 1234567, 16, 1.5, "Test".to_string());
    ///
    /// assert!(l.get_y() == 45.);
    /// ```
    ///
    pub fn get_y(&self) -> f64 {
        self.m_y
    }

    ///
    /// This function compares two pois with respect to their y coordinate
    ///
    /// # Examples
    /// ```
    /// use std::cmp::Ordering;
    /// use rt_datastructure::primitives::label;
    ///
    /// let p1 = label::Label::new(90., 90., 0.9, 1234567, 16, 1.5, "Test1".to_string());
    /// let p2 = label::Label::new(90., 90., 0.8, 1234568, 16, 1.5, "Test2".to_string());
    /// let p3 = label::Label::new(90., 90., 0.8, 1234569, 16, 1.5, "Test3".to_string());
    /// assert!(label::Label::order_t(&p1, &p2) == Ordering::Greater);
    /// assert!(label::Label::order_t(&p2, &p3) == Ordering::Equal);
    /// assert!(label::Label::order_t(&p3, &p1) == Ordering::Less);
    /// ```
    ///
    pub fn order_t(first: &Self, second: &Self) -> Ordering {
        let t_first = first.m_t;
        let t_second = second.m_t;
        if t_first < t_second {
            Ordering::Less
        } else if t_first > t_second {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }

    ///
    /// This function compares two pois with respect to their x coordinate
    ///
    /// # Examples
    /// ```
    /// use std::cmp::Ordering;
    /// use rt_datastructure::primitives::label;
    ///
    /// let p1 = label::Label::new(90., 90., 0.9, 1234567, 16, 1.5, "Test1".to_string());
    /// let p2 = label::Label::new(45., 90., 0.9, 1234568, 16, 1.5, "Test2".to_string());
    /// let p3 = label::Label::new(45., 90., 0.9, 1234569, 16, 1.5, "Test3".to_string());
    /// assert!(label::Label::order_x(&p1, &p2) == Ordering::Greater);
    /// assert!(label::Label::order_x(&p2, &p3) == Ordering::Equal);
    /// assert!(label::Label::order_x(&p3, &p1) == Ordering::Less);
    /// ```
    ///
    pub fn order_x(first: &Self, second: &Self) -> Ordering {
        let x_first = first.m_x;
        let x_second = second.m_x;
        if x_first < x_second {
            Ordering::Less
        } else if x_first > x_second {
            Ordering::Greater
        } else {
            Ordering::Equal
        }

    }

    ///
    /// This function compares two pois with respect to their y coordinate
    ///
    /// # Examples
    /// ```
    /// use std::cmp::Ordering;
    /// use rt_datastructure::primitives::label;
    ///
    /// let p1 = label::Label::new(90., 90., 0.9, 1234567, 16, 1.5, "Test1".to_string());
    /// let p2 = label::Label::new(90., 45., 0.9, 1234568, 16, 1.5, "Test2".to_string());
    /// let p3 = label::Label::new(90., 45., 0.9, 1234569, 16, 1.5, "Test3".to_string());
    /// assert!(label::Label::order_y(&p1, &p2) == Ordering::Greater);
    /// assert!(label::Label::order_y(&p2, &p3) == Ordering::Equal);
    /// assert!(label::Label::order_y(&p3, &p1) == Ordering::Less);
    /// ```
    ///
    pub fn order_y(first: &Self, second: &Self) -> Ordering {
        let y_first = first.m_y;
        let y_second = second.m_y;
        if y_first < y_second {
            Ordering::Less
        } else if y_first > y_second {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }

    ///
    /// This function compares two pois with respect to their y coordinate
    ///
    /// # Examples
    /// ```
    /// use rt_datastructure::primitives::label;
    ///
    /// let l = label::Label::new(90., 45., 0.9, 1234567, 16, 1.5, "Test".to_string());
    ///
    /// let s = l.to_string();
    /// assert!(s == "Label [#1234567]: 'Test' at (90, 45) with prio 16, elim-t: 0.9 and label \
    ///                                                                               factor: 1.5")
    /// ```
    ///
    pub fn to_string(&self) -> String {
        format!("Label [#{}]: '{}' at ({}, {}) with prio {}, elim-t: {} and label factor: {}",
                self.m_osm_id,
                self.m_label,
                self.m_x,
                self.m_y,
                self.m_prio,
                self.m_t,
                self.m_lbl_fac)
    }
}

impl Clone for Label {
    fn clone(&self) -> Self {
        Self::new(self.m_x,
                  self.m_y,
                  self.m_t,
                  self.m_osm_id,
                  self.m_prio,
                  self.m_lbl_fac,
                  self.m_label.clone())
    }
}
