fn main() {
    vector_test();
    string_test();
    map_test();
}

fn vector_test() {
    // let v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3];
    v.push(4);
    v.push(5);
    v.push(6);

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(3) {
        Some(forth) => println!("The forth element is {}", forth),
        None => println!("There is no forth element."),
    }

    for i in &v {
        println!("{}", i);
    }

    for i in &mut v {
        *i += 10;
    }

    for i in &v {
        println!("{}", i);
    }

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];

    for i in &row {
        println!("{:?}", i);
    }
}

fn string_test() {
    // let s = String::new();

    let data = "initial contents";
    let s = data.to_string();
    println!("{}", s);

    let hello = String::from("السلام عليكم");
    println!("{}", hello);
    let hello = String::from("Dobrý den");
    println!("{}", hello);
    let hello = String::from("Hello");
    println!("{}", hello);
    let hello = String::from("שָׁלוֹם");
    println!("{}", hello);
    let hello = String::from("नमस्ते");
    println!("{}", hello);
    let hello = String::from("こんにちは");
    println!("{}", hello);
    let hello = String::from("안녕하세요");
    println!("{}", hello);
    let hello = String::from("你好");
    println!("{}", hello);
    let hello = String::from("Olá");
    println!("{}", hello);
    let hello = String::from("Здравствуйте");
    println!("{}", hello);
    let hello = String::from("Hola");
    println!("{}", hello);

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{}", s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("{}", s1);
    s1.pop();
    s1.pop();
    s1.pop();
    println!("{}", s1);

    // +运算符使用了add函数，像这样：
    // fn add(self, s: &str) -> String{}
    let s3 = s1 + &s2;
    println!("{}", s3);
    println!("{}", s3.len());

    let s4 = "测试字符串";
    for i in s4.chars(){
        println!("{}", i);
    }
    for i in s4.bytes(){
        println!("{}", i);
    }
}

fn map_test(){
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blur"), 10);
    scores.insert(String::from("Yellow"), 50);
    // for i in scores{
    //     println!("{:?}", i);
    // }
    for (k, v) in scores{
        println!("{}, {}", k, v);
    }
}
