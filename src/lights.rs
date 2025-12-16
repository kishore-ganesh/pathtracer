use crate::bounding_box::BoundingBox;
use crate::color::RGB;
use crate::primitives::{get_perp_vec, Object, Ray, RayIntersection};
use crate::sphere::Sphere;
use glm::{angle, cross, distance, normalize, TVec3};
use rand::Rng;
use std::f32::consts::PI;

pub struct RadianceInfo {
    pub light_color: RGB,
    pub light_vector: TVec3<f32>,
    pub intersection_point: TVec3<f32>,
    pub light_distance: f32,
    pub light_pdf: f32,
}

pub trait Light: LightClone + Send + Sync {
    // TODO: light shouldn't be concerned with normal, should be handled externally
    fn radiance_info(&self, r: &Ray, normal: TVec3<f32>) -> Option<RadianceInfo>;
    fn sample_radiance(&self, point: TVec3<f32>, normal: TVec3<f32>)
        -> (RGB, TVec3<f32>, f32, f32);
    fn is_delta(&self) -> bool;
}

/*
 * The following is a trick to get clone to work on dyn from:
 * https://stackoverflow.com/questions/30353462/how-to-clone-a-struct-storing-a-boxed-trait-object/30353928
 * */
pub trait LightClone {
    fn clone_light(&self) -> Box<dyn Light>;
}
impl<T> LightClone for T
where
    T: 'static + Light + Clone + Send,
{
    fn clone_light(&self) -> Box<dyn Light> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Light> {
    fn clone(&self) -> Box<dyn Light> {
        self.clone_light()
    }
}
//TODO: check light source interface
//
#[derive(Copy, Debug, Clone)]
pub struct PointLight {
    location: TVec3<f32>,
    color: RGB,
    intensity: f32,
}
impl PointLight {
    pub fn create(location: TVec3<f32>, color: RGB, intensity: f32) -> Self {
        PointLight {
            location,
            color,
            intensity,
        }
    }
}

impl Light for PointLight {
    fn radiance_info(&self, r: &Ray, normal: TVec3<f32>) -> Option<RadianceInfo> {
        unimplemented!();
    }
    fn sample_radiance(
        &self,
        point: TVec3<f32>,
        normal: TVec3<f32>,
    ) -> (RGB, TVec3<f32>, f32, f32) {
        let dist = distance(&self.location, &point);
        let light_vec = -normalize(&(point - self.location));
        let cos_angle = angle(&light_vec, &normal).cos();
        //////debug!("{}, {:?}, {:?}", cos_angle, self.color, self.color * cos_angle);
        (
            self.color * cos_angle * self.intensity,
            light_vec,
            dist,
            1.0,
        )
        //* (self.intensity/dist.powi(2));
    }
    fn is_delta(&self) -> bool {
        true
    }
}

struct InfiniteLight {}

#[derive(Debug, Copy, Clone)]
pub struct SphericalAreaLight {
    sphere: Sphere,
    color: RGB,
    intensity: f32,
}

impl SphericalAreaLight {
    pub fn create(sphere: Sphere, color: RGB, intensity: f32) -> Self {
        SphericalAreaLight {
            sphere,
            color,
            intensity,
        }
    }

    fn res_color_at_point(
        &self,
        point: TVec3<f32>,
        point_normal: TVec3<f32>,
        intersection_point: TVec3<f32>,
    ) -> RGB {
        let light_vec = -normalize(&(point - intersection_point));
        let theta_area = angle(&(intersection_point - self.sphere.center), &-light_vec);
        let theta_light = angle(&point_normal, &light_vec);
        if theta_light.cos() > 0.0 {
            self.color * theta_area.cos() * self.intensity * theta_light.cos() / 2.0
        } else {
            RGB::black()
        }
    }
}

impl Light for SphericalAreaLight {
    fn radiance_info(&self, r: &Ray, normal: TVec3<f32>) -> Option<RadianceInfo> {
        let intersection = self.sphere.intersection(r)?;
        let sin_theta_max = distance(&r.origin, &self.sphere.center).clamp(-1.0, 1.0);
        let theta_max = sin_theta_max.asin();
        // TODO: verify that this pdf is correct
        let light_pdf = 1.0 / ((1.0 - theta_max.cos()) * 2.0 * PI);
        let res_color = self.res_color_at_point(r.origin, normal, intersection.point);
        Some(RadianceInfo {
            light_color: res_color,
            light_vector: normalize(&(intersection.point - r.origin)),
            intersection_point: intersection.point,
            light_distance: distance(&intersection.point, &r.origin), // TODO: get this from RayIntersection
            light_pdf,
        })
    }
    fn sample_radiance(
        &self,
        point: TVec3<f32>,
        point_normal: TVec3<f32>,
    ) -> (RGB, TVec3<f32>, f32, f32) {
        //debug!("Sampling light at: {}", point);
        let dist = distance(&point, &self.sphere.center);
        let sin_theta_max = (self.sphere.r / dist).clamp(-1.0, 1.0);
        let theta_max = sin_theta_max.asin();
        let mut rng = rand::rng();
        let e1 = rng.random::<f32>() * theta_max;
        let e2 = rng.random::<f32>() * 2.0 * PI;
        let d_s =
            dist * e1.cos() - (self.sphere.r.powi(2) - dist.powi(2) * e1.sin().powi(2)).sqrt();

        let cos_alpha = ((self.sphere.r.powi(2) + dist.powi(2) - d_s.powi(2))
            / (2.0 * dist * self.sphere.r))
            .clamp(-1.0, 1.0);

        let alpha = cos_alpha.acos();
        let normal = normalize(&(point - self.sphere.center));
        let tangent = normalize(&get_perp_vec(&normal));
        let bitangent = cross(&normal, &tangent);
        //debug!("Theta max: {} alpha: {}", theta_max, alpha);
        //debug!("numerator: {}, denom: {}", self.sphere.r, dist);
        ////debug!("Length of normal: {}, tangent: {}, bitangent: {}", length(&normal), length(&tangent), length(&bitangent));
        ////debug!("Dot of normal, tangent is: {}", dot(&normal, &tangent));
        //TODO: refactor out (same thing in Disney BRDF)
        let intersection_point = (normal * cos_alpha
            + tangent * alpha.sin() * e2.sin()
            + bitangent * alpha.sin() * e2.cos())
            * self.sphere.r
            + self.sphere.center;
        //debug!("Normal: {}, Tangent: {}, Bitangent: {}, Intersection Point: {}", normal, tangent, bitangent, intersection_point);
        ////debug!("Length: {}", length(&(intersection_point)));
        //debug!("Point: {:?}, Intersection Point: {:?}", point, intersection_point);
        let light_vec = -normalize(&(point - intersection_point));
        let res_color = self.res_color_at_point(point, point_normal, intersection_point);
        let point_distance = distance(&intersection_point, &point);
        let pdf = 1.0 / ((1.0 - theta_max.cos()) * (2.0 * PI));
        (res_color, light_vec, point_distance, pdf)
    }

    fn is_delta(&self) -> bool {
        false
    }
}

// TODO: unification of emissive lights with regular lights
impl Object for SphericalAreaLight {
    fn intersection(&self, r: &Ray) -> Option<RayIntersection> {
        self.sphere.intersection(r)
    }
    fn color(&self, _: &TVec3<f32>) -> RGB {
        RGB::black()
    }
    fn le(&self, p: &TVec3<f32>, v: &TVec3<f32>) -> RGB {
        let normal = p - self.sphere.center;
        let theta_area = angle(&normal, v);
        self.color * theta_area.cos() * self.intensity
    }

    fn bounds(&self) -> BoundingBox {
        self.sphere.bounds()
    }
}
