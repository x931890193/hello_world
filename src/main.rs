use std::cell::Ref;
use std::cmp::Ordering;
use std::rc::Rc;
use num::complex::Complex;

struct Person {
    name: String, // &str
    age: u8,
}


impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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
    let regions = [ch, en]; // 数组
    for (i, &region) in regions.iter().enumerate() {
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
        name: String::from("test"), // "test".to_string(),
        age: 18,
    };
    println!("{}", p);

    vec_test();
    test_count_iter();
    assert_eq!(5, sum(2i8, 3i8));
    assert_eq!(50, sum(20, 30));
    assert_eq!(2.46, sum(1.23, 1.23));
    // generics  泛型
    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: "Hello", y: '中'};

    let p3 = p1.mix_up(p2);

    assert_eq!(p3.x, 5);
    assert_eq!(p3.y, '中');

    let arr = [1, 2, 3];
    print_array(arr);

    let arr = ["hello", "world"];
    print_array(arr);

    assert_eq!(6, multiply(2u8, 3u8));
    assert_eq!(5.0, multiply(1.0, 5.0));

    // 特征trait
    let pair = Pair{
        x: Unit(1),
        y: Unit(3)
    };

    pair.cmp_display();
    example1();
    example2();
    println!("Success!");

    // 返回特征对象 用 Box 包裹, 编译时不知道类型大小用 智能指针 Box 包裹
    let duck = Duck{};
    duck.swim();
    let bird = hatch_a_bird(2);
    // 变成鸟儿后，它忘记了如何游，因此以下代码会报错
    // bird.swim();
    // 但它依然可以叫唤
    assert_eq!(bird.quack(), "duck duck");

    let bird = hatch_a_bird(1);
    // 这只鸟儿忘了如何飞翔，因此以下代码会报错
    // bird.fly();
    // 但它也可以叫唤
    assert_eq!(bird.quack(), "swan swan");
    println!("Success!");

    // 数组中使用特征对象
    let birds: [Box<dyn Bird>; 2] = [Box::new(Swan{}), Box::new(Duck{})];

    // &dyn and Box<dyn> 包裹和不包裹
    let x = 1.1f64;
    let y = 8u8;
    // draw x
    draw_with_box(Box::new(x));
    // draw y
    draw_with_ref(&y);
    println!("Success!");

    // 静态分发和动态分发Static and Dynamic dispatch
    let x = 5u8;
    let y = "Hello".to_string();

    static_dispatch(x);
    dynamic_dispatch(&y);

    println!("Success!")

    // 一个特征能变成特征对象，首先该特征必须是对象安全的，即该特征的所有方法都必须拥有以下特点：
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
    struct Test;
    let t = Test;
}

// Vec
fn vec_test() {
    let mut v1: Vec<i32> = Vec::new();
    let mut v2 = vec![1, 2, 3];
    v1.push(1);
    v1.push(2);
    v2.push(2);
    let mut v3 : Vec<i32> = Vec::new();
    v3.push(2);
    let first = &v3[0];

    // get index not get value
    if let Some(first) = v3.get(0) {
        println!("this is {}", first)
    }
    // get index not get value
    match v3.get(0) {
        Some(first) => println!("this is {}", first),
        None => (),
    }
}

struct Counter {
    count: u32,
}

// 自定义迭代器
impl Counter {
    // 关联函数
    fn new() -> Counter {
        Counter{count: 0}
    }
}

// 实现Iterator trait
impl Iterator for Counter {
    type Item = u32;
    // 实现next 方法
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 100 {
            Some(self.count)
        }else {
            None
        }
    }
}

fn test_count_iter() {
    let mut count = Counter::new();
    // for 循环遍历
    for c in count{
        println!("{}", c)
    }
}

enum MyOption<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}


fn test3() -> usize {
    let a = Rc::new("ss");
    Rc::strong_count(&a)
}

fn sum<T>(x: T, y: T) -> T
    where
        T: std::ops::Add<Output = T>,
{

    x + y
}

struct Val<T> {
    val: T,
}

impl<T> Val<T> {
    fn value(&self) -> &T {
        &self.val
    }
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    // 实现 mixup，不要修改其它代码！
    fn mix_up<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point{
            x: self.x,
            y: other.y,
        }
    }
}

fn print_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
    println!("{:?}", arr);
}


// 实现 fn multiply 方法
// 如上所述，`+` 需要 `T` 类型实现 `std::ops::Add` 特征
// 那么, `*` 运算符需要实现什么特征呢? 你可以在这里找到答案: https://doc.rust-lang.org/core/ops/
fn multiply<T>(x: T, y: T) -> T
    where
        T: std::ops::Mul + std::ops::Mul<Output = T>,
{
    x * y
}

