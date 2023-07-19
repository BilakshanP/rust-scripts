use std::f64::consts::PI;

const PI_BY_D180: f64 = PI / 180.0;
const D180_BY_PI: f64 = 180.0 / PI;

#[derive(Clone, Copy)]
pub struct Angle {
    deg: f64,
    rad: f64
}

impl Angle {
    pub fn new_from_degrees(deg: f64) -> Self {
        Angle { deg, rad: deg * PI_BY_D180 }
    }

    pub fn new_from_radians(rad: f64) -> Self {
        Angle { deg: rad * D180_BY_PI, rad }
    }

    pub fn normalize(&self) -> Self {
        *self % 360.0
    }

    pub fn get(&self) -> (f64, f64) {
        (self.deg, self.rad)
    }

    pub fn get_degrees(&self) -> f64 {
        self.deg
    }

    pub fn get_radians(&self) -> f64 {
        self.rad
    }

    pub fn sin(&self) -> f64 {
        self.rad.sin()
    }

    pub fn cos(&self) -> f64 {
        self.rad.cos()
    }

    pub fn tan(&self) -> f64 {
        self.rad.tan()
    }
}

impl std::fmt::Display for Angle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}deg", self.deg)
    }
}

impl std::fmt::Debug for Angle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "< {}deg {}rad >", self.deg, self.rad)
    }
}

impl std::ops::Add for Angle {
    type Output = Self;
    
    fn add(self, other: Self) -> Self::Output {
        Self { deg: self.deg + other.deg, rad: self.rad + other.rad }
    }
}

impl std::ops::Sub for Angle {
    type Output = Self;
    
    fn sub(self, other: Self) -> Self::Output {
        Self { deg: self.deg - other.deg, rad: self.rad - other.rad }
    }
}

impl std::ops::Mul<f64> for Angle {
    type Output = Self;
    
    fn mul(self, num: f64) -> Self::Output {
        Self { deg: self.deg * num, rad: self.rad * num }
    }
}

impl std::ops::Div<f64> for Angle {
    type Output = Self;
    
    fn div(self, num: f64) -> Self::Output {
        Self { deg: self.deg / num, rad: self.rad / num }
    }
}

impl std::ops::Rem<f64> for Angle {
    type Output = Self;

    fn rem(self, num: f64) -> Self::Output {
        Self::new_from_degrees(self.deg % num)
    }
}

impl std::ops::Mul<Angle> for f64 {
    type Output = Angle;

    fn mul(self, angle: Angle) -> Self::Output {
        angle * self
    }
}

impl std::ops::Neg for Angle {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self { deg: -self.deg, rad: -self.rad }
    }
}

impl std::cmp::PartialEq for Angle {
    fn eq(&self, other: &Self) -> bool {
        self.rad == other.rad
    }
}