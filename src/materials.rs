//
use std::f32::consts::PI;
use glm::{angle, cross, length, make_vec3, normalize, TVec3};
use rand::Rng;
use crate::color::RGB;
use crate::sphere::{Ray, RayIntersection};
use crate::primitives::{get_vec_at_angle, reflect_about_vec};
pub trait Material {
    //TODO: check for better interface
    //For now, this will return a spectrum and a ray in the direction
    fn brdf(&self, r: RayIntersection, v: TVec3<f32>) -> (RGB, Ray, f32);
    fn brdf_eval(&self, r: &RayIntersection, v: &TVec3<f32>) -> RGB;
}
#[derive(Debug, Copy, Clone)]
pub struct DiffuseMaterial {
    fraction: RGB
}

impl DiffuseMaterial{
    pub fn create(f: RGB) -> Self{
        return DiffuseMaterial{fraction: f};
    }
}

impl Material for DiffuseMaterial{
    fn brdf(&self, r: RayIntersection, v: TVec3<f32>) -> (RGB, Ray, f32){
        //TODO: make this random direction
        
        let mut rand = rand::thread_rng();
        let degree_angle = rand.gen_range(0.0..90.0);
        let rad_angle = (PI/180.0) * degree_angle;
        let direction = get_vec_at_angle(&r.normal, &r.perp, rad_angle);

        return (self.fraction, Ray::create(r.point, direction), 1.0);
    }
    fn brdf_eval(&self, r: &RayIntersection, v: &TVec3<f32>) -> RGB{
        //TODO: fill in
        return self.fraction;
        //return RGB::black();
    }
}


#[derive(Debug, Copy, Clone)]
pub struct SpecularMaterial {

}

impl SpecularMaterial{
    pub fn create() -> Self{
        return SpecularMaterial{};
    }
}

impl Material for SpecularMaterial{
    fn brdf(&self, r: RayIntersection, v: TVec3<f32>) -> (RGB, Ray, f32){
        //TODO: extract out the reflection
        let ray = Ray::create(r.point, r.reflection);
        return (RGB::create(255.0,255.0,255.0), ray, 1.0);
    }
    fn brdf_eval(&self, r: &RayIntersection, v: &TVec3<f32>) -> RGB{
        let ang = angle(&r.normal, &v);
        let err = 1e-5; //TODO: make error more global, new float class?
        if((ang-r.normal_angle).abs() < err){
            return RGB::create(255.0,255.0,255.0);
        }
        else{
            return RGB::black();
        }
    }
}

//Credits to Brent Burley and Disney for the equations
pub struct DisneyBRDFMaterial{
    base_color: RGB,
    metallic: f32, //TODO: linear blend
    specular: f32,
    roughness: f32,
    //f0: f32
}

impl DisneyBRDFMaterial{
    fn diffuse(&self, theta_d: f32, theta_l: f32, theta_v: f32) -> RGB{
        let fd_90 = 0.5 + 2.0 * theta_d.powi(2).cos() * self.roughness;
        let const_l = (1.0  + (fd_90 - 1.0 )*(1.0-theta_l.cos()).powi(5));
        let const_r = (1.0  + (fd_90 - 1.0 )*(1.0-theta_v.cos()).powi(5));
        let const_c = (const_l * const_r) / PI;
        let f_d = self.base_color * const_c;
        return f_d;


    }

    fn specular_d(&self,alpha: f32,theta_h: f32) -> f32{
        let gamma = 2.0;
        let numerator = (gamma-1.0) * (alpha.powf(2.0) -1.0);
        let denom = PI * (1.0 - (alpha.powi(2).powf(1.0-gamma))) * (1.0 + (alpha.powi(2) - 1.0)*theta_h.cos().powi(2)).powf(gamma);

    
        return numerator / denom; //TODO: have to use normalized form?
    }

    fn specular_f(&self, theta_d: f32) -> RGB{
        //TODO: need to handle specular tint
        //is f0 a color?
        let tint = RGB::create(255.0,255.0,255.0);
        let f0 = tint * self.specular;
        let res = f0 +  (f0-1.0) * (-1.0) * (1.0-theta_d.cos()).powi(5);
        return res;
    }

