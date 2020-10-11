use crate::{math::*, physics::*, object::*};

mod diffuse;
mod metal;

pub use self::{diffuse::*, metal::*};

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
    Emissive
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

impl Material
{
    #[inline]
    pub fn lambertian(nAlbedo: Color, nEmit: Color) -> Material
    {
        Material::Lambertian{ albedo: nAlbedo, emit: nEmit }
    }
    #[inline]
    pub fn metal(nAlbedo: Color, nFuzz: f64, nEmit: Color) -> Material
    {
        Material::Metal{ albedo: nAlbedo, fuzz: nFuzz, emit: nEmit }
    }


    #[inline]
    pub fn scatters(&self) -> bool
    {
        match self
        {
            Material::Metal { albedo, fuzz, emit } => true,
            Material::Lambertian { albedo, emit } => true,
            Material::Emissive { emit } => false,

            //-!-!-!-DEBUG ONLY MATERIAL-!-!-!-//
            Material::DebugNormalShading { normal, mode } => false,
            Material::DebugNormalRaycasting {} => false,
        }
    }
    #[inline]
    pub fn rngs(&self) -> bool
    {
        match self
        {
            Material::Metal { albedo, fuzz, emit } => if *fuzz > 0.0 { true } else { false },
            Material::Lambertian { albedo, emit } => true,
            Material::Emissive { emit } => false,

            //-!-!-!-DEBUG ONLY MATERIAL-!-!-!-//
            Material::DebugNormalShading { normal, mode } => false,
            Material::DebugNormalRaycasting {} => false,
        }
    }
    #[inline]
    pub fn emits(&self) -> bool
    {
        match self
        {
            Material::Metal { albedo, fuzz, emit } => (emit.r > 0.0 && emit.g > 0.0 && emit.b > 0.0),
            Material::Lambertian { albedo, emit } => (emit.r > 0.0 && emit.g > 0.0 && emit.b > 0.0),
            Material::Emissive { emit } => true,

            //-!-!-!-DEBUG ONLY MATERIAL-!-!-!-//
            Material::DebugNormalShading { normal, mode } => true,
            Material::DebugNormalRaycasting {} => false,
        }
    }

    #[inline]
    pub fn albedo(&self) -> Color
    {
        match self
        {
            Material::Metal { albedo, fuzz, emit } => *albedo,
            Material::Lambertian { albedo, emit } => *albedo,
            Material::Emissive { emit } => Color::new(0.0, 0.0, 0.0),

            //-!-!-!-DEBUG ONLY MATERIAL-!-!-!-//
            Material::DebugNormalShading { normal, mode } => Color::new(0.0, 0.0, 0.0),
            Material::DebugNormalRaycasting {} => Color::new(1.0, 1.0, 1.0),
        }
    }

    #[inline]
    pub fn emit(&self) -> Color
    {
        match self
        {
            Material::Metal { albedo, fuzz, emit } => *emit,
            Material::Lambertian { albedo, emit } => *emit,
            Material::Emissive { emit } => *emit,

            //-!-!-!-DEBUG ONLY MATERIAL-!-!-!-//
            Material::DebugNormalShading { normal, mode } => Color::new(normal.x, normal.y, normal.z),
            Material::DebugNormalRaycasting {} => Color::new(0.0, 0.0, 0.0),
        }
    }
}