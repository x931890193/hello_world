use std::fmt;
use num::complex::Complex;

struct Person {
    name: String,
    age: u8,
}


impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}, {}, rust",
            self.name, self.age
        )
    }
}

#[warn(unused_mut)]
fn main() {
    let en = "world hello";
    let ch = "世界 你好👋";
    let regions = [ch, en];
    for region in regions.iter() {
    // for region in regions {
        println!("{}", region);

    }
    println!("Hello, world!");


    let a = 10;
    let b: i32 = 20;
    let mut c = 30i32;
    let nums = [a, b, c];
    for num in nums {
        println!("{}", num);

    }

    let guess: i32 = "42".parse().expect("need num");

    println!("{}", guess);
        
    let guess = "43".parse::<i32>().expect("need num");
    println!("{}", guess);

    let x = (-42.1_f64).sqrt();
    assert_eq!(x.is_nan(), true);
    for i in 1..=5 {
        println!("{}", i);
    }
    let a = Complex {re: 2.1, im: 1.2};
    let b = Complex::new(2.1, 1.2);
    let result = a + b;
    println!("{} + {}i", result.re, result.im);
    // 测试实现Display特征
    let p = Person {
        name: "test".to_string(),
        age: 18,
    };
    println!("{}", p);

}

fn test() {
    let s = String::from("Hello World");
    let hello = &s[..5]; // &s[0..5]
    let world = &s[6..]; // &s[..11]
    let hello_world = &s[..];
    // 语法🍬
    println!("{}{} {}", hello, world, hello_world)
}

// 寻找第一个单词
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i
        }
    }
    return s.len()

}

fn first_word_v2(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]
        }
    }
    return &s
}

fn first_word_v3(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]
        }
    }
    return &s
}

// 数组切片
fn arr_slice() {
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..]; //
}

// struct
fn test_struct (){
    struct User {
        user_name: String,
        email: String,
        age: i32,
    }
    let mut user = User {
        user_name: String::from("test"),
        email: String::from("@aaa"),
        age: 11,
    };
    println!("{}", user.user_name);
    user.user_name = String::from("change")

}


// tuple struct 元组结构体， 未命名结构体， 匿名结构体 命名元组
fn tuple_struct(){
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let black = Color(0, 0,0);
}
// unit_like struct

fn unit_like_struct() {

}

