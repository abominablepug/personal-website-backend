use serde::Serialize;
use std::error::Error;
use std::io::{self, Write};
use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy, Serialize)]
struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Mul<f32> for Vector3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Vector3 {
    fn square(self) -> Vector3 {
        Vector3 {
            x: self.x * self.x,
            y: self.y * self.y,
            z: self.z * self.z,
        }
    }
    fn sum(self) -> f32 {
        self.x + self.y + self.z
    }
    fn cross(self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

struct Particle {
    x: Vector3,
    v: Vector3,
    q: f32,
    m: f32,
}

impl Particle {
    fn new(x: Vector3, v: Vector3, q: f32, m: f32) -> Self {
        Self { x, v, q, m }
    }
    fn initialize_boris(&mut self, b: Vector3, e: Vector3, dt: f32) {
        let half_dt = dt / 2.0;
        self.update_boris_velocity(b, e, -half_dt);
    }
    fn update_boris_velocity(&mut self, b: Vector3, e: Vector3, dt: f32) {
        let q_prime = (self.q * dt) / (2.0 * self.m);
        let h = b * q_prime;
        let s = h * (2.0 / (1.0 + h.square().sum()));
        let u = self.v + (e * q_prime);
        let u_prime = u + (u + (u.cross(h))).cross(s);
        self.v = u_prime + (e * q_prime);
    }
    fn update_position(&mut self, dt: f32) {
        self.x = self.x + (self.v * dt);
    }
}

fn get_f32(prompt: &str) -> f32 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        let mut i = String::new();
        io::stdin().read_line(&mut i).unwrap();
        match i.trim().parse::<f32>() {
            Ok(v) => return v,
            Err(_) => println!("Invalid input. Please enter a valid number."),
        }
    }
}

fn get_usize(prompt: &str) -> usize {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        let mut i = String::new();
        io::stdin().read_line(&mut i).unwrap();
        match i.trim().parse::<usize>() {
            Ok(v) => return v,
            Err(_) => println!("Invalid input. Please enter a valid integer."),
        }
    }
}

fn get_vec3(prompt: &str) -> Vector3 {
    println!("Enter components for {}:", prompt);
    let x = get_f32("  x: ");
    let y = get_f32("  y: ");
    let z = get_f32("  z: ");
    Vector3 { x, y, z }
}

fn get_string(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Boris Particle Simulation");
    let B: Vector3 = get_vec3("Magnetic Field (B)");
    let E: Vector3 = get_vec3("Electric Field (E)");

    let DT: f32 = get_f32("Time Step (dt): ");
    let steps: usize = get_usize("Number of Steps: ");

    let pos: Vector3 = get_vec3("Initial Position (x0)");
    let vel: Vector3 = get_vec3("Initial Velocity (v0)");
    let q: f32 = get_f32("Particle Charge (q): ");
    let m: f32 = get_f32("Particle Mass (m): ");

    let mut plot_dim = get_string("Plot Dimensions ('2d' or '3d'): ");
    while plot_dim != "2d" && plot_dim != "3d" {
        println!("Invalid input. Please enter exactly '2d' or '3d'.");
        plot_dim = get_string("Plot Dimensions ('2d' or '3d'): ");
    }

    println!("Starting simulation with {} steps...", steps);

    let mut p = Particle::new(pos, vel, q, m);
    p.initialize_boris(B, E, DT);

    std::fs::write("pos_config.txt", &plot_dim)?;

    let file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("pos_output.csv")?;
    let mut wtr = csv::Writer::from_writer(file);

    for _ in 0..steps {
        p.update_boris_velocity(B, E, DT);
        p.update_position(DT);
        wtr.serialize(&p.x)?;
    }

    wtr.flush()?;
    println!("Simulation complete.");
    println!("Trajectory data saved to 'pos_output.csv'.");
    println!("Plot configuration saved to 'pos_config.txt'.");

    Ok(())
}
