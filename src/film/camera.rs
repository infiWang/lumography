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
    pub fn new(nPos: Pos3, nDirect: Vec3, nR: f64, nF: f64) -> Camera
    {
        Camera{ pos: nPos, direction: nDirect, aspectRatio: nR, focal: nF }
    }
    pub fn default() -> Camera
    {
        Camera{ pos: Pos3::new(0.0, 0.0, 0.0), direction: Vec3::new(0.0, 0.0, 0.0), aspectRatio: 0.0, focal: 0.0 }
    }
}