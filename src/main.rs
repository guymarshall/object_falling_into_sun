mod user_input;

fn main() {
    println!("Choose your object!");
    println!("1 - Mercury");
    println!("2 - Venus");
    println!("3 - Earth");
    println!("4 - Mars");
    println!("5 - Jupiter");
    println!("6 - Saturn");
    println!("7 - Uranus");
    println!("8 - Neptune");
    println!("9 - Pluto");
    println!("10 - Proxima Centauri b");

    let choice: i32 = user_input::get_user_input("Choice");

    let radius: f64 = match choice {
        1 => 69550000000.0,
        2 => 107550000000.0,
        3 => 151480000000.0,
        4 => 211620000000.0,
        5 => 741850000000.0,
        6 => 1474100000000.0,
        7 => 2944700000000.0,
        8 => 4474000000000.0,
        9 => 5900000000000.0,
        10 => 39924000000000000.0,
        _ => 0.0
    };
    let update_every: i32 = user_input::get_user_input("After how many seconds do you want to update?");

    const GRAVITATIONAL_CONSTANT: f64 = 0.000000000066743;
    const MASS_OF_SUN_KG: f64 = 1988470000000000000000000000000.0;

    let delta_t: f64 = 1.0;
    let mut time: f64 = 0.0;
    let mut distance_from_sun: f64 = radius;
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

        if time as i32 % update_every == 0 {
            println!("Time: {}s, acceleration: {}ms^-2, velocity: {}ms^-1, distance: {}m", time, acceleration, velocity, distance_from_sun);
        }

        time += delta_t;
    }
}
