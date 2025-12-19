use crate::bounding_box::BoundingBox;
use crate::primitives::{min_intersection, Primitive, Ray, RayIntersection};
use std::cmp::{self, Ordering};
const MIN_PRIMITIVES: usize = 5;
use log::{debug, info};

#[derive(Clone)]
struct BVHNode {
    primitives_at_level: Vec<usize>,
    is_terminal: bool,
    left: Option<Box<BVHNode>>,
    right: Option<Box<BVHNode>>,
    left_bounding_box: BoundingBox,
    right_bounding_box: BoundingBox,
}

#[derive(Clone)]
struct Bucket {
    count: i32,
    bound: BoundingBox,
    cost: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct BVHIntersectionResult {
    pub intersection: RayIntersection,
    pub primitive_idx: usize,
}

impl PartialEq<BVHIntersectionResult> for BVHIntersectionResult {
    fn eq(&self, other: &BVHIntersectionResult) -> bool {
        self.intersection == other.intersection
    }
}
impl PartialOrd<BVHIntersectionResult> for BVHIntersectionResult {
    fn partial_cmp(&self, other: &BVHIntersectionResult) -> Option<Ordering> {
        self.intersection.partial_cmp(&other.intersection)
    }
}

pub struct BVH {
    root: BVHNode,
    primitives: Vec<Primitive>
}

impl BVH {
    pub fn create(primitives: Vec<Primitive>) -> Self {
        let root = BVHNode::create(&primitives);
        BVH {
            root,
            primitives
        }
    }

    pub fn get_primitive(&self, idx: usize) -> &Primitive {
        &self.primitives[idx]
    }

    pub fn intersection(&self, r: &Ray) -> Option<BVHIntersectionResult> {
        self.root.intersection(&self.primitives, r)
    }

