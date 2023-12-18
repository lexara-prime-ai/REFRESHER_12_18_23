use std::io;

#[allow(dead_code)]
#[derive(Debug)]
struct LoginCredentials<'a> {
    email: &'a str,
    password: &'a str,
}

impl<'a> LoginCredentials<'a> {
    fn read_user_input() {
        println!("* * * AUTHENTICATION * * *");
        // INPUT -> email
        println!("Please provide an email e.g johndoe@gmail.com...");
        let mut email = String::new();
        io::stdin()
            .read_line(&mut email)
            .expect("ERROR:: Failed to read user input...");
        // INPUT -> password
        println!("Please provide a password(> 8 characters recommended)");
        let mut password = String::new();
        io::stdin()
            .read_line(&mut password)
            .expect("Failed to read user input...");

        LoginCredentials::parse_user_input(email.as_str(), &password.as_str());
    }

    fn parse_user_input(email: &'a str, password: &'a str) -> Self {
        println!(
            "* * * Received * * *\nEMAIL: {}\nPASSWORD: {}",
            email, password
        );
        let user = LoginCredentials { email, password };
        user
    }
}

fn main() {
    // Call
    LoginCredentials::read_user_input();
}
