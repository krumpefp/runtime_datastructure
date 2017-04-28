///
/// Implements a tree node
///
mod root;

use primitives::label::Label;
use primitives::bbox::BBox;

use self::root::Root;

///
/// A struct to store the 3d PST and provide a basic interface
///
pub struct Pst3d {
    m_bbox: BBox,

    m_data: Vec<Root>,
    m_root_idx: usize,
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
    /// let t = pst_3d::Pst3d::new(v);
    ///
    /// let bb = bbox::BBox::new(4., 5., 7., 8.);
    /// let r = t.get(&bb, 4.);
    ///
    /// // resulting labels are T5 to  T7
    /// assert!(r.len() == 3);
    /// ```
    ///
    pub fn get<'a>(&'a self, bbox: &BBox, min_t: f64) -> Vec<&'a Label> {
        self.m_data[self.m_root_idx].get(&bbox, min_t, &self.m_data)
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
    /// let t = pst_3d::Pst3d::new(v);
    ///
    /// let res_string = "\
    ///   x-node (split: 4.5): Label [#1]: 'T1' at (1, 2) with prio 1, elim-t: 9 and label \
    ///                                                                              factor: 1.5\n\
    ///   l    y-node (split: 4.5): Label [#2]: 'T2' at (2, 3) with prio 1, elim-t: 8 and label \
    ///                                                                              factor: 1.5\n\
    ///   l        x-node (split: NaN): Label [#3]: 'T3' at (3, 4) with prio 1, elim-t: 7 and \
    ///                                                                        label factor: 1.5\n\
    ///   r        x-node (split: 4): Label [#4]: 'T4' at (3, 5) with prio 1, elim-t: 7 and label \
    ///                                                                              factor: 1.5\n\
    ///   l            y-node (split: NaN): Label [#5]: 'T5' at (4, 6) with prio 1, elim-t: 6 and \
    ///                                                                        label factor: 1.5\n\
    ///   r    y-node (split: 9.5): Label [#6]: 'T6' at (5, 7) with prio 1, elim-t: 5 and label \
    ///                                                                              factor: 1.5\n\
    ///   l        x-node (split: 7): Label [#7]: 'T7' at (6, 8) with prio 1, elim-t: 4 and label \
    ///                                                                              factor: 1.5\n\
    ///   l            y-node (split: NaN): Label [#8]: 'T8' at (7, 9) with prio 1, elim-t: 3 and \
    ///                                                                        label factor: 1.5\n\
    ///   r        x-node (split: 9): Label [#9]: 'T9' at (8, 10) with prio 1, elim-t: 2 and \
    ///                                                                        label factor: 1.5\n\
    ///   l            y-node (split: NaN): Label [#10]: 'T10' at (9, 11) with prio 1, elim-t: 1 \
    ///                                                                      and label factor: 1.5\
    ///   ".to_string();
    /// assert!(t.to_string() == res_string);
    /// ```
    ///
    pub fn to_string(&self) -> String {
        self.m_data[self.m_root_idx].to_string(0, &self.m_data)
    }
}
