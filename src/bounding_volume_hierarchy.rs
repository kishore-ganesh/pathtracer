use crate::bounding_box::BoundingBox;
use crate::color::RGB;
use crate::sphere::{Object, Ray, RayIntersection, Primitive, min_intersection};
use glm::TVec3;
use crate::materials::Material;
const min_primitives:usize = 5;
#[derive(Clone)]
pub struct BVHNode {
    pub primitives: Vec<Primitive>,
    is_terminal: bool,
    left: Option<Box<BVHNode>>, 
    right: Option<Box<BVHNode>>, 
    left_bounding_box: BoundingBox,
    right_bounding_box: BoundingBox,
    cached_primitive: Option<Primitive>,
    cached_primitive_old: Option<Primitive>
}

#[derive(Clone)]
pub struct Bucket {
    count: i32,
    bound: BoundingBox,
    cost: f32 
}

impl BVHNode {
    pub fn create(primitives: &Vec<Primitive>) -> BVHNode {
        return BVHNode::recursive_helper(primitives.clone(), 0, (primitives.len() as i32) -1);
    }
    //TODO: use move
    pub fn recursive_helper(primitives: Vec<Primitive>, l: i32, r: i32) -> BVHNode {
        if primitives.len() <= min_primitives {
            let mut new_primitives = vec![];
            for i in l..r+1 {
                //TODO: prevent unnecessary copies
                new_primitives.push(primitives[i as usize].clone())
            }
            return BVHNode {
                primitives: new_primitives, 
                is_terminal: true,
                left: None,
                right: None,
                left_bounding_box: BoundingBox::create_empty(),
                right_bounding_box: BoundingBox::create_empty(),
                cached_primitive: None,
                cached_primitive_old: None,
            }
        }
        let mut centroid_bounds = BoundingBox::create_empty();
        
        let dim = 1; //Hardcoding splitting dimension to y
        for i in l..r+1 {
            centroid_bounds = BoundingBox::union_point(centroid_bounds, primitives[i as usize].bounds().centroid());
        }
        let n_buckets = 12;
        let mut buckets = vec![Bucket { count: 0, bound: BoundingBox::create_empty(), cost: 0.0 }; n_buckets];
        for i in l..r+1 {
            let b = (centroid_bounds.offset(primitives[i as usize].bounds().centroid())[dim] * (n_buckets as f32)).floor();
            buckets[b as usize].bound = BoundingBox::union(buckets[b as usize].bound, primitives[i as usize].bounds());
            buckets[b as usize].count += 1;
        }
        let mut min_cost = f32::MAX;
        let mut min_index: i32 = -1;
        for i in 0..n_buckets {
            println!("Count of ith: {} bucket is: {}", i, buckets[i].count);
            let mut left = BoundingBox::create_empty();
            let mut right = BoundingBox::create_empty();
            let mut left_count = 0;
            let mut right_count = 0;

            for left_index in 0..i+1 {
                left = BoundingBox::union(left, buckets[left_index].bound);
                left_count += buckets[left_index].count;
            }
            for right_index in i+1..n_buckets {
                right = BoundingBox::union(right, buckets[right_index].bound);
                right_count += buckets[right_index].count;
            }
            buckets[i].cost = 0.125 + (left.surface_area() * (left_count as f32) + right.surface_area()*(right_count as f32))/(left.surface_area()+right.surface_area());
            if min_cost > buckets[i].cost  {
                min_cost = buckets[i].cost;
                min_index = i as i32;
            }
        
            

        }

        println!("Min cost is: {} at index: {}", min_cost, min_index);

        let left_primitives: Vec<Primitive> = primitives.iter().cloned().filter(|x| ((n_buckets as f32) * centroid_bounds.offset(x.bounds().centroid())[dim]).floor() <= min_index as f32).collect();
        let right_primitives: Vec<Primitive> = primitives.iter().cloned().filter(|x|((n_buckets as f32) * centroid_bounds.offset(x.bounds().centroid())[dim]).floor() > min_index as f32).collect();
        let mut left_bounding_box = BoundingBox::create_empty();
        let mut right_bounding_box = BoundingBox::create_empty();
        for primitive in left_primitives.iter() {
            left_bounding_box = BoundingBox::union(left_bounding_box, primitive.bounds());
        }
        for primitive in right_primitives.iter() {
            right_bounding_box = BoundingBox::union(right_bounding_box, primitive.bounds());
        }
        if left_primitives.len()==primitives.len() || right_primitives.len()==primitives.len() {
            panic!("Size of reduced sprimitives array is the same as the original - no splitting occurring");
        }
        let left_primitives_len = left_primitives.len();
        let right_primitives_len = right_primitives.len();
        let left_node = BVHNode::recursive_helper(left_primitives, l, left_primitives_len as i32 -1);
        let right_node = BVHNode::recursive_helper(right_primitives, l, right_primitives_len as i32 -1);
        return BVHNode{
            primitives: vec![],
            is_terminal: false,
            left: if left_primitives_len > 0 {Some(Box::new(left_node))} else  {None},
            right: if right_primitives_len >  0 {Some(Box::new(right_node))} else {None},
            left_bounding_box: BoundingBox::create_empty(),
            right_bounding_box: BoundingBox::create_empty(),
            cached_primitive: None,
            cached_primitive_old: None
        }


    }

    
    pub fn intersection_helper(&mut self, r: &Ray) -> (Option<RayIntersection>, Option<Primitive>) {
        self.cached_primitive_old = self.cached_primitive.clone();
        self.cached_primitive = None;
        if self.is_terminal {
            let mut min_intersection_v: Option<RayIntersection> = None;
            for (index, primitive) in (&self.primitives).into_iter().enumerate() {
                ////println!("Before ray object intersection test");
    
                let intersection = primitive.object.intersection(&r);
                //println!("{:?}", intersection);
                //TODO: Add generic object type later 
                //Closest
                let min_intersection_tuple = min_intersection(min_intersection_v, intersection);
                min_intersection_v = min_intersection_tuple.0;
                let is_min = min_intersection_tuple.1;
                if is_min {
                    self.cached_primitive = Some(primitive.clone());
                }
            
            }
            return (min_intersection_v, self.cached_primitive.clone());
    

        }
        else {
            let mut ray_intersection: Option<RayIntersection> = None;
            if self.left_bounding_box.intersection(r) {
                if let Some(left) = &mut self.left {
                    let left_intersection_tuple = left.intersection_helper(r);
                    ray_intersection = left_intersection_tuple.0;
                    self.cached_primitive = left_intersection_tuple.1;
                }
                
            }
            if self.right_bounding_box.intersection(r) {
                if let Some(right) = &mut self.right {
                    let right_intersection_tuple = right.intersection_helper(r);
                    let min_intersection_tuple = min_intersection ( ray_intersection, right_intersection_tuple.0);

                    ray_intersection = min_intersection_tuple.0; 
                    if min_intersection_tuple.1 {
                        self.cached_primitive = right_intersection_tuple.1;
                    }
                }
                
            }
            return (ray_intersection, self.cached_primitive.clone());
        }


    }

