pub fn calculate_acceleration(gravitational_constant: f64, mass: f64, radius: f64) -> f64 {
    return -1 * (gravitational_constant * mass) / (radius * radius);
}
