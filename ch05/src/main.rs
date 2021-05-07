struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Coloc(i32, i32, i32);
struct Point(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
    // print_user(user1);
    println!(
        "{}, {}, {}, {}",
        user1.email, user1.username, user1.active, user1.sign_in_count
    );

    let user2 = build_user(
        String::from("builduser@example.com"),
        String::from("build_user"),
    );
    print_user(user2);

    let user3 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername456"),
        ..user1
        // active: user1.active,
        // sign_in_count: user1.sign_in_count,
    };
    print_user(user3);

    let black = Coloc(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("{} {} {}", black.0, black.1, black.2);
    println!("{} {} {}", origin.0, origin.1, origin.2);

    let width1 = 30;
    let height1 = 50;
    println!(
        "Normal: The area of the rectangle is {} square pixels",
        area(width1, height1)
    );

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect1 is {:#?}", rect1);
    println!(
        "Struct: The area of the rectangle is {} square pixels",
        area_by_struct(&rect1)
    );
    println!(
        "Struct_Method: The area of the rectangle is {} square pixels",
        rect1.area()
    );

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let squa1 = Rectangle::square(30);
    println!("Square is {:#?}", squa1);
}

fn print_user(user: User) {
    println!(
        "{}, {}, {}, {}",
        user.email, user.username, user.active, user.sign_in_count
    );
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username, //参数名与字段名完全相同时，可以使用字段初始化简写语法来重写
        active: true,
        sign_in_count: 1,
    }
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_by_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
