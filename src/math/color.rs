#[derive(Copy, Clone)]
pub struct Color
{
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

#[derive(Copy, Clone)]
pub struct RGB8
{
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
#[derive(Copy, Clone)]
pub struct RGB16
{
    pub r: u16,
    pub g: u16,
    pub b: u16,
}

impl Color
{
    pub fn default() -> Self { Color { r: 0.0, g: 0.0, b: 0.0 } }
    pub fn new(nr: f64, ng: f64, nb: f64) -> Color
    {
        Color { r: nr, g: ng, b: nb }
    }

    #[inline]
    pub fn rgb_empty() -> Color
    {
        Color::new( 0.0, 0.0, 0.0 )
    }
    #[inline]
    pub fn rgb_full() -> Color
    {
        Color::new( 1.0, 1.0, 1.0 )
    }
    #[inline]
    pub fn tidy(&self) -> Color
    {
        let mut nc: Color = *self;
        if nc.r < 0.0 { nc.r = 0.0 };
        if nc.g < 0.0 { nc.g = 0.0 };
        if nc.b < 0.0 { nc.b = 0.0 };

        nc
    }
    #[inline]
    pub fn tidy_self(&mut self)
    {
        if self.r < 0.0 { self.r = 0.0 };
        if self.g < 0.0 { self.g = 0.0 };
        if self.b < 0.0 { self.b = 0.0 };
    }
}

impl PartialEq for Color
{
    fn eq(&self, other: &Self) -> bool
    {
        self.r == other.r && self.g == other.g && self.b == other.b
    }
}
impl std::ops::Add for Color
{
    type Output = Color;
    #[inline]
    fn add(self, color: Color) -> Color
    {
        Color::new(self.r + color.r, self.g + color.g, self.b + color.b)
    }
}
impl std::ops::AddAssign for Color
{
    #[inline]
    fn add_assign(&mut self, color: Color)
    {
        *self = Color::new(self.r + color.r, self.g + color.g, self.b + color.b)
    }
}
impl std::ops::Sub for Color
{
    type Output = Color;
    #[inline]
    fn sub(self, color: Color) -> Color
    {
        Color::new(self.r - color.r, self.g - color.g, self.b - color.b).tidy()
    }
}
impl std::ops::Mul<Color> for Color
{
    type Output = Color;
    #[inline]
    fn mul(self, multiplier: Color) -> Color
    {
        Color::new(self.r * multiplier.r, self.g * multiplier.g, self.b * multiplier.b)
    }
}
impl std::ops::MulAssign<Color> for Color
{
    #[inline]
    fn mul_assign(&mut self, multiplier: Color)
    {
        self.r *= multiplier.r; self.g *= multiplier.g; self.b *= multiplier.b;
    }
}
impl std::ops::Mul<f64> for Color
{
    type Output = Color;
    #[inline]
    fn mul(self, multiplier: f64) -> Color
    {
        Color::new(self.r * multiplier, self.g * multiplier, self.b * multiplier)
    }
}
impl std::ops::Mul<Color> for f64
{
    type Output = Color;
    #[inline]
    fn mul(self, color: Color) -> Color
    {
        Color::new(color.r * self, color.g * self, color.b * self)
    }
}

impl RGB8
{
    pub fn default() -> Self { RGB8{r: 0, g: 0, b: 0} }
    pub fn new(n_r: u8, n_g: u8, n_b: u8) -> RGB8
    {
        RGB8 { r: n_r, g: n_g, b: n_b }
    }
    pub fn gamma_correction(&self) -> RGB8 { RGB8::new(RGB8::GAMMA_TABLE[(self.r as usize)], RGB8::GAMMA_TABLE[(self.g as usize)], RGB8::GAMMA_TABLE[(self.b as usize)]) }

    pub const GAMMA_TABLE:[u8;256] = [0, 20, 28, 33, 38, 42, 46, 49, 52, 55, 58, 61, 63, 66, 68, 70, 72, 74, 76, 78, 80, 82, 83, 85, 87, 88, 90, 92, 93, 95, 96, 98, 99, 100, 102, 103, 104, 106, 107, 108, 110, 111, 112, 113, 114, 116, 117, 118, 119, 120, 121, 122, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 150, 151, 152, 153, 154, 155, 155, 156, 157, 158, 159, 159, 160, 161, 162, 163, 163, 164, 165, 166, 166, 167, 168, 169, 169, 170, 171, 172, 172, 173, 174, 175, 175, 176, 177, 177, 178, 179, 180, 180, 181, 182, 182, 183, 184, 184, 185, 186, 186, 187, 188, 188, 189, 190, 190, 191, 192, 192, 193, 193, 194, 195, 195, 196, 197, 197, 198, 198, 199, 200, 200, 201, 201, 202, 203, 203, 204, 204, 205, 206, 206, 207, 207, 208, 209, 209, 210, 210, 211, 211, 212, 213, 213, 214, 214, 215, 215, 216, 217, 217, 218, 218, 219, 219, 220, 220, 221, 221, 222, 223, 223, 224, 224, 225, 225, 226, 226, 227, 227, 228, 228, 229, 229, 230, 230, 231, 231, 232, 232, 233, 233, 234, 234, 235, 235, 236, 236, 237, 237, 238, 238, 239, 239, 240, 240, 241, 241, 242, 242, 243, 243, 244, 244, 245, 245, 246, 246, 247, 247, 248, 248, 249, 249, 250, 250, 250, 251, 251, 252, 252, 253, 253, 254, 254, 255, 255];
}
impl From<Color> for RGB8
{
    fn from(rgb: Color) -> RGB8
    {
        let c: Color = rgb;
        c.tidy();
        RGB8::new((c.r * 255.999).floor() as u8, (c.g * 255.999).floor() as u8, (c.b * 255.999).floor() as u8)
    }
}

impl PartialEq for RGB8
{
    fn eq(&self, other: &Self) -> bool
    {
        self.r == other.r && self.g == other.g && self.b == other.b
    }
}

impl RGB16
{
    pub fn default() -> Self { RGB16{r: 0, g: 0, b: 0} }
    pub fn new(nr: u16, ng: u16, nb: u16) -> RGB16
    {
        RGB16 { r: nr, g: ng, b: nb }
    }
}