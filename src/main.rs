use std::io;

fn main() {
    println!("Welcome to mini-redis!\nPlease input your command\nSupported commands:\n`EXIT`, `exit` to exit;");
    loop {
        let mut cmd = String::new();

        io::stdin()
            .read_line(&mut cmd)
            .expect("Failed to read line");

        let cmd = cmd.trim().to_uppercase();

        if cmd == "EXIT" {
                println!("EXIT command received, shutting down...");
                break;
        } else {
            println!("{cmd}");
            continue;
        }
    }
}
