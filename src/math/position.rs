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

impl Pos2
{
    #[inline] pub fn default() -> Self
    {
        Pos2 { x: 0.0, y: 0.0 }
    }
    #[inline] pub fn new(n_x: f64, n_y:f64) -> Pos2
    {
        Pos2 { x: n_x, y: n_y }
    }
}
impl Pos3
{
    #[inline] pub fn default() -> Self
    {
        Pos3 { x: 0.0, y: 0.0, z: 0.0 }
    }
    #[inline] pub fn new(n_x: f64, n_y:f64, n_z: f64) -> Pos3
    {
        Pos3 { x: n_x, y: n_y, z: n_z }
    }
}
impl Pos4
{
    #[inline] pub fn default() -> Self
    {
        Pos4 { x: 0f64, y: 0f64, z: 0f64, h:0f64 }
    }
    #[inline] pub fn new(n_x: f64, n_y:f64, n_z: f64, n_h: f64) -> Pos4
    {
        Pos4 { x: n_x, y: n_y, z: n_z, h: n_h }
    }
}

// Position2(Special Vector2) Operations

impl PartialEq for Pos2
{
    #[inline] fn eq(&self, other: &Self) -> bool
    {
        self.x == other.x && self.y == other.y
    }
}
impl std::ops::Add<Vec2> for Pos2
{
    type Output = Pos2;
    #[inline] fn add(self, vec2: Vec2) -> Pos2
    {
        Pos2::new(self.x + vec2.x, self.y + vec2.y)
    }
}
impl std::ops::AddAssign<Vec2> for Pos2
{
    #[inline] fn add_assign(&mut self, vec2: Vec2)
    {
        (*self) = (*self) + vec2;
    }
}
impl std::ops::Sub<Vec2> for Pos2
{
    type Output = Pos2;
    #[inline] fn sub(self, vec2: Vec2) -> Pos2
    {
        Pos2::new(self.x - vec2.x, self.y - vec2.y)
    }
}
impl std::ops::SubAssign<Vec2> for Pos2
{
    #[inline] fn sub_assign(&mut self, vec2: Vec2)
    {
        (*self) = (*self) - vec2;
    }
}
impl std::ops::Sub<Pos2> for Pos2
{
    type Output = Vec2;
    #[inline] fn sub(self, pos2: Pos2) -> Vec2
    {
        Vec2::new(self.x - pos2.x, self.y - pos2.y)
    }
}

// Position3(Special Vector3) Operations

impl PartialEq for Pos3
{
    #[inline] fn eq(&self, other: &Self) -> bool
    {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}
impl std::ops::Add<Vec3> for Pos3
{
    type Output = Pos3;
    #[inline] fn add(self, vec3: Vec3) -> Pos3
    {
        Pos3::new(self.x + vec3.x, self.y + vec3.y,self.z + vec3.z)
    }
}
impl std::ops::AddAssign<Vec3> for Pos3
{
    #[inline] fn add_assign(&mut self, vec3: Vec3)
    {
        (*self) = (*self) + vec3;
    }
}
impl std::ops::Sub<Vec3> for Pos3
{
    type Output = Pos3;
    #[inline] fn sub(self, vec3: Vec3) -> Pos3
    {
        Pos3::new(self.x - vec3.x, self.y - vec3.y,self.z - vec3.z)
    }
}
impl std::ops::SubAssign<Vec3> for Pos3
{
    #[inline] fn sub_assign(&mut self, vec3: Vec3)
    {
        (*self) = (*self) - vec3;
    }
}
impl std::ops::Sub<Pos3> for Pos3
{
    type Output = Vec3;
    #[inline] fn sub(self, pos3: Pos3) -> Vec3
    {
        Vec3::new(self.x - pos3.x, self.y - pos3.y,self.z - pos3.z)
    }
}

// Convert

impl From<Pos2> for Pos3
{
    fn from(pos2: Pos2) -> Pos3 { Pos3::new(pos2.x, pos2.y, 0f64) }
}