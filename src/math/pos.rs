use crate::math::*;

#[derive(Clone, Copy)]
pub struct Pos2
{
    pub x: f64,
    pub y: f64,
}
#[derive(Clone, Copy)]
pub struct Pos3
{
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
#[derive(Clone, Copy)]
pub struct Pos4
{
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub h: f64,
}

impl Pos3
{
    #[inline]
    pub fn default() -> Self
    {
        Pos3 { x: 0.0, y: 0.0, z: 0.0 }
    }
    #[inline]
    pub fn new(nx: f64, ny:f64, nz: f64) -> Pos3
    {
        Pos3 { x: nx, y: ny, z: nz }
    }
}

impl std::ops::Add<Vec3> for Pos3
{
    type Output = Pos3;
    #[inline]
    fn add(self, vec3: Vec3) -> Pos3
    {
        Pos3::new(self.x + vec3.x, self.y + vec3.y,self.z + vec3.z)
    }
}
impl std::ops::AddAssign<Vec3> for Pos3
{
    #[inline]
    fn add_assign(&mut self, vec3: Vec3)
    {
        *self = Pos3::new(self.x + vec3.x, self.y + vec3.y, self.z + vec3.z);
    }
}
impl std::ops::Sub<Vec3> for Pos3
{
    type Output = Pos3;
    #[inline]
    fn sub(self, vec3: Vec3) -> Pos3
    {
        Pos3::new(self.x - vec3.x, self.y - vec3.y,self.z - vec3.z)
    }
}
impl std::ops::SubAssign<Vec3> for Pos3
{
    #[inline]
    fn sub_assign(&mut self, vec3: Vec3)
    {
        *self = Pos3::new(self.x - vec3.x, self.y - vec3.y, self.z - vec3.z);
    }
}
impl std::ops::Sub<Pos3> for Pos3
{
    type Output = Vec3;
    #[inline]
    fn sub(self, pos3: Pos3) -> Vec3
    {
        Vec3::new(self.x - pos3.x, self.y - pos3.y,self.z - pos3.z)
    }
}