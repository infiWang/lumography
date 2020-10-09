use crate::{math::*, physics::*, object::*};

mod diffuse;
mod metal;

pub use self::{diffuse::*, metal::*};

#[derive(Copy, Clone)]
pub enum Material
{
    Lambertian
    {
        albedo: f64
    },
    Metal
    {
        albedo: f64,
        fuzz: f64,
    },

    //-!-!-!-DEBUG ONLY MATERIAL-!-!-!-//
    DebugNormalShading
    {
        mode: u8
    },
    DebugNormalRaycasting {}
}

impl Material
{
    pub fn lambertian(nAlbedo: f64) -> Material
    {
        Material::Lambertian{ albedo: nAlbedo }
    }
    pub fn metal(nAlbedo: f64, nFuzz: f64) -> Material
    {
        Material::Metal{ albedo: nAlbedo, fuzz: nFuzz }
    }

    pub fn albedo(material: Material) -> f64
    {
        match material
        {
            Material::Metal { albedo, fuzz } => albedo,
            Material::Lambertian { albedo } => albedo,

            //-!-!-!-DEBUG ONLY MATERIAL-!-!-!-//
            Material::DebugNormalShading { mode } => 1.0,
            Material::DebugNormalRaycasting {} => 1.0,
        }
    }
}