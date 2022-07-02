use std::cmp;
use crate::bounding_box::BoundingBox;
use crate::color::RGB;
use crate::sphere::{Object, Ray, RayIntersection, Primitive, min_intersection};
use glm::TVec3;
use crate::materials::Material;
use std::mem::swap;
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
        println!("Length of primitives is: {}", primitives.len());
        return BVHNode::recursive_helper(primitives.clone(), 0, (primitives.len() as i32) -1);
    }
    //TODO: use move
    pub fn recursive_helper(primitives: Vec<Primitive>, l: i32, r: i32) -> BVHNode {
        println!("Recursive helper called");
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
        let mut total_bounds = BoundingBox::create_empty();
        
        for i in l..r+1 {
            centroid_bounds = BoundingBox::union_point(centroid_bounds, primitives[i as usize].bounds().centroid());
            total_bounds = BoundingBox::union(total_bounds, primitives[i as usize].bounds());
        }
        let dim = total_bounds.maximum_extent(); //NOTE: Hardcoding splitting dimension to y
        println!("dim is: {} for total_bounds: {:?}", dim, total_bounds);
        let n_buckets = 12;
        let mut buckets = vec![Bucket { count: 0, bound: BoundingBox::create_empty(), cost: 0.0 }; n_buckets];
        for i in l..r+1 {
            let b = (centroid_bounds.offset(primitives[i as usize].bounds().centroid())[dim] * (n_buckets as f32)).floor();
            //println!("Centroid bounds offset: {:?}", centroid_bounds.offset(primitives[i as usize].bounds().centroid()));
            let b = cmp::min(b as i32, (n_buckets-1) as i32);
            buckets[b as usize].bound = BoundingBox::union(buckets[b as usize].bound, primitives[i as usize].bounds());
            buckets[b as usize].count += 1;
        }
        let mut min_cost = f32::MAX;
        let mut min_index: i32 = -1;
        for i in 0..n_buckets {
            //println!("Count of ith: {} bucket is: {}", i, buckets[i].count);
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
            buckets[i].cost = 0.125 + (left.surface_area() * (left_count as f32) + right.surface_area()*(right_count as f32))/(total_bounds.surface_area());
            println!("Cost of ith: {} bucket is: {}", i, buckets[i].cost);
            if min_cost > buckets[i].cost  {
                min_cost = buckets[i].cost;
                min_index = i as i32;
            }
        
            

        }

        println!("Min cost is: {} at index: {}", min_cost, min_index);

        for primitive in primitives.iter().cloned() {
            println!("Primitive bounds is: {:?}, primitive centroid is: {:?}", primitive.bounds(), primitive.bounds().centroid());
            println!("Offset is: {:?}", centroid_bounds.offset(primitive.bounds().centroid()));
            println!("Index is: {}", (n_buckets as f32) * centroid_bounds.offset(primitive.bounds().centroid())[dim].floor());
        }
        let left_primitives: Vec<Primitive> = primitives.iter().cloned().filter(|x| ((n_buckets as f32) * centroid_bounds.offset(x.bounds().centroid())[dim]).floor() <= (min_index as f32)).collect();
        let right_primitives: Vec<Primitive> = primitives.iter().cloned().filter(|x|((n_buckets as f32) * centroid_bounds.offset(x.bounds().centroid())[dim]).floor() > (min_index as f32)).collect();
        println!("Centroid bounds: {:?}", centroid_bounds);
        println!("Left length is: {}, Right length is: {}, Primitives length is: {}", left_primitives.len(), right_primitives.len(), primitives.len());
        debug_assert!((left_primitives.len() + right_primitives.len())==primitives.len());
        let mut left_bounding_box = BoundingBox::create_empty();
        let mut right_bounding_box = BoundingBox::create_empty();
        for primitive in left_primitives.iter() {
            left_bounding_box = BoundingBox::union(left_bounding_box, primitive.bounds());
        }
        for primitive in right_primitives.iter() {
            right_bounding_box = BoundingBox::union(right_bounding_box, primitive.bounds());
        }
        if left_primitives.len()==primitives.len() || right_primitives.len()==primitives.len() {
            //No splitting occurring here
            println!("Size of reduced sprimitives array is the same as the original - no splitting occurring");
            return BVHNode {
                primitives: primitives.clone(), 
                is_terminal: true,
                left: None,
                right: None,
                left_bounding_box: BoundingBox::create_empty(),
                right_bounding_box: BoundingBox::create_empty(),
                cached_primitive: None,
                cached_primitive_old: None,
            };
            //panic!("Size of reduced sprimitives array is the same as the original - no splitting occurring");
        }
        let left_primitives_len = left_primitives.len();
        let right_primitives_len = right_primitives.len();
    
        let left_node = if left_primitives_len > 0  {Some(Box::new(BVHNode::recursive_helper(left_primitives, l, left_primitives_len as i32 -1)))} else  {None};
        let right_node = if right_primitives_len > 0 {Some(Box::new(BVHNode::recursive_helper(right_primitives, l, right_primitives_len as i32 -1)))} else {None};
        return BVHNode{
            primitives: vec![],
            is_terminal: false,
            left: left_node,
            right: right_node,
            left_bounding_box: left_bounding_box,
            right_bounding_box: right_bounding_box,
            cached_primitive: None,
            cached_primitive_old: None
        }


    }

    
    pub fn intersection_helper(&mut self, r: &Ray) -> (Option<RayIntersection>, Option<Primitive>) {
        
        swap(&mut self.cached_primitive_old, &mut self.cached_primitive);
        // self.cached_primitive_old = self.cached_primitive;
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
                // println!("Left box intersected");
                if let Some(left) = &mut self.left {
                    let left_intersection_tuple = left.intersection_helper(r);
                    ray_intersection = left_intersection_tuple.0;
                    self.cached_primitive = left_intersection_tuple.1;
                }
                
            }
            if self.right_bounding_box.intersection(r) {
                // println!("Right box intersected");
                if let Some(right) = &mut self.right {
                    let right_intersection_tuple = right.intersection_helper(r);
                    let min_intersection_tuple = min_intersection ( ray_intersection, right_intersection_tuple.0);

                    ray_intersection = min_intersection_tuple.0; 
                    if min_intersection_tuple.1 {
                        self.cached_primitive = right_intersection_tuple.1;
                    }
                    else{
                        //panic!("Test right intersection being smaller");
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
        return (RGB::create(255.0,0.0,0.0), Ray::create_empty(), 0.0);
    }
    pub fn brdf_eval_old(&self, r: &RayIntersection, v: &TVec3<f32>) -> RGB{
        
        if let Some(p) = &self.cached_primitive_old {
            //println!("Cached primitive old is valid");
            return p.brdf_eval(r, v);
        }
        else{
            //println!("Cached primitive old is invalid");
        }
        panic!("BRDF Eval old cached primitive missing");
        return RGB::create(0.0,255.0,0.0);
    }

    pub fn intersection(&mut self, r: &Ray) -> Option<RayIntersection> {
        //println!("Intersection requested");
        let (ray_intersection, _) = self.intersection_helper(r);
        return ray_intersection;
    }
    pub fn color(&self, p: &TVec3<f32>) -> RGB
     {
        if let Some(primitive) = &self.cached_primitive {
            return primitive.color(p);
        }
        return RGB::black();
    }
    pub fn le(&self, p: &TVec3<f32>, v: &TVec3<f32>) -> RGB{
        if let Some(primitive) = &self.cached_primitive  {
            return primitive.le(p, v);
        }
        return RGB::create(255.0,255.0,255.0);
    }
    pub fn bounds(&self) -> BoundingBox {
        panic!("Unimplemented BVHNode box called");
        return BoundingBox::create_empty()
    }
    pub fn print_traverse_helper(&self, depth: usize){
        println!("depth is: {}", depth);
        println!("is_terminal: {}", self.is_terminal);
        println!("Primitives length: {}", self.primitives.len());
        println!("Left box is: {:?}", self.left_bounding_box);
        println!("Right box is: {:?}", self.right_bounding_box);
        if let Some(left) = &self.left {
            left.print_traverse_helper(depth+1);
        }
        if let Some(right) = &self.right {
            right.print_traverse_helper(depth+1);
        }
    }
    pub fn print_traverse(&self) {
        self.print_traverse_helper(0);
    }

    


}


impl Material for BVHNode {
    fn brdf(&self, r: RayIntersection, v: TVec3<f32>) -> (RGB, Ray, f32){
        if let Some(p) = &self.cached_primitive {
            return p.brdf(r, v);
        }
        return (RGB::create(0.0,0.0,255.0), Ray::create_empty(), 0.0);
    }
    fn brdf_eval(&self, r: &RayIntersection, v: &TVec3<f32>) -> RGB{
        if let Some(p) = &self.cached_primitive {
            return p.brdf_eval(r, v);
        }
        return RGB::create(255.0,255.0,0.0);
    }
}

