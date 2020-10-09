use crate::math::*;

struct Camera
{
    pub pos: Pos3,
    pub direction: Vec3,
    pub aspectRatio: f64,
    pub focal: f64,
}

impl Camera
{
    pub fn new(npos: Pos3, ndirect: Vec3, nr: f64, nf: f64) -> Camera
    {
        Camera{ pos: npos, direction: ndirect, aspectRatio: nr, focal: nf }
    }
    pub fn default() -> Camera
    {
        Camera{ pos: Pos3::new(0.0, 0.0, 0.0), direction: Vec3::new(0.0, 0.0, 0.0), aspectRatio: 0.0, focal: 0.0 }
    }
}