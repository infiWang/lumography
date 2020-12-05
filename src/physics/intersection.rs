use crate::{math::*};
use crate::{object::*};
use crate::physics::{ray::*};

const ZERO: f64 = 0.001;


// SYMBOL& is "reloaded" to determine whether ray and sphere are intersected.
impl std::ops::BitAnd<Sphere> for Ray
{
    type Output = bool;
    #[inline]
    fn bitand(self, sphere: Sphere) -> bool
    {
        let oc: Vec3 = self.origin - sphere.pos;
        let a: f64 = self.direction * self.direction;
        let b_half: f64 = oc * self.direction;
        let c: f64 = oc * oc - sphere.r*sphere.r;
        let discriminant_quarter: f64 = b_half*b_half - a*c;

        if discriminant_quarter > 0.0
        {
            let sqrt_d: f64 = discriminant_quarter.sqrt();
            (-b_half - sqrt_d) / a > ZERO || (-b_half + sqrt_d) / a > ZERO
        }
        else { false }
    }
}
impl std::ops::BitAnd<Ray> for Sphere
{
    type Output = bool;
    #[inline]
    fn bitand(self, ray: Ray) -> bool
    {
        let oc: Vec3 = ray.origin - self.pos;
        let a: f64 = ray.direction * ray.direction;
        let b_half: f64 = oc * ray.direction;
        let c: f64 = oc*oc - self.r*self.r;
        let discriminant_quarter: f64 = b_half*b_half - a*c;

        if discriminant_quarter > 0.0
        {
            let sqrt_d: f64 = discriminant_quarter.sqrt();

            (-b_half - sqrt_d) / a > ZERO || (-b_half + sqrt_d) / a > ZERO
        }
        else { false }
    }
}

impl std::ops::Mul<Sphere> for Ray
{
    type Output = BoolF64;
    #[inline]
    fn mul(self, sphere: Sphere) -> BoolF64
    {
        let oc: Vec3 = self.origin - sphere.pos;
        let a: f64 = self.direction * self.direction;
        let b_half: f64 = oc * self.direction;
        let c: f64 = oc * oc - sphere.r*sphere.r;
        let discriminant_quarter: f64 = b_half*b_half - a*c;

        let mut res: BoolF64 = BoolF64::new(false, 0.0);
        if discriminant_quarter > 0.0
        {
            let sqrt_d: f64 = discriminant_quarter.sqrt();

            let r_1: f64 = (-b_half - sqrt_d) / a;
            let r_2: f64 = (-b_half + sqrt_d) / a;
            if r_1 > ZERO && r_2 > ZERO
            {
                res.bool = true;
                res.f = if r_1 < r_2 { r_1 } else { r_2 };
            }
            else
            {
                let r_bb = if r_1 < r_2 { r_2 } else { r_1 };
                if r_bb > ZERO { res.bool = true; res.f = r_bb; }
            }
        }

        return res;
    }
}
impl std::ops::Mul<Ray> for Sphere
{
    type Output = BoolF64;
    #[inline]
    fn mul(self, ray: Ray) -> BoolF64
    {
        let oc: Vec3 = ray.origin - self.pos;
        let a: f64 = ray.direction * ray.direction;
        let b_half: f64 = oc * ray.direction;
        let c: f64 = oc*oc - self.r*self.r;
        let discriminant_quarter: f64 = b_half*b_half - a*c;
        // println!("s*r, a:{} b:{} c:{} delta:{}", a, b, c, discriminant);

        let mut res: BoolF64 = BoolF64::new(false, 0.0);
        if discriminant_quarter > 0.0
        {
            let sqrt_d: f64 = discriminant_quarter.sqrt();

            let r_1: f64 = (-b_half - sqrt_d) / a;
            let r_2: f64 = (-b_half + sqrt_d) / a;
            if r_1 > ZERO && r_2 > ZERO
            {
                res.bool = true;
                res.f = if r_1 < r_2 { r_1 } else { r_2 };
            }
            else
            {
                let r_bb = if r_1 < r_2 { r_2 } else { r_1 };
                if r_bb > ZERO { res.bool = true; res.f = r_bb; }
            }
        }

        return res;
    }
}

pub fn get_stat_hit(ray: &Ray, list_hitable: &Vec<Sphere>) -> BoolObjF64
{
    let mut list = list_hitable.clone();
    let mut stat_closest: BoolObjF64 = BoolObjF64::default();

    for obj in list.iter()
    {
        let stat: BoolF64 = (*obj)*(*ray);
        if stat.bool && stat.f > ZERO
        {
            if !stat_closest.bool
            {
                stat_closest = BoolObjF64::new(stat.bool, (*obj).clone(), stat.f);
            }
            else if stat.f < stat_closest.f
            {
                stat_closest = BoolObjF64::new(stat.bool, (*obj).clone(), stat.f);
            }
        }
    }

    return stat_closest;
}
pub fn get_stat_eta(ray: &Ray, list_hitable: &Vec<Sphere>, cur_obj_stat: Sphere) -> f64
{
    let mut stat_closest: BoolObjF64 = BoolObjF64::default();
    let mut eta = 1.0;

    for obj in (*list_hitable).iter()
    {
        let stat: BoolF64 = (*obj)*(*ray);
        if stat.bool && stat.f > ZERO
        {
            if !stat_closest.bool
            {
                stat_closest = BoolObjF64::new(stat.bool, (*obj).clone(), stat.f);
                eta = stat_closest.obj.material.eta();
            }
            else if stat.f < stat_closest.f
            {
                stat_closest = BoolObjF64::new(stat.bool, (*obj).clone(), stat.f);
                eta = stat_closest.obj.material.eta();
            }
        }
    }

    return if cur_obj_stat != stat_closest.obj { 1f64 } else { eta };
}