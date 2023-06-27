use crypto::digest::Digest;
use crypto::sha2::Sha256;

fn encrypt_password(password: &str) -> String {
    // Create a new SHA-256 hash object
    let mut sha256 = Sha256::new();

    // Hash the password
    sha256.input_str(password);

    // Get the hexadecimal representation of the hashed password
    let encrypted_password = sha256.result_str();

    encrypted_password
}

fn main() {
    // Example usage
    let password = String::from("password123");
    let encrypted_password = encrypt_password(&password);
    println!("Encrypted password: {}", encrypted_password);
}