    pub fn print_traverse(&self) {
        self.root.print_traverse();
    }

}

impl BVHNode {
    pub fn create(primitives: &Vec<Primitive>) -> BVHNode {
        debug!("Length of primitives is: {}", primitives.len());
        BVHNode::recursive_helper(primitives, (0..primitives.len()).collect())
    }
    pub fn recursive_helper(
        primitives: &Vec<Primitive>,
        primitives_at_level: Vec<usize>,
    ) -> BVHNode {
        if primitives_at_level.len() <= MIN_PRIMITIVES {
            return BVHNode {
                primitives_at_level,
                is_terminal: true,
                left: None,
                right: None,
                left_bounding_box: BoundingBox::create_empty(),
                right_bounding_box: BoundingBox::create_empty(),
            };
        }
        let mut centroid_bounds = BoundingBox::create_empty();
        let mut total_bounds = BoundingBox::create_empty();

        for i in &primitives_at_level {
            centroid_bounds =
                BoundingBox::union_point(centroid_bounds, primitives[*i].bounds().centroid());
            total_bounds = BoundingBox::union(total_bounds, primitives[*i].bounds());
        }
        let dim = total_bounds.maximum_extent(); //NOTE: Hardcoding splitting dimension to y
        debug!("dim is: {} for total_bounds: {:?}", dim, total_bounds);
        let n_buckets = 12;
        let mut buckets = vec![
            Bucket {
                count: 0,
                bound: BoundingBox::create_empty(),
                cost: 0.0
            };
            n_buckets
        ];
        for i in &primitives_at_level {
            let b = (centroid_bounds.offset(primitives[*i].bounds().centroid())[dim]
                * (n_buckets as f32))
                .floor();
            //debug!("Centroid bounds offset: {:?}", centroid_bounds.offset(primitives[i as usize].bounds().centroid()));
            let b = cmp::min(b as i32, (n_buckets - 1) as i32);
            buckets[b as usize].bound =
                BoundingBox::union(buckets[b as usize].bound, primitives[*i].bounds());
            buckets[b as usize].count += 1;
        }
        let mut min_cost = f32::MAX;
        let mut min_index: i32 = -1;
        for i in 0..n_buckets {
            //debug!("Count of ith: {} bucket is: {}", i, buckets[i].count);
            let mut left = BoundingBox::create_empty();
            let mut right = BoundingBox::create_empty();
            let mut left_count = 0;
            let mut right_count = 0;

            for left_index in 0..i + 1 {
                left = BoundingBox::union(left, buckets[left_index].bound);
                left_count += buckets[left_index].count;
            }
            for right_index in i + 1..n_buckets {
                right = BoundingBox::union(right, buckets[right_index].bound);
                right_count += buckets[right_index].count;
            }
            buckets[i].cost = 0.125
                + (left.surface_area() * (left_count as f32)
                    + right.surface_area() * (right_count as f32))
                    / (total_bounds.surface_area());
            debug!("Cost of ith: {} bucket is: {}", i, buckets[i].cost);
            if min_cost > buckets[i].cost {
                min_cost = buckets[i].cost;
                min_index = i as i32;
            }
        }

        debug!("Min cost is: {} at index: {}", min_cost, min_index);

        for primitive in primitives.iter() {
            debug!(
                "Primitive bounds is: {:?}, primitive centroid is: {:?}",
                primitive.bounds(),
                primitive.bounds().centroid()
            );
            debug!(
                "Offset is: {:?}",
                centroid_bounds.offset(primitive.bounds().centroid())
            );
            debug!(
                "Index is: {}",
                (n_buckets as f32)
                    * centroid_bounds.offset(primitive.bounds().centroid())[dim].floor()
            );
        }
        let left_primitives: Vec<usize> = primitives_at_level
            .iter()
            .copied()
            .filter(|i| {
                ((n_buckets as f32)
                    * centroid_bounds.offset(primitives[*i].bounds().centroid())[dim])
                    .floor()
                    <= (min_index as f32)
            })
            .collect();
        let right_primitives: Vec<usize> = primitives_at_level
            .iter()
            .copied()
            .filter(|i| {
                ((n_buckets as f32)
                    * centroid_bounds.offset(primitives[*i].bounds().centroid())[dim])
                    .floor()
                    > (min_index as f32)
            })
            .collect();
        debug!("Centroid bounds: {:?}", centroid_bounds);
        debug!(
            "Left length is: {}, Right length is: {}, Primitives length is: {}",
            left_primitives.len(),
            right_primitives.len(),
            primitives.len()
        );
        debug_assert!((left_primitives.len() + right_primitives.len()) == primitives.len());
        let mut left_bounding_box = BoundingBox::create_empty();
        let mut right_bounding_box = BoundingBox::create_empty();
        for primitive_idx in left_primitives.iter() {
            left_bounding_box =
                BoundingBox::union(left_bounding_box, primitives[*primitive_idx].bounds());
        }
        for primitive_idx in right_primitives.iter() {
            right_bounding_box =
                BoundingBox::union(right_bounding_box, primitives[*primitive_idx].bounds());
        }
        if left_primitives.len() == primitives_at_level.len()
            || right_primitives.len() == primitives_at_level.len()
        {
            //No splitting occurring here
            debug!("Size of reduced sprimitives array is the same as the original - no splitting occurring");
            let primitives_at_level = if left_primitives.len() == primitives_at_level.len() {
                left_primitives
            } else {
                right_primitives
            };
            return BVHNode {
                primitives_at_level,
                is_terminal: true,
                left: None,
                right: None,
                left_bounding_box: BoundingBox::create_empty(),
                right_bounding_box: BoundingBox::create_empty(),
            };
            //panic!("Size of reduced sprimitives array is the same as the original - no splitting occurring");
        }
        let left_primitives_len = left_primitives.len();
        let right_primitives_len = right_primitives.len();

        let left_node = if left_primitives_len > 0 {
            Some(Box::new(BVHNode::recursive_helper(
                primitives,
                left_primitives,
            )))
        } else {
            None
        };
        let right_node = if right_primitives_len > 0 {
            Some(Box::new(BVHNode::recursive_helper(
                primitives,
                right_primitives,
            )))
        } else {
            None
        };
        BVHNode {
            primitives_at_level: vec![],
            is_terminal: false,
            left: left_node,
            right: right_node,
            left_bounding_box,
            right_bounding_box,
        }
    }

