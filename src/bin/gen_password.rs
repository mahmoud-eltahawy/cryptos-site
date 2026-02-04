use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: gen_password <password>");
        eprintln!("Example: gen_password admin123");
        std::process::exit(1);
    }

    let password = &args[1];
    let hash = password_auth::generate_hash(password);

    println!("Password: {}", password);
    println!("Hash: {}", hash);
    println!();
    println!("To update in database:");
    println!("UPDATE users SET password = '{}' WHERE name = 'admin';", hash);
}
