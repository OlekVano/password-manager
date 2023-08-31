use ansi_term::Colour::Blue;
use std::fs::File;
use std::io::stdin;


const SAVE_FILE: &str = "save.txt";


fn main() {
    log_starting_screen();
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
    println!("Please enter your password:    ");

    let mut password: String = String::new();
    stdin().read_line(&mut password);

    // Remove newline
    password = password.trim().to_string();

    let length: usize = password.chars().count();
    if length == 0 {
        println!("Password cannot be empty.");
        return false;
    }

    println!("Please repeat your password:    ");

    let mut repeated_password: String = String::new();
    stdin().read_line(&mut repeated_password);

    // Remove newline
    repeated_password = repeated_password.trim().to_string();

    if repeated_password != password {
        println!("Passwords do not match.");
        return false;
    }
}