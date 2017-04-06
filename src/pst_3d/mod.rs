mod root;

use primitives::label::Label;
use primitives::bbox::BBox;

use self::root::Root;

pub struct PST_3D {
    m_bbox: BBox,
    
    m_data: Vec<Root>,
    m_root_idx : usize,
}

impl PST_3D {
    pub fn new(mut labels : Vec<Label>) -> PST_3D {
        labels.sort_by(Label::order_t);
        labels.reverse();
        
        let mut v : Vec<Root> = Vec::with_capacity(labels.len());
        let mut bbox = BBox::new_empty();
        
        for mut l in labels {
            bbox.add_to_box(&mut l);
            
            v.push(Root::new(l));
        }
        
        let tree_root = Root::init_PST_3D(&mut v);
        
        PST_3D {
            m_bbox : bbox,
            
            m_data : v,
            m_root_idx : tree_root,
        }
    }
    
    pub fn get<'a>(&'a self, bbox : &BBox, min_t : f64) -> Vec<&'a Label> {
        self.m_data[self.m_root_idx].get(&bbox, min_t, &self.m_data)
    }
    
    pub fn to_string(&self) -> String {
        self.m_data[self.m_root_idx].to_string(0, &self.m_data)
    }
}