    pub fn intersection_helper(&self, primitives: &Vec<Primitive>, r: &Ray) -> (Option<BVHIntersectionResult>, usize) {
        let mut intersection_count = 0;
        if self.is_terminal {
            let mut min_intersection_v: Option<BVHIntersectionResult> = None;
            //debug!("Number of primitives at base level: {}", self.primitives.len());
            intersection_count = self.primitives_at_level.len();
            for i in &self.primitives_at_level {
                let primitive = &primitives[*i];
                let intersection = primitive.object.intersection(r);

                //TODO: Add generic object type later
                (min_intersection_v, _) = min_intersection(
                    min_intersection_v,
                    intersection.map(|intersection| BVHIntersectionResult {
                        intersection,
                        primitive_idx: *i,
                    }),
                );
            }
            (min_intersection_v, intersection_count)
        } else {
            debug!("Non terminal");
            let mut min_intersection_v: Option<BVHIntersectionResult> = None;
            intersection_count += 2;
            if self.left_bounding_box.intersection(r) {
                debug!("Left box intersected");
                if let Some(left) = &self.left {
                    let left_intersection_tuple = left.intersection_helper(primitives, r);
                    min_intersection_v = left_intersection_tuple.0;
                    intersection_count += left_intersection_tuple.1;
                }
            }
            if self.right_bounding_box.intersection(r) {
                debug!("Right box intersected");
                if let Some(right) = &self.right {
                    let right_intersection_tuple = right.intersection_helper(primitives, r);
                    (min_intersection_v, _) =
                        min_intersection(min_intersection_v, right_intersection_tuple.0);
                    intersection_count += right_intersection_tuple.1;
                }
            }
            (min_intersection_v, intersection_count)
        }
    }


    pub fn intersection(&self, primitives: &Vec<Primitive>, r: &Ray) -> Option<BVHIntersectionResult> {
        debug!("Intersection requested");
        let intersection_time = std::time::Instant::now();
        let (ray_intersection, intersection_count) = self.intersection_helper(primitives, r);
        info!(
            "Intersection elapsed time: {:?}",
            intersection_time.elapsed()
        );
        debug!("Intersection count: {}", intersection_count);
        ray_intersection
    }

    pub fn print_traverse_helper(&self, depth: usize) {
        println!("depth is: {}", depth);
        println!("is_terminal: {}", self.is_terminal);
        println!("Primitives length: {}", self.primitives_at_level.len());
        println!("Left box is: {:?}", self.left_bounding_box);
        println!("Right box is: {:?}", self.right_bounding_box);
        if let Some(left) = &self.left {
            left.print_traverse_helper(depth + 1);
        }
        if let Some(right) = &self.right {
            right.print_traverse_helper(depth + 1);
        }
    }
    pub fn print_traverse(&self) {
        self.print_traverse_helper(0);
    }
}

#[cfg(test)]
mod tests {
    use glm::{make_vec3, normalize};

    use crate::{color::RGB, materials::DiffuseMaterial, sphere::Sphere};

    use super::*;

    fn init() {
        env_logger::init();
    }

    #[test]
    fn check_sphere_intersection() {
        init();
        let center = make_vec3(&[0.0, 1.0, 0.0]);
        let x: Sphere = Sphere::create(1.0, center.clone());
        let diffuse_material = DiffuseMaterial::create(RGB::create(0.0, 255.0, 127.0));
        let primitives = vec![Primitive::create(Box::new(x), Box::new(diffuse_material))];
        let mut bvh = BVH::create(primitives);
        bvh.print_traverse();
        let ray_origin = make_vec3(&[0.0, 10.0, 0.0]);
        let ray_direction = normalize(&(x.center - ray_origin));
        let r = Ray::create(ray_origin, ray_direction);
        assert!(bvh.intersection(&r).is_some());
    }
}
