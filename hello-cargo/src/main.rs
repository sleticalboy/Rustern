
#[derive(Debug)]
struct User {
    username: String,
    email: String,
}

impl User {
    fn say_hello(&self) -> String {
        let mut s = String::from("name: ");
        s.push_str(&*self.username);
        s.push_str(", email: ");
        s.push_str(&*self.email);
        return s;
    }
}


fn main() {
    let mut tom = User {
        username: String::from("tom"),
        email: String::from("tom@tom.com"),
    };
    tom.username = String::from("little tom");
    println!("tom's email is: {}", tom.email);
    println!("tom is: {:?}", tom);
    println!("{}", tom.say_hello());

    // 调用系统宏函数需要叹号
    println!("Hello, rust world!");
    println!("Hello, cargo world!");
    // 自定义函数调用不需要叹号
    my_func();

    let condition = true;
    let num = if condition { 5 } else { 6 };
    println!("num is {num}");
    let mut num = 0;
    // 循环
    loop {
        println!("Loop! {num}");
        num += 1;
        if num >= 10 {
            break
        }
    }
    println!("num is {num}");
    while num > 0 {
        println!("while loop: {num}");
        num -= 1;
    }
    let arr = [1, 2, 3, 4, 5];
    for n in arr {
        println!("arr: {n}");
    }
    for n in (0..5).rev() {
        println!("arr: {n}");
    }

    let s = "hello scop";
    {
        let s = "inner scope";
        println!("s is: {s}");
    }
    println!("s is: {s}");
    let mut s = String::from("Hello");
    s.push_str(", world!");
    println!("s is: {}", s);
    change_str(&mut s);
    let s2 = s;
    println!("s2 is: {}", s2);
    let s3 = s2.clone();
    println!("s3 is: {}", s3);
}

fn change_str(s: &mut String) {
    s.push_str(" ...");
}

// 自定义函数
fn my_func() {
    println!("my_func() called!");
}