    fn g1(&self, theta_m: f32, theta_n: f32, alpha_g: f32) -> f32{
         //TODO: check positive term 
          let r_term = 2.0 / (1.0 + (1.0 +alpha_g.powi(2) * theta_n.tan().powi(2)).sqrt());
          let l_term = (theta_m)/(theta_n);
          return l_term * r_term;
          
    }
    fn specular_g(&self, theta_l: f32, theta_v: f32, theta_d: f32) -> f32{
        //ggx
        let alpha_g = (0.5 + self.roughness/2.0).powi(2);
        let l_term = self.g1(theta_d, theta_l, alpha_g);
        let r_term = self.g1(theta_d, theta_v, alpha_g);
        return l_term * r_term;

    } 

    fn sample_from_specular_d(&self, alpha: f32) -> (f32, f32) {
        //let mut rand = 
        let mut rng = rand::thread_rng();
        let e1 = rng.gen::<f32>();
        let e2 = rng.gen::<f32>();
        let phi = 2.0 * PI * e1;
        let gamma = 2.0;
        let numerator = (1.0 - ((alpha.powi(2).powf(1.0-gamma)) * (1.0-e2) + e2).powf(1.0/(1.0-gamma)));
        let denominator = (1.0-alpha.powi(2));
        let cos_theta_h = (numerator/denominator).sqrt();
        return (cos_theta_h, phi);

    }

    fn eval(&self, theta_d: f32, theta_h: f32, theta_l: f32, theta_v: f32) -> (RGB, f32){
       //TODO: refactor alpha 
        let alpha = self.roughness.powi(2);
        let diffuse = self.diffuse(theta_d, theta_l, theta_v);
        let specular_d = self.specular_d(alpha, theta_h);
        let specular_f = self.specular_f(theta_d);
        let specular_g = self.specular_g(theta_l, theta_v, theta_d);
        let res_color =  diffuse + specular_f * specular_d * specular_g / 4.0 * (theta_l.cos() * theta_v.cos());
        let pdf = specular_d * theta_h.cos() / (4.0 * theta_d.cos());
        return (res_color, pdf);
    }


    //Where to get theta_h? Sample from D(theta_h), for anisotropic, phi = 1/2pi. Use it to find
    //half vector orientation, then reflect view about halfway.
}

impl Material for DisneyBRDFMaterial{
    fn brdf(&self, r: RayIntersection, v: TVec3<f32>) -> (RGB, Ray, f32){
        //Sample from D(theta_h) to get theta_h, phi 
        // Calculate h using theta_h, pi, three vectors (normal, tangent, bi tangent[r.normal,
        // r.perp, cross r.normal, r.perp?] 
        // Reflect view vector: reflect_about_vec(&h, &v);
        // theta_d = angle(&h, &v); theta_h. theta_v = angle(&normal, &v). theta_l = angle(&normal,
        // &)
        // Return self.diffuse(theta_d, theta_-, theta_v) + self.specular_d(theta_h) *
        // self.specular_f(theta_d)*self.specular_g/4*cos theta_h*costheta_d
        //TODO: currently independent of PHI
        
        let alpha = self.roughness.powi(2);
        let (cos_theta_h, phi) = self.sample_from_specular_d(alpha);
        let theta_h = cos_theta_h.acos(); 
        let bitangent = cross(&r.normal, &r.perp);
        let h = r.normal * cos_theta_h + r.perp * theta_h.sin() * phi.cos() + bitangent * theta_h.sin() * phi.sin();
        let (l, _) = reflect_about_vec(&h, &v);
        let theta_l = angle(&r.normal, &l);
        let theta_v = angle(&r.normal, &v);
        let theta_d = angle(&h, &v);
        let (res_color, pdf) = self.eval(theta_d, theta_h, theta_l, theta_v);
        let ray = Ray::create(r.point, normalize(&l));
        return (res_color, ray, pdf);


    }
    //TODO: refactor into just i, o
    fn brdf_eval(&self, r: &RayIntersection, v: &TVec3<f32>) -> RGB{
        let l = r.origin - r.point;
        let h = (l + v) / length(&(l + v));
        let theta_d = angle(&l, &h);
        let theta_l = angle(&l, &r.normal);
        let theta_h = angle(&h, &r.normal);
        let theta_v = angle(&v, &r.normal);
        let (res_color, _) = self.eval(theta_d, theta_h, theta_l, theta_v);
        return res_color; 
        //return RGB::black();
    }
}


//later introduce BRDF
