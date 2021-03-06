use rand::{thread_rng, Rng};
use std::thread;
use std::time::Duration;

// fn simulated_expensive_calculation(intensity: u32) -> u32 {
//     println!("calculating slowly...");
//     thread::sleep(Duration::from_secs(2));
//     intensity
// }

fn generate_workout(intensity: u32, random_number: u32) {
    // 将重复的函数调用提取到一个变量里
    // let expensive_result = simulated_expensive_calculation(intensity);

    // 闭包
    let expensive_closure = |num: u32| -> u32 {
        println!("calculation slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure(intensity));
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    // let simulated_random_numbe = rng.gen_range(0..10);

    let mut rng = thread_rng();
    generate_workout(simulated_user_specified_value, rng.gen_range(0..10));

    // 创建一个迭代器
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("Got: {}", val);
    }
}
