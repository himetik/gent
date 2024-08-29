use rand::Rng;

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

#[test]
fn test_generate_password_length() {
    let length = 16;
    let password = generate_password(length);
    assert_eq!(password.len(), length);
}

#[test]
fn test_generate_password_contains_chars() {
    let length = 16;
    let password = generate_password(length);
    let contains_expected_chars = password.chars().all(|c| CHARSET.contains(c));
    assert!(contains_expected_chars);
}

#[test]
fn test_generate_password_non_zero_length() {
    let length = 0;
    let password = generate_password(length);
    assert!(password.is_empty());
}

#[test]
fn test_generate_password_default_length() {
    // Запускаем `main` функцию с пустыми аргументами (по умолчанию)
    let args = vec!["gent".to_string()];
    let length = match args.get(1) {
        Some(arg) => match arg.parse::<usize>() {
            Ok(val) if val > 0 => val,
            _ => 14,  // Используем значение по умолчанию
        },
        None => 14,
    };
    
    let password = generate_password(length);
    assert_eq!(password.len(), length);
}
