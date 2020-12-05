use crate::{math::*, material::*};

impl Material
{
    #[inline]
    pub fn lambertian(n_albedo: Color, n_emit: Color) -> Material
    {
        Material::Lambertian{ albedo: n_albedo, emit: n_emit }
    }
    #[inline]
    pub fn metal(n_albedo: Color, n_fuzz: f64, n_emit: Color) -> Material
    {
        Material::Metal{ albedo: n_albedo, fuzz: n_fuzz, emit: n_emit }
    }


    #[inline]
    pub fn scatters(&self) -> bool
    {
        match self
        {
            Material::Metal { albedo: _albedo, fuzz: _fuzz, emit: _emit } => true,
            Material::Lambertian { albedo: _albedo, emit: _emit } => true,
            Material::Dielectric { albedo, refraction: _refraction, eta: _eta, fuzz: _fuzz, emit: _emit } => false,
            Material::Lightsource { emit: _emit } => false,

            //-!-!-!-DEBUG ONLY MATERIAL-!-!-!-//
            Material::DebugNormalShading { normal: _normal, mode: _mode } => false,
            Material::DebugNormalRaycasting {} => false,
        }
    }
    #[inline]
    pub fn rngs(&self) -> bool
    {
        match self
        {
            Material::Metal { albedo: _albedo, fuzz, emit: _emit } => if *fuzz > 0.0 { true } else { false },
            Material::Lambertian { albedo: _albedo, emit: _emit } => true,
            Material::Dielectric { albedo: _albedo, refraction: _refraction, eta: _eta, fuzz, emit: _emit } => if *fuzz > 0.0 { true } else { false },
            Material::Lightsource { emit: _emit } => false,

            //-!-!-!-DEBUG ONLY MATERIAL-!-!-!-//
            Material::DebugNormalShading { normal: _normal, mode: _mode } => false,
            Material::DebugNormalRaycasting {} => false,
        }
    }
    #[inline]
    pub fn refracts(&self) -> bool
    {
        match self
        {
            Material::Metal { albedo: _albedo, fuzz: _fuzz, emit: _emit } => false,
            Material::Lambertian { albedo: _albedo, emit: _emit } => false,
            Material::Dielectric { albedo: _albedo, refraction: _refraction, eta: _eta, fuzz: _fuzz, emit: _emit } => true,
            Material::Lightsource { emit: _emit } => false,

            //-!-!-!-DEBUG ONLY MATERIAL-!-!-!-//
            Material::DebugNormalShading { normal: _normal, mode: _mode } => false,
            Material::DebugNormalRaycasting {} => false,
        }
    }
    #[inline]
    pub fn emits(&self) -> bool
    {
        match self
        {
            Material::Metal { albedo: _albedo, fuzz: _fuzz, emit } => (emit.r > 0.0 && emit.g > 0.0 && emit.b > 0.0),
            Material::Lambertian { albedo: _albedo, emit } => (emit.r > 0.0 && emit.g > 0.0 && emit.b > 0.0),
            Material::Dielectric { albedo: _albedo, refraction: _refraction, eta: _eta, fuzz: _fuzz, emit } => (emit.r > 0.0 && emit.g > 0.0 && emit.b > 0.0),
            Material::Lightsource { emit: _emit } => true,

            //-!-!-!-DEBUG ONLY MATERIAL-!-!-!-//
            Material::DebugNormalShading { normal: _normal, mode: _mode } => true,
            Material::DebugNormalRaycasting {} => false,
        }
    }

