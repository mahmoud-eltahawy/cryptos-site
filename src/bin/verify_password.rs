use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: verify_password <password> <hash>");
        eprintln!("Example: verify_password admin123 '$argon2id$v=19$...'");
        std::process::exit(1);
    }

    let password = &args[1];
    let hash = &args[2];

    println!("Testing password verification...");
    println!("Password: {}", password);
    println!("Hash: {}", hash);
    println!();

    match password_auth::verify_password(password, hash) {
        Ok(_) => {
            println!("✅ SUCCESS: Password verification passed!");
            std::process::exit(0);
        }
        Err(e) => {
            println!("❌ FAILED: Password verification failed!");
            println!("Error: {:?}", e);
            println!();
            println!("Generating new hash for this password:");
            let new_hash = password_auth::generate_hash(password);
            println!("New hash: {}", new_hash);
            println!();
            println!("SQL to update:");
            println!("UPDATE users SET password = '{}' WHERE name = 'admin';", new_hash);
            std::process::exit(1);
        }
    }
}
