// 引入系统库
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Hello, guessing game!");
    let target_num = rand::thread_rng().gen_range(1..=100);
    println!("The target num is {target_num}");
    loop {
        println!("Please input your guess.");
        // 使用 let 声明变量，mut 表示可变
        let mut guess = String::new();
        // 从标准输入中读取一行数据
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");
        println!("Your guessed: {guess}");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("Invalid num, err: {err}");
                continue;
            }
        };
        // 比较两个变量
        match guess.cmp(&target_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You guess it!");
                break;
            }
        }
    }
}
