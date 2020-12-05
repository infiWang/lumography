use crate::{math::*, material::*, object::*};

#[derive(Copy, Clone)]
pub struct Ray
{
    pub origin: Pos3,
    pub direction: Vec3,
    pub color: Color
}

impl Ray
{
    #[inline]
    pub fn default() -> Ray
    {
        Ray
        {
            origin: Pos3 { x: 0.0, y: 0.0, z: 0.0 },
            direction: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
            color: Color { r: 0.0, g: 0.0, b: 0.0 }
        }
    }
    #[inline]
    pub fn new(n_ori: Pos3, n_direct: Vec3, n_color: Color) -> Ray
    {
        Ray
        {
            origin: n_ori,
            direction: n_direct,
            color: n_color
        }
    }

    #[inline] pub fn at(&self, t: f64) -> Pos3 {self.origin + self.direction*t}
}

#[inline]
pub fn scatter(ray: &mut Ray, stat: &BoolObjF64) -> Vec3
{
    let normal: Vec3 = (ray.at(stat.f ) - stat.obj.pos).unit();

    return match stat.obj.material
    {
        Material::Metal{ albedo: _albedo, fuzz, emit: _emit } =>
        {
            let mut ray_direction: Vec3 = ((*ray).direction | normal).unit();
            if fuzz > 0.0 { ray_direction += fuzz.sqrt()*Vec3::rng_unit(); }

            ray_direction.unit()
        },
        Material::Lambertian{ albedo: _albedo, emit: _emit } =>
        {
            let mut ray_direction: Vec3 = Vec3::rng_unit();
            if !(ray_direction &normal) { ray_direction = -ray_direction; }

            ray_direction
        },
        Material::Dielectric{ albedo: _albedo, refraction: _refraction, eta: _eta, fuzz, emit: _emit } =>
        {
            let mut ray_direction: Vec3 = ((*ray).direction | normal).unit();
            if fuzz > 0.0 { ray_direction += fuzz.sqrt()*Vec3::rng_unit(); }

            ray_direction.unit()
        },

        //-!-!-!-DEBUG ONLY MATERIAL-!-!-!-//
        Material::DebugNormalShading { normal: _normal, mode: _mode } =>
        {
            Vec3::default()
        },
        Material::DebugNormalRaycasting {} =>
        {
            normal
        },


        _ => { Vec3::default() }
    }
}

pub fn refract(ray: &mut Ray, stat: &BoolObjF64) -> Vec3
{
    let r_in_dir: Vec3 = ray.direction.unit();
    let normal: Vec3 = (ray.at( stat.f ) - stat.obj.pos).unit();
    let cos_theta: f64 = -r_in_dir*normal;
    let sin_theta: f64 = (1f64 - cos_theta*cos_theta).abs().sqrt();
    let ratio: f64 = if !(r_in_dir&normal) { 1.0 / stat.obj.material.eta() } else { stat.obj.material.eta() };
    // let ratio: f64 = get_stat_eta(&Ray::new(ray.at(stat.f), if r_in_dir&normal { -normal } else { normal }, Color::default()), list_hitable, stat.obj)
    //                / get_stat_eta(&Ray::new(ray.at(stat.f), if r_in_dir&normal { normal } else { -normal }, Color::default()), list_hitable, stat.obj);

    return match stat.obj.material
    {
        Material::Dielectric { albedo: _albedo, refraction: _refraction, eta: _eta, fuzz, emit: _emit } =>
            {
                if sin_theta*ratio <= 1f64
                {
                    let r_out_dir_perp: Vec3 = ratio * (r_in_dir + cos_theta * normal);
                    let r_out_dir_para: Vec3 = -(1.0 - r_out_dir_perp.len2()).abs().sqrt() * if r_in_dir&normal { -normal } else { normal };
                    let ray_direction: Vec3 = r_out_dir_perp + r_out_dir_para;

                    ray_direction.unit()
                }
                else
                {
                    let ray_direction: Vec3 = r_in_dir | if r_in_dir&normal { -normal } else { normal };

                    ray_direction.unit()
                }
            },

        _ => { Vec3::default() }
    }
}