    pub fn brdf_old(&self, r: RayIntersection, v: TVec3<f32>) -> (RGB, Ray, f32){
        if let Some(p) = &self.cached_primitive_old {
            return p.brdf(r, v);
        }
        return (RGB::black(), Ray::create_empty(), 0.0);
    }
    pub fn brdf_eval_old(&self, r: &RayIntersection, v: &TVec3<f32>) -> RGB{
        if let Some(p) = &self.cached_primitive_old {
            return p.brdf_eval(r, v);
        }
        return RGB::black();
    }

    pub fn intersection(&mut self, r: &Ray) -> Option<RayIntersection> {
        let (ray_intersection, _) = self.intersection_helper(r);
        return ray_intersection;
    }
    pub fn color(&self, p: &TVec3<f32>) -> RGB {
        if let Some(primitive) = &self.cached_primitive {
            return primitive.color(p);
        }
        return RGB::black();
    }
    pub fn le(&self, p: &TVec3<f32>, v: &TVec3<f32>) -> RGB{
        if let Some(primitive) = &self.cached_primitive  {
            return primitive.le(p, v);
        }
        return RGB::black();
    }
    pub fn bounds(&self) -> BoundingBox {
        panic!("Unimplemented BVHNode box called");
        return BoundingBox::create_empty()
    }

    


}


impl Material for BVHNode {
    fn brdf(&self, r: RayIntersection, v: TVec3<f32>) -> (RGB, Ray, f32){
        if let Some(p) = &self.cached_primitive {
            return p.brdf(r, v);
        }
        return (RGB::black(), Ray::create_empty(), 0.0);
    }
    fn brdf_eval(&self, r: &RayIntersection, v: &TVec3<f32>) -> RGB{
        if let Some(p) = &self.cached_primitive {
            return p.brdf_eval(r, v);
        }
        return RGB::black();
    }
}

