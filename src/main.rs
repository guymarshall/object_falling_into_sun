/*
There is a little one that comes to mind: if the Earth stopped in its orbit around the Sun, it would then
accelerate under gravity until it hit the Sun. Using Newton's equation for universal gravitation, you could
calculate the Earth's progress as it falls towards the Sun second by second.

Yes, and for the acceleration (which is what you care about, not the force) you can remove the mass of the
Earth from that equation

It will be similar to your car acceleration problem.

Work out the acceleration, a = -GM/(r²) . Take this as constant for 1s = delta-t .

The change in velocity, delta-v = a × delta-t .

The change in displacement (distance from the Sun) in 1s, delta-s = v × delta-t .

The new displacement, s = s-old + delta-s .
And similar to work out the new v.
*/

mod functions;

fn main() {
    const GRAVITATIONAL_CONSTANT: f64 = 0.000000000066743;
    const MASS_OF_SUN_KG: f64 = 1988470000000000000000000000000.0;
    const DISTANCE_BETWEEN_EARTH_AND_SUN_M: f64 = 151480000000.0;

    let delta_t: f64 = 1.0;
    let mut time: f64 = 0.0;
    let mut distance_from_sun: f64 = DISTANCE_BETWEEN_EARTH_AND_SUN_M;
    let mut acceleration: f64 = 0.0;
    let mut velocity: f64 = 0.0;
    let mut delta_v: f64 = 0.0;
    let mut delta_s: f64 = 0.0;

    while distance_from_sun > 0.0 {
        acceleration = functions::calculate_acceleration(GRAVITATIONAL_CONSTANT, MASS_OF_SUN_KG, distance_from_sun);
        delta_v = acceleration * delta_t;
        velocity += delta_v;
        delta_s = velocity * delta_t;
        distance_from_sun += delta_s;

        println!("Time: {}s, acceleration: {}ms^-2, velocity: {}ms^-1, distance: {}m", time, acceleration, velocity, distance_from_sun);

        time += delta_t;
    }
}
