use std::f64; // for sqrt()
use std::ops::{Add, Div, Mul, Sub}; // for op def on Vec3

pub mod integrators;


// basic units we will need
pub mod units {
    // SI constants
    pub const G: f64 = 6.674_30e-11; //  m^3 kg^-1 s^-2
    pub const C: f64 = 299_792_458.0; //  m s^-1
    pub const M_SUN: f64 = 1.988_92e30; //  kg

    // Schwarzschild radius r_s = 2GM/c^2   (in meters)
    pub fn schwarzschild_radius(mass_kg: f64) -> f64 {
        // TODO: implement
        // 2.0 * G * mass_kg / (C * C)
        (2.0 * G * mass_kg) / (C * C)
    }
}

// define Vec3 for 3D space
#[derive(Clone, Copy, Debug, Default)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

// implement basic functions
impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    pub fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    pub fn normalized(&self) -> Self {
        let n = self.norm();
        if n == 0.0 {
            return *self;
        } else {
            Self::new(self.x / n, self.y / n, self.z / n)
        }
    }
    pub fn dot(&self, r: &Self) -> f64 {
        self.x * r.x + self.y * r.y + self.z * r.z
    }
    pub fn cross(&self, r: &Self) -> Self {
        Self::new(
            self.y * r.z - self.z * r.y,
            self.z * r.x - self.x * r.z,
            self.x * r.y - self.y * r.x,
        )
    }
}

// define arithmetic ops for Vec3
// first time I do this in Rust
// a bit awkward compared to C++ or Swift
// very clear syntax though, difficult to mess up
impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, s: f64) -> Self {
        Self::new(self.x * s, self.y * s, self.z * s)
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, s: f64) -> Self {
        Self::new(self.x / s, self.y / s, self.z / s)
    }
}

// Black Hole data structure
pub struct BlackHole {
    pub mass_kg: f64,
}
impl BlackHole {
    pub fn from_solar_mass(m: f64) -> Self {
        Self {
            mass_kg: m * units::M_SUN,
        }
    }
    pub fn r_s(&self) -> f64 {
        units::schwarzschild_radius(self.mass_kg)
    }
}

// Newton accel: -GM / r^2
pub fn newtonian_accel(pos: Vec3, bh: &BlackHole) -> Vec3 {
    let r = pos.norm();
    if r == 0.0 {
        return Vec3::default();
    }
    let a_mag = -units::G * bh.mass_kg / (r * r);
    pos.normalized() * a_mag
}
// Newton orbital speed: sqrt(GM/r0)
pub fn newton_orbit_speed(bh: &BlackHole, radius: f64) -> f64 {
    (units::G * bh.mass_kg / radius).sqrt()
}

// PW-accel
pub fn pw_accel(pos: Vec3, bh: &BlackHole) -> Vec3 {
    let r = pos.norm();
    let r_s = bh.r_s();
    if r <= r_s {
        return Vec3::default(); // treat this as absorbed
    }
    let a_mag = -units::G * bh.mass_kg / ((r - r_s) * (r - r_s));
    pos.normalized() * a_mag
}
// PW orbital speed
pub fn pw_orbital_speed(bh: &BlackHole, radius: f64) -> f64 {
    (units::G * bh.mass_kg * radius).sqrt() / (radius - bh.r_s())
}

#[cfg(test)]
mod tests {
    use super::units::*;
    #[test]
    fn schwarzschild_of_sun_about_2953m() {
        let rs = schwarzschild_radius(M_SUN);
        assert!((rs - 2953.339_382).abs() < 1.0);
    }
}
