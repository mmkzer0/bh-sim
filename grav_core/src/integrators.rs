use crate::{BlackHole, Vec3};

// Accel function signature: a = a(pos, bh)
pub type AccelFn = fn(pos: Vec3, bh: &BlackHole) -> Vec3;

// State data type
#[derive(Clone, Copy, Debug)]
pub struct State {
    pub pos: Vec3,
    pub vel: Vec3,
}

impl State {
    pub fn new(pos: Vec3, vel: Vec3) -> Self {
        Self { pos, vel }
    }
}

// test if particle got absorbed
fn absorbed(pos: Vec3, bh: &BlackHole) -> bool {
    pos.norm() <= 1.01 * bh.r_s() // small buffer above the horizon
}

// Velocity-Verlet Step
// Equations per unit mass:
//      a0      = a(x_n)
//      x_{n+1} = x_n + v_n * dt + 0.5 * a0 * dt^2
//      a1      = a(x_{n+1})
//      v_{n+1} = v_n + 0.5 * (a0 + a1) * dt
pub fn step_velocity_verlet(state: &mut State, dt: f64, bh: &BlackHole, accel: AccelFn) {
    // a_n
    let a0 = accel(state.pos, bh);

    // x_{n+1} 
    let pos_next = state.pos + state.vel * dt + a0 * (0.5 * dt * dt);

    // a_{n+1}
    let a1 = accel(pos_next, bh);

    // v_{n+1}
    let vel_next = state.vel + (a0 + a1) * dt * 0.5;

    state.pos = pos_next;
    state.vel = vel_next;
}
