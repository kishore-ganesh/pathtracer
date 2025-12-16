//
use crate::color::RGB;
use crate::primitives::{Ray, RayIntersection, get_basis_vectors, get_vec_at_angle, reflect_about_vec};
use glm::{angle, cross, normalize, TVec3};
use rand::Rng;
use std::f32::consts::PI;
pub trait Material: Send + Sync + MaterialClone {
    //TODO: check for better interface
    //For now, this will return a spectrum and a ray in the direction
    fn brdf(&self, r: RayIntersection, v: TVec3<f32>) -> (RGB, Ray, f32);
    fn brdf_eval(&self, r: &RayIntersection, v: &TVec3<f32>) -> (RGB, f32);
    fn is_delta(&self) -> bool;
}

/*
 * The following is a trick to get clone to work on dyn from:
 * https://stackoverflow.com/questions/30353462/how-to-clone-a-struct-storing-a-boxed-trait-object/30353928
 * */
pub trait MaterialClone {
    fn clone_material(&self) -> Box<dyn Material + Send>;
}
impl<T> MaterialClone for T
where
    T: 'static + Material + Clone,
{
    fn clone_material(&self) -> Box<dyn Material + Send> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Material> {
    fn clone(&self) -> Box<dyn Material> {
        self.clone_material()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct DiffuseMaterial {
    fraction: RGB,
}

impl DiffuseMaterial {
    pub fn create(f: RGB) -> Self {
        DiffuseMaterial { fraction: f }
    }
}

impl Material for DiffuseMaterial {
    fn brdf(&self, r: RayIntersection, _: TVec3<f32>) -> (RGB, Ray, f32) {
        //TODO: make this random direction

        let mut rand = rand::rng();
        let degree_angle = rand.gen_range(0.0..90.0);
        let rad_angle = (PI / 180.0) * degree_angle;
        let (tangent, bitangent) = get_basis_vectors(r.normal);
        // TODO: fix
        let direction = get_vec_at_angle(&r.normal, &tangent, rad_angle);

        (self.fraction, Ray::create(r.point, direction), 1.0)
    }
    fn brdf_eval(&self, _: &RayIntersection, _: &TVec3<f32>) -> (RGB, f32) {
        //TODO: fill in
        // return self.fraction;
        unimplemented!()

        //return RGB::black();
    }
    fn is_delta(&self) -> bool {
        false
    }
}

#[derive(Debug, Copy, Clone)]
pub struct SpecularMaterial {}

impl SpecularMaterial {
    pub fn create() -> Self {
        SpecularMaterial {}
    }
}

impl Material for SpecularMaterial {
    fn brdf(&self, r: RayIntersection, _: TVec3<f32>) -> (RGB, Ray, f32) {
        //TODO: extract out the reflection
        let ray = Ray::create(r.point, r.reflection);
        (RGB::create(255.0, 255.0, 255.0), ray, 1.0)
    }
    fn brdf_eval(&self, r: &RayIntersection, v: &TVec3<f32>) -> (RGB, f32) {
        let ang = angle(&r.normal, v);
        let err = 1e-5; //TODO: make error more global, new float class?
                        // TODO: fix pdf here
        if (ang - r.normal_angle).abs() < err {
            (RGB::create(255.0, 255.0, 255.0), 1.0)
        } else {
            (RGB::black(), 1.0)
        }
    }
    fn is_delta(&self) -> bool {
        true
    }
}

#[derive(Debug, Clone, Copy)]
//Credits to Brent Burley and Disney for the equations
pub struct DisneyBRDFMaterial {
    base_color: RGB,
    metallic: f32, //TODO: linear blend
    specular: f32,
    roughness: f32,
    //f0: f32
}

impl DisneyBRDFMaterial {
    pub fn create(base_color: RGB, metallic: f32, specular: f32, roughness: f32) -> Self {
        DisneyBRDFMaterial {
            base_color,
            metallic,
            specular,
            roughness,
        }
    }
    fn diffuse(&self, theta_d: f32, theta_l: f32, theta_v: f32) -> RGB {
        let fd_90 = 0.5 + 2.0 * (theta_d).cos().powi(2) * self.roughness;
        let const_l = 1.0 + (fd_90 - 1.0) * (1.0 - theta_l.cos()).powi(5);
        let const_r = 1.0 + (fd_90 - 1.0) * (1.0 - theta_v.cos()).powi(5);
        let const_c = (const_l * const_r) / PI;
        
        ////debug!("theta_d: {}, theta_l: {}, theta_v: {}, res: {:?}", theta_d, theta_l, theta_v, f_d);
        //debug!("fd90: {}, const_l: {}, const_r: {}, const_c: {}", fd_90, const_l, const_r, const_c);
        self.base_color * const_c
    }

    fn specular_d(&self, alpha: f32, theta_h: f32) -> f32 {
        
        alpha.powi(2) / (PI * (1.0 + (alpha.powi(2) - 1.0) * theta_h.cos().powi(2)).powi(2))//TODO: have to use normalized form?
    }

    fn specular_f(&self, theta_d: f32) -> RGB {
        //TODO: need to handle specular tint
        //is f0 a color?
        let remapped_specular = self.specular * 0.08;
        let tint = RGB::create(1.0, 1.0, 1.0);
        let f0 = (tint * remapped_specular) * (1.0 - self.metallic)
            + (self.base_color * self.metallic / 255.0);
        let res = f0 + (f0 - 1.0) * (-1.0) * (1.0 - theta_d.cos()).powi(5);

        res * 255.0
    }

    fn g1(&self, theta_m: f32, theta_n: f32, alpha_g: f32) -> f32 {
        //TODO: check positive term
        let r_term = 2.0 / (1.0 + (1.0 + alpha_g.powi(2) * theta_n.tan().powi(2)).sqrt());
        let l_term = (theta_m.cos()) / (theta_n.cos());
        
        l_term * r_term
    }
    fn specular_g(&self, theta_l: f32, theta_v: f32, theta_d: f32) -> f32 {
        //ggx
        let alpha_g = (0.5 + self.roughness / 2.0).powi(2);
        let l_term = self.g1(theta_d, theta_l, alpha_g);
        let r_term = self.g1(theta_d, theta_v, alpha_g);
        
        ////debug!("l_term: {}, r_term: {}, res: {}", l_term, r_term, res);
        l_term * r_term
    }

    fn sample_from_specular_d(&self, alpha: f32) -> (f32, f32) {
        //let mut rand =
        let mut rng = rand::rng();
        let e1 = rng.random::<f32>();
        let e2 = rng.random::<f32>();
        let phi = 2.0 * PI * e1;
        ////debug!("alpha: {}, e1: {}, e2: {}, numerator: {}, denominator: {}", alpha, e1, e2, numerator, denominator);
        let cos_theta_h = ((1.0 - e2) / (1.0 + ((alpha.powi(2) - 1.0) * e2)))
            .sqrt()
            .clamp(-1.0, 1.0);

        //debug!("{} {}", cos_theta_h, alt_value_n);
        (cos_theta_h, phi)
    }

    fn eval(&self, theta_d: f32, theta_h: f32, theta_l: f32, theta_v: f32) -> (RGB, f32) {
        //TODO: refactor alpha
        // debug!("Theta_d: {}, theta_h: {}, theta_l: {}, theta_v: {}", theta_d, theta_h, theta_l, theta_v);
        // debug!("Theta l.cos() {}, theta_v.cos() {}", theta_l.cos(), theta_v.cos());
        if theta_l.cos() < 0.0 || theta_v.cos() < 0.0 {
            return (RGB::black(), 1.0);
        }

        let alpha = self.roughness.powi(2);
        let diffuse = self.diffuse(theta_d, theta_l, theta_v);
        let specular_d = self.specular_d(alpha, theta_h);
        let specular_f = self.specular_f(theta_d);
        let specular_g = self.specular_g(theta_l, theta_v, theta_d);
        let specular = specular_f * specular_d * specular_g / (4.0 * theta_l.cos() * theta_v.cos());

        //debug!("Specular check: {:?} {:?}", specular, (specular_f*specular_d*specular_g)/(4.0 * theta_l.cos() * theta_v.cos()));
        //NOTE: for debugging, might be helpful to just check for diffuse
        let res_color = diffuse * (1.0 - self.metallic) + specular;
        // let res_color = specular;
        //TODO: check if there should be a sine here for solid sngle conversion: https://schuttejoe.github.io/post/ggximportancesamplingpart1/
        // pdf_h = specular_d * cos_theta
        // pdf_l = pdf_h / 4 * (l.h)
        let pdf = specular_d * theta_h.cos().abs() / (4.0 * theta_d.cos().abs());
        ////debug!("Specular color is: {:?}", specular);
        ////debug!("specular_d: {}, theta_h.cos(): {}, theta_d.cos(): {}", specular_d, theta_h.cos(), theta_d.cos());
        //debug!("Diffuse: {:?}, Specular_D: {}, Specular f: {:?}, Specular g: {}", diffuse, specular_d, specular_f, specular_g);
        (res_color, pdf)
    }

    //Where to get theta_h? Sample from D(theta_h), for anisotropic, phi = 1/2pi. Use it to find
    //half vector orientation, then reflect view about halfway.
}

impl Material for DisneyBRDFMaterial {
    fn brdf(&self, r: RayIntersection, v: TVec3<f32>) -> (RGB, Ray, f32) {
        //Sample from D(theta_h) to get theta_h, phi
        // Calculate h using theta_h, pi, three vectors (normal, tangent, bi tangent[r.normal,
        // r.perp, cross r.normal, r.perp?]
        // Reflect view vector: reflect_about_vec(&h, &v);
        // theta_d = angle(&h, &v); theta_h. theta_v = angle(&normal, &v). theta_l = angle(&normal,
        // &)
        // Return self.diffuse(theta_d, theta_-, theta_v) + self.specular_d(theta_h) *
        // self.specular_f(theta_d)*self.specular_g/4*cos theta_h*costheta_d
        //TODO: currently independent of PHI
        let normalized_v = normalize(&v);
        // debug!("Normalized v = {}", normalized_v);
        let alpha = self.roughness.powi(2);
        let (cos_theta_h, phi) = self.sample_from_specular_d(alpha);
        let theta_h = cos_theta_h.acos();
        debug_assert!(!theta_h.is_nan());
        let (tangent, bitangent) = get_basis_vectors(r.normal);
        // debug!("normal: {}, tangent: {}, bitangent: {}", r.normal, r.perp, bitangent);
        let h = r.normal * cos_theta_h
            + tangent * theta_h.sin() * phi.cos()
            + bitangent * theta_h.sin() * phi.sin();
        // debug!("Reflecting about within material");
        let l = reflect_about_vec(&normalized_v, &h);
        // debug!("Non normalized l is: {}", l);
        ////debug!("Length of l: {}, h: {}, v: {}", length(&l), length(&h), length(&v));
        ////debug!("Length of h: {}", length(&h));
        ////debug!("Angle lh: {}, vh: {}", angle(&l, &h), angle(&v, &h));
        ////debug!("cos_theta_h: {}, h: {}, light_vector: {}", cos_theta_h, h, l);
        let theta_l = angle(&r.normal, &l);
        let theta_v = angle(&r.normal, &normalized_v);
        let theta_d = angle(&h, &normalized_v);
        debug_assert!((theta_d - angle(&h, &l) <= 1e-5));
        let (res_color, pdf) = self.eval(theta_d, theta_h, theta_l, theta_v);
        //NOTE: this is when the light ray goes inside. For refractive, may have to handle this
        //separately
        if theta_l.cos() < 0.0 {
            // debug!("View vector inside");
            return (RGB::black(), Ray::create(r.point, r.normal), pdf);
        }

        //debug!("l is: {}", normalize(&l));
        //NOTE: we're starting the ray from a point slightly offset from the point.
        //TODO: Check later if this prevents the ray from intersecting the object it originated from
        //NOTE: the above comment is now invalid
        let ray = Ray::create(r.point, normalize(&l));
        if res_color.is_nan() {
            //panic!("Res Color is NaN");
        }
        (res_color, ray, pdf)
    }
    //TODO: refactor into just i, o
    fn brdf_eval(&self, r: &RayIntersection, l: &TVec3<f32>) -> (RGB, f32) {
        let v = normalize(&(r.origin - r.point));
        // debug!("Normal is: {}", r.normal);
        ////debug!("LIGHT LENGTH: {}", length(&l))

        //debug!("l: {}, v: {}", l, v);
        let h = normalize(&(l + v));
        let theta_d = angle(l, &h);
        let theta_l = angle(l, &r.normal);
        let theta_h = angle(&h, &r.normal);
        let theta_v = angle(&v, &r.normal);
        //debug!("Ray origin: {:?}, Ray Point: {:?}, Ray: {:?}", r.origin, r.point, r.origin - r.point);
        ////debug!("LIGHTS: Angle lh: {}, vh: {}", angle(&l, &h), angle(&v, &h));
        self.eval(theta_d, theta_h, theta_l, theta_v)
    }
    fn is_delta(&self) -> bool {
        false
    }
}

//later introduce BRDF
