use ansi_term::Colour::Blue;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Write, Read, stdin, stdout};
use magic_crypt::{new_magic_crypt, MagicCryptTrait};


const SAVE_FILE: &str = "save.txt";


fn main() {
    log_starting_screen();
    verify_first_run();
    main_loop();
}


fn log_starting_screen() {
    // Password
    println!("{}", Blue.paint("/$$$$$$$                                                                      /$$ "));
    println!("{}", Blue.paint("| $$__  $$                                                                    | $$"));
    println!("{}", Blue.paint("| $$  \\ $$ /$$$$$$   /$$$$$$$ /$$$$$$$ /$$  /$$  /$$  /$$$$$$   /$$$$$$   /$$$$$$$"));
    println!("{}", Blue.paint("| $$$$$$$/|____  $$ /$$_____//$$_____/| $$ | $$ | $$ /$$__  $$ /$$__  $$ /$$__  $$"));
    println!("{}", Blue.paint("| $$____/  /$$$$$$$|  $$$$$$|  $$$$$$ | $$ | $$ | $$| $$  \\ $$| $$  \\__/| $$  | $$"));
    println!("{}", Blue.paint("| $$      /$$__  $$ \\____  $$\\____  $$| $$ | $$ | $$| $$  | $$| $$      | $$  | $$"));
    println!("{}", Blue.paint("| $$     |  $$$$$$$ /$$$$$$$//$$$$$$$/|  $$$$$/$$$$/|  $$$$$$/| $$      |  $$$$$$$"));
    println!("{}", Blue.paint("|__/      \\_______/|_______/|_______/  \\_____/\\___/  \\______/ |__/       \\_______/"));

    // Manager
    println!("{}", Blue.paint(" /$$      /$$"));
    println!("{}", Blue.paint("| $$$    /$$$"));
    println!("{}", Blue.paint("| $$$$  /$$$$  /$$$$$$  /$$$$$$$   /$$$$$$   /$$$$$$   /$$$$$$   /$$$$$$"));
    println!("{}", Blue.paint("| $$ $$/$$ $$ |____  $$| $$__  $$ |____  $$ /$$__  $$ /$$__  $$ /$$__  $$"));
    println!("{}", Blue.paint("| $$  $$$| $$  /$$$$$$$| $$  \\ $$  /$$$$$$$| $$  \\ $$| $$$$$$$$| $$  \\__/"));
    println!("{}", Blue.paint("| $$\\  $ | $$ /$$__  $$| $$  | $$ /$$__  $$| $$  | $$| $$_____/| $$"));
    println!("{}", Blue.paint("| $$ \\/  | $$|  $$$$$$$| $$  | $$|  $$$$$$$|  $$$$$$$|  $$$$$$$| $$"));
    println!("{}", Blue.paint("|__/     |__/ \\_______/|__/  |__/ \\_______/ \\____  $$ \\_______/|__/"));
    println!("{}", Blue.paint("                                            /$$  \\ $$"));
    println!("{}", Blue.paint("                                           |  $$$$$$/"));
    println!("{}", Blue.paint("                                            \\______/"));
}


fn verify_first_run() {
    let res: Result<File, std::io::Error> = File::open(SAVE_FILE);

    let file = res.ok();

    // File already exists. Not first run.
    if file.is_some() {
        return;
    }

    manage_password_creation();
}


fn manage_password_creation() {
    println!("PASSWORD CREATION");

    loop {
        if password_creation_iteration() {
            break;
        }
    }
}


fn password_creation_iteration() -> bool {
    stdout().flush().expect("Failed to flush");

    let mut password: String = String::new();
    stdin().read_line(&mut password).expect("Failed to read password.");

    // Remove newline
    password = password.trim().to_string();

    let length: usize = password.chars().count();
    if length == 0 {
        println!("Password cannot be empty.");
        return false;
    }

    print!("Please repeat your password:    ");
    stdout().flush().expect("Failed to flush");

    let mut repeated_password: String = String::new();
    stdin().read_line(&mut repeated_password).expect("Failed to read repeated password.");

    // Remove newline
    repeated_password = repeated_password.trim().to_string();

    if repeated_password != password {
        println!("Passwords do not match.");
        return false;
    }

    create_save_file(&password);
    return true;
}


fn create_save_file(password: &str) {
    let passwords: HashMap<String, String> = HashMap::new();
    save_passwords(&passwords, password);
}


fn encrypt(string: &str, password: &str) -> String {
    let mc: magic_crypt::MagicCrypt256 = new_magic_crypt!(password, 256);

    let base64: String = mc.encrypt_str_to_base64(string);
    base64
}


fn decrypt(base64: &str, password: &str) -> String {
    let mc: magic_crypt::MagicCrypt256 = new_magic_crypt!(password, 256);

    let string: String = mc.decrypt_base64_to_string(base64).expect("Failed to decrypt");
    string
}


fn main_loop() {
    println!("MAIN LOOP");
    print!("Please enter your password:    ");
    stdout().flush().expect("Failed to flush");

    let mut password: String = String::new();
    stdin().read_line(&mut password).expect("Failed to read password.");
    password = password.trim().to_string();

    let mut file: File = File::open(SAVE_FILE).expect("Failed to open file.");
    let mut file_contents: String = String::new();
    file.read_to_string(&mut file_contents).expect("Failed to read file.");

    println!("{} {}", file_contents, password);

    let json_str: String = decrypt(&file_contents, &password);

    println!("{}", json_str);

    let passwords: Result<HashMap<String, String>, serde_json::Error> = serde_json::from_str(&json_str);

    loop {
        let mut password_name: String = String::new();
        stdin().read_line(&mut password_name).expect("Failed to read password.");
        password_name = password_name.trim().to_string();
    }
}
fn save_passwords(passwords: &HashMap<String, String>, password: &str) {
    let json_str: String = serde_json::to_string(passwords).expect("Failed to serialize passwords.");
    let encrypted_json: String = encrypt(&json_str, password);

    let mut file: File = File::create(SAVE_FILE).expect("Failed to create file");
    file.write_all(encrypted_json.as_bytes()).expect("Failed to write file");
}