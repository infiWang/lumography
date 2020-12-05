use crate::{math::*, film::*};

pub struct Camera
{
    pub pos: Pos3,
    pub direction: Vec3,
    pub rotation: f64,
    pub focal: f64,
    pub screen: Screen,

    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3
}

impl Camera
{
    pub fn new(n_pos: Pos3, n_direct: Vec3, n_rot: f64, n_foc: f64, n_scr: Screen) -> Camera
    {
        let n_direct = n_direct.unit();
        Camera{ pos: n_pos, direction: n_direct, rotation: n_rot, focal: n_foc, screen: n_scr
              , u: (n_direct ^ Vec3::k()), v: n_direct, w: (n_direct ^ Vec3::k()) ^ n_direct }
    }
    pub fn default() -> Camera
    {
        Camera{ pos: Pos3::default(), direction: Vec3::i(), rotation: 0f64, focal: 1f64, screen: Screen::default()
              , u: Vec3::i(), v: Vec3::j(), w: Vec3::k() }
    }

    pub fn get_direction(&self, pixel: Pos2) -> Vec3
    {
        let t_cam: Mat3 = Mat3::new(self.u, self.v, self.w);
        let t_scr: Mat3 = Mat3::new(Vec3::i(), Vec3::k(), Vec3::default());
        let t_rot: Mat2 = Mat2::new(Vec2::new(self.rotation.cos(), -self.rotation.sin()), Vec2::new(self.rotation.sin(), self.rotation.cos()));
        let e_scr: Vec3 = (t_rot*self.screen.get_vec(pixel)).into(); let e_scr = t_scr * e_scr;

        return t_cam*(e_scr + Vec3::j()*self.focal*256f64).unit();
    }
}