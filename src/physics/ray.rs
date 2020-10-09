use crate::{math::*, material::*, object::*};

#[derive(Copy, Clone)]
pub struct Ray
{
    pub origin: Pos3,
    pub direction: Vec3,
    pub color: Color,
    pub step: u8,
    pub t: f64,
}

impl Ray
{
    pub fn default() -> Ray
    {
        Ray
        {
            origin: Pos3 { x: 0.0, y: 0.0, z: 0.0 },
            direction: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
            color: Color { r: 0.0, g: 0.0, b: 0.0 },
            step: 0,
            t: 0.0,
        }
    }
    pub fn new(nOri: Pos3, nDirect: Vec3, nColor: Color, nS: u8, nT: f64) -> Ray
    {
        Ray
        {
            origin: nOri,
            direction: nDirect,
            color: nColor,
            step: nS,
            t: nT,
        }
    }

    #[inline] pub fn at(&self, t: f64) -> Pos3 {self.origin + self.direction*t}
}


pub fn scatter(ray: &mut Ray, stat: &BoolObjF64) -> Vec3
{
    let normal: Vec3 = (ray.at(stat.f ) - stat.obj.pos).unit();
    match stat.obj.material
    {
        Material::Metal { albedo, fuzz } =>
        {
            let mut rayDirection: Vec3 = ((*ray).direction | normal).unit();
            rayDirection += fuzz.sqrt()*Vec3::rngUnit();

            rayDirection.unit()
        },
        Material::Lambertian { albedo } =>
        {
            let mut rayDirection: Vec3 = Vec3::rngUnit();
            if !(rayDirection&normal) { rayDirection = -rayDirection; }

            rayDirection
        },

        //-!-!-!-DEBUG ONLY MATERIAL-!-!-!-//
        Material::DebugNormalShading { mode } =>
        {
            Vec3::new((normal.x + 1.0) / 2.0, (normal.y + 1.0) / 2.0, (normal.z + 1.0) / 2.0).unit()
        },
        Material::DebugNormalRaycasting {} =>
        {
            Vec3::default()
        }
    }
}