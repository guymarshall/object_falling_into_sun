fn main() {
    const GRAVITATIONAL_CONSTANT: f64 = 0.000000000066743;
    const MASS_OF_SUN_KG: f64 = 1988470000000000000000000000000.0;
    const DISTANCE_BETWEEN_EARTH_AND_SUN_M: f64 = 151480000000.0;

    let delta_t: f64 = 1.0;
    let mut time: f64 = 0.0;
    let mut distance_from_sun: f64 = DISTANCE_BETWEEN_EARTH_AND_SUN_M;
    let mut acceleration: f64;
    let mut velocity: f64 = 0.0;
    let mut delta_v: f64;
    let mut delta_s: f64;

    while distance_from_sun > 0.0 {
        acceleration = -1.0 * (GRAVITATIONAL_CONSTANT * MASS_OF_SUN_KG) / (distance_from_sun * distance_from_sun);
        delta_v = acceleration * delta_t;
        velocity += delta_v;
        delta_s = velocity * delta_t;
        distance_from_sun += delta_s;

        println!("Time: {}s, acceleration: {}ms^-2, velocity: {}ms^-1, distance: {}m", time, acceleration, velocity, distance_from_sun);

        time += delta_t;
    }
}
