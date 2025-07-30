pub fn format_number(num: i32) -> String {
    format!("Number: {}", num)
}

pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
