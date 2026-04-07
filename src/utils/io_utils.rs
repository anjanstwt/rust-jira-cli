use std::io;

pub struct IO {}

impl IO {
    pub fn input(input: Option<&str>, expect: &str) -> String {
        let mut user_input = String::new();

        if let Some(val) = input {
            println!("{val}");
        }

        io::stdin().read_line(&mut user_input).expect(expect);

        user_input
    }
    pub fn wait_for_key_press() {
        io::stdin().read_line(&mut String::new()).unwrap();
    }
}
