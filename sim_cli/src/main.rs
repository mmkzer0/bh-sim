use grav_core::{integrators::{step_velocity_verlet, State}, newton_orbit_speed, newtonian_accel, Vec3};

fn main() {
    // quick r_s sanity check
    let bh = grav_core::BlackHole::from_solar_mass(1.0);
    println!("Schwarzschild radius of (Sun): {:.3} m", bh.r_s());

    let m10_bh = grav_core::BlackHole::from_solar_mass(10.0);

    let r_0 = 20.0 * m10_bh.r_s();
    let v = newton_orbit_speed(&m10_bh, r_0);

    let pos1: Vec3 = Vec3::new(r_0, 0.0, 0.0);
    let vel1: Vec3 = Vec3::new(0.0, v, 0.0);
    let dt = 0.01 * r_0 / v;
    let mut state: State = State { pos: pos1, vel: vel1 };

    for i in 0..100  {
        step_velocity_verlet(&mut state, dt, &m10_bh, newtonian_accel);
        println!("Position {:?}, Velocity {:?}.\n", state.pos, state.vel);
    }


}