// 通过两种方法使用特征约束来实现 `fn sum`
// 1 -
// 2 where 语句特征约束
fn sum2<T: std::ops::Add<Output=T>>(x: T, y: T) -> T
    where
        T: std::ops::Add<Output=T>,
{
    x + y
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

#[derive(PartialEq, PartialOrd, Debug)]
struct Unit(i32);

impl<T: std::fmt::Debug + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {:?}", self.x);
        } else {
            println!("The largest member is y = {:?}", self.y);
        }
    }
}




// 填空
fn example1() {
    // `T: Trait` 是最常使用的方式
    // `T: Fn(u32) -> u32` 说明 `T` 只能接收闭包类型的参数
    struct Cacher<T: Fn(u32) -> u32> {
        calculation: T,
        value: Option<u32>,
    }

    impl<T: Fn(u32) -> u32> Cacher<T> {
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: None,
            }
        }

        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                },
            }
        }
    }

    let mut cacher = Cacher::new(|x| x+1);
    assert_eq!(cacher.value(10), 11);
    assert_eq!(cacher.value(15), 11);
}


fn example2() {
    // 还可以使用 `where` 来约束 T
    struct Cacher<T>
        where T: Fn(u32) -> u32,
    {
        calculation: T,
        value: Option<u32>,
    }

    impl<T> Cacher<T>
        where T: Fn(u32) -> u32,
    {
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: None,
            }
        }
        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                },
            }
        }
    }

    let mut cacher = Cacher::new(|x| x+1);
    assert_eq!(cacher.value(20), 21);
    assert_eq!(cacher.value(25), 21);
}



trait Bird {
    fn quack(&self) -> String;
}

struct Duck;
impl Duck {
    fn swim(&self) {
        println!("Look, the duck is swimming")
    }
}


struct Swan;
impl Swan {
    fn fly(&self) {
        println!("Look, the duck.. oh sorry, the swan is flying")
    }
}

impl Bird for Duck {
    fn quack(&self) -> String{
        "duck duck".to_string()
    }
}

impl Bird for Swan {
    fn quack(&self) -> String{
        "swan swan".to_string()
    }
}

fn hatch_a_bird(t: i32) -> Box<dyn Bird> {
    if t == 2 {
        println!("{}", format!("amazing!!!")); // test
        Box::new(Duck{})
    } else {
        Box::new(Swan{})
    }
}


trait Draw {
    fn draw(&self) -> String;
}

impl Draw for u8 {
    fn draw(&self) -> String {
        format!("u8: {}", *self)
    }
}

impl Draw for f64 {
    fn draw(&self) -> String {
        format!("f64: {}", *self)
    }
}


// 包裹
fn draw_with_box(x: Box<dyn Draw>) {
    x.draw();
}
// 不包裹
fn draw_with_ref(x: &dyn Draw) {
    x.draw();
}


trait Foo {
    fn method(&self) -> String;
}

impl Foo for u8 {
    fn method(&self) -> String { format!("u8: {}", *self) }
}

impl Foo for String {
    fn method(&self) -> String { format!("string: {}", *self) }
}

// 特征参数 静态
fn static_dispatch(s: impl Foo) -> String {
    s.method()
}

// 通过泛型实现以下函数
fn static_dispatch_2<T: Foo>(x: T) {
    x.method();
}

// 通过特征对象实现以下函数
fn dynamic_dispatch(d: &dyn Foo) -> String {
    d.method()
}

// 特征对象的安全
trait MyTrait {
    fn f(&self) -> Self;
    // fn f(&self) -> Box<dyn MyTrait>
}

impl MyTrait for u32 {
    fn f(&self) -> Self { 42 } // 不能成为特征对象 返回值有Self change to  u32 { 42 }
    // fn f(&self) -> Box<dyn MyTrait> { Box::new(42) }
}

impl MyTrait for String {
    fn f(&self) -> Self { self.clone() } //不能成为特征对象 返回值有Self change to String {self.clone }
    // fn f(&self) -> Box<dyn MyTrait> { Box::new(self.clone) }
}

fn my_function(x: impl MyTrait) -> impl MyTrait {
//fn my_function(x: Box<dyn MyTrait>) -> Box<dyn MyTrait> {
    x.f()
}

// my_function(Box::new(13_u32));
// my_function(Box::new(String::from("abc")));
//
// println!("Success!")
