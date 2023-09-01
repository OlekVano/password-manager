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

    let file: Option<File> = res.ok();

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
    print!("Please create your password:    ");
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

    let mut passwords: HashMap<String, String> = serde_json::from_str(&json_str).expect("Failed to get json from file.");

    println!("COMMANDS: ");
    println!("add <name> <password> -> add a password");
    println!("rem <name> -> remove a password");
    println!("get <name> -> get a password");
    println!("edit <name> <password> -> edit a password");

    loop {
        let mut input: String = String::new();
        stdin().read_line(&mut input).expect("Failed to read input.");
        input = input.trim().to_string();

        let words: Vec<&str> = input.split_whitespace().collect();

        if words.len() <= 1 {
            continue;
        }

        match words[0] {
            "add" => add_password(&words, &mut passwords, &password),
            "rem" => remove_password(&words, &mut passwords, &password),
            "get" => get_password(&words, &mut passwords, &password),
            "edit" => edit_password(&words, &mut passwords, &password),
            _ => println!("Invalid command: {}.", words[0])
        }
    }
}

fn add_password(words: &Vec<&str>, passwords: &mut HashMap<String, String>, password: &str) {
    let num_words: usize = words.len();

    if num_words != 3 {
        println!("Invalid number of arguments: {}. Must be 3", num_words);
    }

    let name: &str = words[1];
    let value: &str = words[2];

    if passwords.contains_key(name) {
        println!("Password already exists: {}.", name);
    }

    passwords.insert(name.to_string(), value.to_string());
    save_passwords(passwords, password);
    println!("Successfully added password {}.", name);
}

fn remove_password(words: &Vec<&str>, passwords: &mut HashMap<String, String>, password: &str) {
    let num_words: usize = words.len();

    if num_words != 2 {
        println!("Invalid number of arguments: {}. Must be 2", num_words);
    }

    let name: &str = words[1];

    if !passwords.contains_key(name) {
        println!("Password doesn't exist: {}.", name);
    }

    passwords.remove(name);
    save_passwords(passwords, password);
    println!("Successfully removed password {}.", name);
}

fn get_password(words: &Vec<&str>, passwords: &mut HashMap<String, String>, password: &str) {
    let num_words: usize = words.len();

    if num_words != 2 {
        println!("Invalid number of arguments: {}. Must be 2", num_words);
    }

    let name: &str = words[1];

    if !passwords.contains_key(name) {
        println!("Password doesn't exist: {}.", name);
    }

    println!("{}", passwords.get(name).expect("Failed to get password."));
    save_passwords(passwords, password);
}

fn edit_password(words: &Vec<&str>, passwords: &mut HashMap<String, String>, password: &str) {
    let num_words: usize = words.len();

    if num_words != 3 {
        println!("Invalid number of arguments: {}. Must be 3", num_words);
    }

    let name: &str = words[1];
    let value: &str = words[2];

    if !passwords.contains_key(name) {
        println!("Password doesn't exists: {}.", name);
    }

    passwords.insert(name.to_string(), value.to_string());
    save_passwords(passwords, password);
    println!("Successfully edited password {}.", name);
}

fn save_passwords(passwords: &HashMap<String, String>, password: &str) {
    let json_str: String = serde_json::to_string(passwords).expect("Failed to serialize passwords.");
    let encrypted_json: String = encrypt(&json_str, password);

    let mut file: File = File::create(SAVE_FILE).expect("Failed to create file");
    file.write_all(encrypted_json.as_bytes()).expect("Failed to write file");
}