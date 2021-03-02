trait Material {
    //TODO: check for better interface
    fn bsdf(&self, r: RayIntersection) -> RGB{
        
    }
}
struct DiffuseMaterial {

}

struct SpecularMaterial {

}

//later introduce BRDF
