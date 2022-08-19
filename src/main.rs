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
}

// int deltaT = 1;
// int time = 0;
// double distanceFromSun = DISTANCE_BETWEEN_EARTH_AND_SUN_M;
// double acceleration = 0.0;
// double velocity = 0.0;
// double deltaV = 0.0;
// double deltaS = 0.0;

// while (distanceFromSun > 0) {
//     acceleration = Functions.calculateAcceleration(GRAVITATIONAL_CONSTANT, MASS_OF_SUN_KG, distanceFromSun);
//     deltaV = acceleration * deltaT;
//     velocity += deltaV;
//     deltaS = velocity * deltaT;
//     distanceFromSun += deltaS;

//     System.out.printf("Time: %ds, acceleration: %fms^-2, velocity: %fms^-1, distance: %fm%n", time, acceleration, velocity, distanceFromSun);

//     time += deltaT;
// }
