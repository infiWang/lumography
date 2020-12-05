use crate::{math::*, material::*};

#[derive(Copy, Clone)]
pub enum Material
{
    Lambertian
    {
        albedo: Color,
        emit: Color
    },
    Metal
    {
        albedo: Color,
        fuzz: f64,
        emit: Color
    },
    Dielectric
    {
        albedo: Color,
        refraction: Color,
        eta: f64,
        fuzz: f64,
        emit: Color
    },
    Lightsource
    {
        emit: Color
    },

    //-!-!-!-DEBUG ONLY MATERIAL-!-!-!-//
    DebugNormalShading
    {
        normal: Vec3,
        mode: u8
    },
    DebugNormalRaycasting {}
}