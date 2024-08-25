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
    
    // Длина пароля по умолчанию
    const DEFAULT_LENGTH: usize = 14;

    // Определите длину пароля из аргументов командной строки или используйте значение по умолчанию
    let length = match args.get(1) {
        Some(arg) => match arg.parse::<usize>() {
            Ok(val) if val > 0 => val,
            _ => {
                eprintln!("Please provide a valid positive number for password length.");
                process::exit(1);
            }
        },
        None => DEFAULT_LENGTH,
    };

    let password = generate_password(length);
    println!("{}", password);
}
