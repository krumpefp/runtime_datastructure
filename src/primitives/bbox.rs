use std::f64;

use primitives::label::Label;

pub struct BBox {
    m_max_x: f64,
    m_max_y: f64,
    m_min_x: f64,
    m_min_y: f64,
}

impl BBox {
    pub fn new(min_x: f64, min_y: f64, max_x: f64, max_y: f64) -> BBox {
        assert!(max_x >= min_x);
        assert!(max_y >= min_y);
        BBox {
            m_max_x: max_x,
            m_max_y: max_y,
            m_min_x: min_x,
            m_min_y: min_y,
        }
    }
    pub fn new_empty() -> BBox {
        BBox {
            m_max_x: f64::NEG_INFINITY,
            m_max_y: f64::NEG_INFINITY,
            m_min_x: f64::INFINITY,
            m_min_y: f64::INFINITY,
        }
    }

    pub fn new_from_point(l: &Label) -> BBox {
        BBox {
            m_max_x: l.get_x(),
            m_max_y: l.get_y(),
            m_min_x: l.get_x(),
            m_min_y: l.get_y(),
        }
    }

    pub fn add_to_box(&mut self, l: &Label) {
        self.m_max_x = self.m_max_x.max(l.get_x());
        self.m_max_y = self.m_max_y.max(l.get_y());
        self.m_min_x = self.m_max_x.min(l.get_x());
        self.m_min_y = self.m_max_y.min(l.get_y());
    }

    pub fn add_box(&mut self, other_box: &Self) {
        self.m_max_x = self.m_max_x.max(other_box.m_max_x);
        self.m_max_y = self.m_max_y.max(other_box.m_max_y);
        self.m_min_x = self.m_max_x.min(other_box.m_min_x);
        self.m_min_y = self.m_max_y.min(other_box.m_min_y);
    }
    
    pub fn get_max_x(&self) -> f64 {
        self.m_max_x
    }
    
    pub fn get_max_y(&self) -> f64 {
        self.m_max_y
    }
    
    pub fn get_min_x(&self) -> f64 {
        self.m_min_x
    }
    
    pub fn get_min_y(&self) -> f64 {
        self.m_min_y
    }
    
    pub fn is_contained(&self, l : &Label) -> bool {
        let x_in = l.get_x() <= self.m_max_x && l.get_x() >= self.m_min_x;
        let y_in = l.get_y() <= self.m_max_y && l.get_y() >= self.m_min_y;
        
        x_in && y_in
    }
    
    pub fn to_string(&self) -> String {
        format!("[x: {} - {}, y: {} - {}]", self.m_min_x, self.m_max_x, self.m_min_y, self.m_max_y)
    }
}
