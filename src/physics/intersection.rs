use crate::{math::*};
use crate::{object::*};
use crate::physics::{ray::*};

const ZERO: f64 = 0.0001;

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
        // println!("r*s, a:{} b:{} c:{} delta:{}", a, b, c, discriminant);

        let mut res: BoolF64 = BoolF64::new(false, 0.0);
        if discriminant_quarter > 0.0
        {
            let sqrt_d: f64 = discriminant_quarter.sqrt();

            let r1: f64 = (-b_half - sqrt_d) / a;
            let r2: f64 = (-b_half + sqrt_d) / a;
            if r1 > ZERO
            {
                res.bool = true;
                res.f = r1;
            }
            else if r2 > 0.001
            {
                res.bool = true;
                res.f = r2;
            }
        }

        res
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

            let r1: f64 = (-b_half - sqrt_d) / a;
            let r2: f64 = (-b_half + sqrt_d) / a;
            if r1 > ZERO
            {
                res.bool = true;
                res.f = r1;
            }
            else if r2 > 0.001
            {
                res.bool = true;
                res.f = r2;
            }
        }

        res
    }
}

pub fn hitStat(ray: &Ray, listHitable: &Vec<Sphere>) -> BoolObjF64
{
    let mut list = listHitable.clone();
    let mut statClosest: BoolObjF64 = BoolObjF64::default();

    for obj in list.iter()
    {
        let stat: BoolF64 = (*obj)*(*ray);
        if stat.bool && stat.f > 0.001
        {
            // println!("Intersects! ");

            if !statClosest.bool
            {
                statClosest = BoolObjF64::new(stat.bool, (*obj).clone(), stat.f);
            }
            else if stat.f < statClosest.f
            {
                statClosest = BoolObjF64::new(stat.bool, (*obj).clone(), stat.f);
            }
        }
    }

    statClosest
}