    #[inline]
    pub fn attenuation(&self) -> Color
    {
        match self
        {
            Material::Metal { albedo, fuzz: _fuzz, emit: _emit } => *albedo,
            Material::Lambertian { albedo, emit: _emit } => *albedo,
            Material::Dielectric { albedo, refraction, eta: _eta, fuzz: _fuzz, emit: _emit } => (*albedo + *refraction),
            Material::Lightsource { emit: _emit } => Color::new(0.0, 0.0, 0.0),

            //-!-!-!-DEBUG ONLY MATERIAL-!-!-!-//
            Material::DebugNormalShading { normal: _normal, mode: _mode } => Color::new(1.0, 1.0, 1.0),
            Material::DebugNormalRaycasting {} => Color::new(1.0, 1.0, 1.0),
        }
    }
    #[inline]
    pub fn albedo(&self) -> Color
    {
        match self
        {
            Material::Metal { albedo, fuzz: _fuzz, emit: _emit } => *albedo,
            Material::Lambertian { albedo, emit: _emit } => *albedo,
            Material::Dielectric { albedo, refraction: _refraction, eta: _eta, fuzz: _fuzz, emit: _emit } => *albedo,
            Material::Lightsource { emit: _emit } => Color::new(0.0, 0.0, 0.0),

            //-!-!-!-DEBUG ONLY MATERIAL-!-!-!-//
            Material::DebugNormalShading { normal: _normal, mode: _mode } => Color::new(0.0, 0.0, 0.0),
            Material::DebugNormalRaycasting {} => Color::new(1.0, 1.0, 1.0),
        }
    }
    #[inline]
    pub fn refraction(&self) -> Color
    {
        match self
        {
            Material::Metal { albedo: _albedo, fuzz: _fuzz, emit: _emit } => Color::rgb_empty(),
            Material::Lambertian { albedo: _albedo, emit: _emit } => Color::rgb_empty(),
            Material::Dielectric { albedo: _albedo, refraction, eta: _eta, fuzz: _fuzz, emit: _emit } => *refraction,
            Material::Lightsource { emit: _emit } => Color::rgb_empty(),

            //-!-!-!-DEBUG ONLY MATERIAL-!-!-!-//
            Material::DebugNormalShading { normal: _normal, mode: _mode } => Color::rgb_empty(),
            Material::DebugNormalRaycasting {} => Color::rgb_empty(),
        }
    }
    #[inline]
    pub fn fuzz(&self) -> f64
    {
        match self
        {
            Material::Metal { albedo: _albedo, fuzz, emit: _emit } => *fuzz,
            Material::Lambertian { albedo: _albedo, emit: _emit } => 0f64,  // Whelp......
            Material::Dielectric { albedo: _albedo, refraction: _refraction, eta: _fuzz, fuzz, emit: _emit } => *fuzz,
            Material::Lightsource { emit: _emit } => 0f64,

            //-!-!-!-DEBUG ONLY MATERIAL-!-!-!-//
            Material::DebugNormalShading { normal: _normal, mode: _mode } => 0f64,
            Material::DebugNormalRaycasting {} => 0f64,
        }
    }
    #[inline]
    pub fn eta(&self) -> f64
    {
        match self
        {
            Material::Metal { albedo: _albedo, fuzz: _fuzz, emit: _emit } => 1f64,
            Material::Lambertian { albedo: _albedo, emit: _emit } => 1f64,
            Material::Dielectric { albedo: _albedo, refraction: _refraction, eta, fuzz: _fuzz, emit: _emit } => *eta,
            Material::Lightsource { emit: _emit } => 1f64,

            //-!-!-!-DEBUG ONLY MATERIAL-!-!-!-//
            Material::DebugNormalShading { normal: _normal, mode: _mode } => 1f64,
            Material::DebugNormalRaycasting {} => 1f64,
        }
    }
    #[inline]
    pub fn emit(&self) -> Color
    {
        match self
        {
            Material::Metal { albedo: _albedo, fuzz: _fuzz, emit } => *emit,
            Material::Lambertian { albedo: _albedo, emit } => *emit,
            Material::Dielectric { albedo: _albedo, refraction: _refraction, eta: _eta, fuzz: _fuzz, emit } => *emit,
            Material::Lightsource { emit } => *emit,

            //-!-!-!-DEBUG ONLY MATERIAL-!-!-!-//
            Material::DebugNormalShading { normal, mode: _mode } => Color::new(normal.x, normal.y, normal.z),
            Material::DebugNormalRaycasting {} => Color::new(0.0, 0.0, 0.0),
        }
    }
}

impl PartialEq for Material
{
    fn eq(&self, other: &Self) -> bool
    {
        self.scatters() == other.scatters() && self.refracts() == other.refracts() && self.rngs() == other.rngs() && self.emits() == other.emits() &&
        self.albedo() == other.albedo() && self.emits() == other.emits() && self.fuzz() == other.fuzz()
    }
}