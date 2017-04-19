use std::cmp::Ordering;

pub struct Label {
    m_x: f64,
    m_y: f64,
    m_t: f64,

    m_osm_id: i64,
    m_prio: i32,

    m_label: String,
}

impl Label {
    pub fn new(x: f64, y: f64, t: f64, osm_id: i64, prio: i32, label: String) -> Label {
        Label {
            m_x: x,
            m_y: y,
            m_t: t,
            m_osm_id: osm_id,
            m_prio: prio,
            m_label: label,
        }
    }

    pub fn get_label(&self) -> &String {
        &self.m_label
    }

    pub fn get_osm_id(&self) -> i64 {
        self.m_osm_id
    }

    pub fn get_prio(&self) -> i32 {
        self.m_prio
    }

    pub fn get_t(&self) -> f64 {
        self.m_t
    }

    pub fn get_x(&self) -> f64 {
        self.m_x
    }

    pub fn get_y(&self) -> f64 {
        self.m_y
    }


    /// This function compares two pois with respect to their y coordinate
    ///
    /// # Examples
    /// ```
    /// use std::cmp::Ordering;
    /// use runtime_datastructure::primitives::label::Label as Label;
    ///
    /// let p1 : Label = Label::new(90., 90., 0.9, 1234567, 16, "Test1".to_string());
    /// let p2 : Label = Label::new(90., 90., 0.8, 1234568, 16, "Test2".to_string());
    /// let p3 : Label = Label::new(90., 90., 0.8, 1234569, 16, "Test3".to_string());
    /// assert!(Label::order_t(&p1, &p2) == Ordering::Greater);
    /// assert!(Label::order_t(&p2, &p3) == Ordering::Equal);
    /// assert!(Label::order_t(&p3, &p1) == Ordering::Less);
    /// ```
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

    /// This function compares two pois with respect to their x coordinate
    ///
    /// # Examples
    /// ```
    /// use std::cmp::Ordering;
    /// use runtime_datastructure::primitives::label::Label as Label;
    ///
    /// let p1 : Label = Label::new(90., 90., 0.9, 1234567, 16, "Test1".to_string());
    /// let p2 : Label = Label::new(45., 90., 0.9, 1234568, 16, "Test2".to_string());
    /// let p3 : Label = Label::new(45., 90., 0.9, 1234569, 16, "Test3".to_string());
    /// assert!(Label::order_x(&p1, &p2) == Ordering::Greater);
    /// assert!(Label::order_x(&p2, &p3) == Ordering::Equal);
    /// assert!(Label::order_x(&p3, &p1) == Ordering::Less);
    /// ```
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


    /// This function compares two pois with respect to their y coordinate
    ///
    /// # Examples
    /// ```
    /// use std::cmp::Ordering;
    /// use runtime_datastructure::primitives::label::Label as Label;
    ///
    /// let p1 : Label = Label::new(90., 90., 0.9, 1234567, 16, "Test1".to_string());
    /// let p2 : Label = Label::new(90., 45., 0.9, 1234568, 16, "Test2".to_string());
    /// let p3 : Label = Label::new(90., 45., 0.9, 1234569, 16, "Test3".to_string());
    /// assert!(Label::order_y(&p1, &p2) == Ordering::Greater);
    /// assert!(Label::order_y(&p2, &p3) == Ordering::Equal);
    /// assert!(Label::order_y(&p3, &p1) == Ordering::Less);
    /// ```
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

    pub fn to_string(&self) -> String {
        format!("Label [#{}]: '{}' at ({}, {}) with prio {} and elim-t: {}",
                self.m_osm_id,
                self.m_label,
                self.m_x,
                self.m_y,
                self.m_prio,
                self.m_t)
    }
}

impl Clone for Label {
    fn clone(&self) -> Self {
        Self::new(self.m_x,
                  self.m_y,
                  self.m_t,
                  self.m_osm_id,
                  self.m_prio,
                  self.m_label.clone())
    }
}
