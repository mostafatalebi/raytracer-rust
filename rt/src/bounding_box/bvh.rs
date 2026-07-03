use std::cmp::{Ordering, PartialEq};
use std::collections::HashMap;
use crate::bounding_box::aabb::{Bounded, AABB};
use crate::common::volume::Centroid;
use crate::common::enums::TraversalReturn;
use crate::common::id::Id;
use crate::ray::ray_context::RayContext;
use crate::vector::vec3f::Vec3f;

#[derive(Default, Clone)]
pub struct BvhNode {
    bb: AABB,
    centroid: Vec3f,
    // index in the scene's list
    index: usize,
    objects: Vec<usize>,
    triangles: Vec<usize>,
    is_leaf: bool,
    left: Option<Box<BvhNode>>,
    right: Option<Box<BvhNode>>,
}



impl BvhNode {

    pub fn create<T>(mut primitives: &mut [T], leaf_size: usize) -> Self where T: Centroid + Bounded + Id {
        let mut index_tbl: HashMap<String, usize> = HashMap::new();
        for (k, obj) in primitives.iter().enumerate() {
            index_tbl.insert(obj.get_id(),  k);
        }


        let mut bvh_node: BvhNode;
        if primitives.len() > leaf_size {
            bvh_node = Self::make_tree(&mut primitives, leaf_size, &index_tbl);
            bvh_node.is_leaf = false;
        } else {
            bvh_node = BvhNode::default();
            bvh_node.is_leaf = true;
            bvh_node.surround(primitives, &index_tbl);
        }

        bvh_node
    }

    fn make_tree<T>(primitives: &mut [T], leaf_size: usize, index_tbl: &HashMap<String, usize>) -> BvhNode where T: Centroid+ Bounded  + Id{


        let mut bvh_node = BvhNode::default();
        bvh_node.surround(primitives, &index_tbl);

        // we sort by longest axis
        let longest_axis = bvh_node.bb.get_longest_axis();

        primitives.sort_by(|a, b| {
            if &a.get_centroid()[longest_axis] < &b.get_centroid()[longest_axis] {
                return Ordering::Less;
            }
            Ordering::Greater
        });

        if primitives.len() > leaf_size {
            let mid = primitives.len() / 2;
            bvh_node.left = Some(Box::new(Self::make_tree(&mut primitives[..mid], leaf_size, &index_tbl)));
            bvh_node.right = Some(Box::new(Self::make_tree(&mut primitives[mid..], leaf_size, &index_tbl)));
        } else {
            bvh_node.is_leaf = true;
        }

        bvh_node
    }




    pub fn new_simple(bb: AABB, centroid: Vec3f, index: usize) -> Self {
        Self {
            bb, centroid, index,
            objects: vec![],
            triangles: vec![],
            is_leaf: false,
            left: None, right: None
        }
    }

    pub fn surround<T>(&mut self, objects: &[T], index_tbl: &HashMap<String, usize>) where T: Centroid + Bounded + Id {
        let mut bb = AABB::default();
        for (k, obj) in objects.iter().enumerate() {
            bb = bb.expand(&obj.get_bb());
            self.objects.push(*index_tbl.get(&obj.get_id()).unwrap());
        }
        self.centroid = bb.get_centroid();
        self.bb = bb;
    }


    pub fn inorder_traversal<F: FnMut(&mut RayContext, bool, &AABB, &Vec<usize>) -> TraversalReturn >(&self, rc: &mut RayContext, f: &mut F) {
        Self::start_inorder_traversal(rc, &Some(self), f)
    }


    /// this recursive traversal function exists when the closure returns false.
    /// the closure is passed the current bounding box and the list of objects
    /// inside of it. It can do the tests and decide to continue or not.
    /// closure: isLeaf, BoundingVolume, Option<Objects>
    /// it is recommended do not do object-based tests until isLead=true
    fn start_inorder_traversal<F: FnMut(&mut RayContext, bool, &AABB, &Vec<usize>) -> TraversalReturn>(rc: &mut RayContext, bvh_node: &Option<&BvhNode>, mut f: &mut F) {
        let nn = bvh_node.as_ref().unwrap();
        if nn.is_leaf == false {
            let test_result = (*f)(rc, false, &nn.bb, &nn.objects);
            if test_result == TraversalReturn::Continue {
                if nn.left.is_some() {
                    Self::start_inorder_traversal(rc, &Some(nn.left.as_ref().unwrap()), f);
                }
                if nn.right.is_some() {
                    Self::start_inorder_traversal(rc, &Some(nn.right.as_ref().unwrap()), f);
                }
            }
        } else {
            f(rc, true, &nn.bb, &nn.objects);
        }
    }
}




#[cfg(test)]
mod tests {
    use rand::random;
    use crate::bounding_box::bvh::BvhNode;
    use crate::geometry::helpers::create_cube;

    #[test]
    fn test_bvh() {
        let cube1 = create_cube(10.0, 5.0, 3.0);
        let bvh = BvhNode::create(&mut [cube1][..], 2);
        assert_eq!(bvh.is_leaf, true);
        let cube1 = create_cube(10.0, 5.0, 3.0);
        let cube2 = create_cube(20.0, 5.0, 3.0);
        let bvh = BvhNode::create(&mut [cube1, cube2][..], 2);
        assert_eq!(bvh.is_leaf, true);
        let cube1 = create_cube(10.0, 5.0, 3.0);
        let cube2 = create_cube(20.0, 5.0, 3.0);
        let cube3 = create_cube(20.0, 5.0, 3.0);
        let bvh = BvhNode::create(&mut [cube1, cube2, cube3][..], 2);
        assert_eq!(bvh.is_leaf, false);

        let cube1 = create_cube(random::<f64>()*100.0, random::<f64>()*100.0, random::<f64>()*100.0);
        let cube2 = create_cube(random::<f64>()*100.0, random::<f64>()*100.0, random::<f64>()*100.0);
        let cube3 = create_cube(random::<f64>()*100.0, random::<f64>()*100.0, random::<f64>()*100.0);
        let cube4 = create_cube(random::<f64>()*100.0, random::<f64>()*100.0, random::<f64>()*100.0);
        let cube5 = create_cube(random::<f64>()*100.0, random::<f64>()*100.0, random::<f64>()*100.0);
        let cube6 = create_cube(random::<f64>()*100.0, random::<f64>()*100.0, random::<f64>()*100.0);
        let cube7 = create_cube(random::<f64>()*100.0, random::<f64>()*100.0, random::<f64>()*100.0);
        let bvh = BvhNode::create(&mut [cube1, cube2, cube3, cube4, cube5, cube6, cube7][..], 2);
        assert_eq!(bvh.is_leaf, false);

        assert_eq!(bvh.left.is_some(), true);
        assert_eq!(bvh.right.is_some(), true);
    }
}