fn main() {
    let bh = grav_core::BlackHole::from_solar_mass(1.0);
    println!("Schwarzschild radius of (Sun): {:.3} m", bh.r_s());
}
