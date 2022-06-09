use num::complex::Complex;


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

}
