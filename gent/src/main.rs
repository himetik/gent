use rand::Rng;
use std::env;
use std::process;

const CHARSET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()_-+=<>?";

fn generate_password(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let password: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET.chars().nth(idx).unwrap()
        })
        .collect();
    password
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <length>", args[0]);
        process::exit(1);
    }

    let length: usize = args[1].parse().unwrap_or_else(|_| {
        eprintln!("Please provide a valid number for password length.");
        process::exit(1);
    });

    let password = generate_password(length);
    println!("{}", password);
}

