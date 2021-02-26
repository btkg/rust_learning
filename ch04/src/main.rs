fn main() {
    let mut s = String::from("Hello");
    s.push_str(", world!");
    println!("{}", s);

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);

    let s3 = String::from("world");
    let len1 = calculate_length_re(&s3);
    println!("The length of '{}' is {}", s3, len1);

    let mut s4 = String::from("Hello");
    change(&mut s4);
    println!("{}", s4);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

fn calculate_length_re(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", next!");
}
