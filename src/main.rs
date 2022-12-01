#![allow(unused)]

mod restaurant;

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::ops::Add;
use std::collections::HashMap;
use crate::restaurant::order_food;

fn main() {
    order_food()

}

fn structs_and_traits(){
    const PI: f32 = 3.141592;
    struct Customer {
        name: String,
        address: String,
        balance: f32,
    }

    let mut bob = Customer {
        name: String::from("Bob Smith"),
        address: String::from("555 Main St"),
        balance: 234.50,
    };

    bob.address = String::from("505 Main St");


    struct Rectangle<T, U> {
        length: T,
        height: U,
    }

    let rec = Rectangle {
        length: 5,
        height: 5.5,
    };
    trait Shape {
        fn new(length: f32, width: f32) -> Self;
        fn area(&self) -> f32;
    }
    struct Rectangle2 {
        length: f32,
        width: f32,
    }

    struct Circle {
        length: f32,
        width: f32,
    }

    impl Shape for Rectangle2{
        fn new(length: f32, width: f32) -> Rectangle2 {
            return Rectangle2{length,width};
        }
        fn area(&self) -> f32 {
            return self.width * self.length;
        }
    }

    impl Shape for Circle{
        fn new(length: f32, width: f32) -> Circle {
            return Circle{length,width};
        }
        fn area(&self) -> f32 {
            return (self.width / 2.0).powf(2.0) * PI;
        }
    }

    let rec: Rectangle2 = Shape::new(10.0, 10.0);
    let circle: Circle = Shape::new(10.0, 10.0);
    println!("area of rectangle:  {}, area of circle: {}", rec.area(), circle.area())
}

fn hashmpas() {
    let mut heroes = HashMap::new();
    heroes.insert("Superman", "Clark Kent");
    heroes.insert("Batman", "Bruce Wayne");
    heroes.insert("The Flash", "Barry Allen");

    for (k, v) in heroes.iter() {
        println!("{} = {}", k, v)
    }

    if heroes.contains_key("Batman") {
        let the_batman = heroes.get("Batman");
        match the_batman {
            Some(x) => println!("that man is a hero"),
            None => println!("Batman is not a hero")
        };
    }
}

fn ownership() {
    let mut str1 = String::from("tuur");
    //let str2 = str1; //str1 now becomes str2 so the println will fail
    let str2 = str1.clone();//now there are two copies of str1 one that's consumed by str2 and one that remains str1
    println!("Hello {}", str1);
    change_string(&mut str1);
    println!("{}", str1)
}

fn change_string(x: &mut String) {
    x.push_str(" is happy");
}

//    println!("4 + 5 = {}", get_sum_gen(1.2, 2.2));
fn get_sum_gen<T: Add<Output=T>>(x: T, y: T) -> T {
    return x + y;
}

//let (val_1, val2) = get_2(2);
fn get_2(x: i32) -> (i32, i32) {
    return (x + 1, x + 2);
}

fn vectors() {
    let vec1: Vec<i32> = Vec::new();
    let mut vec2 = vec![1, 2, 3, 4];
    vec2.push(5);
    println!("{}", vec2[0]);
    let second: &i32 = &vec2[1];
    match vec2.get(1) {
        Some(second) => println!("2nd: {}", second),
        none => println!("no second value")
    }

    for i in &mut vec2 {
        *i *= 2;
    }
    for i in &vec2 {
        println!("{}", i);
    }
}

fn enums() {
    enum DAYS {
        Monday,
        Thuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,

    }

    impl DAYS {
        fn is_weekend(&self) -> bool {
            match self {
                DAYS::Saturday | DAYS::Sunday => true,
                _ => false
            }
        }
    }

    let today: DAYS = DAYS::Monday;


    match today {
        DAYS::Monday => println!("everyone hates monday"),
        _ => println!("nie maandag")
    }

    println!("it is weekend: {}", today.is_weekend())
}

fn casting_as() {
    let int_u8: u8 = 5;
    let int2_u8: u8 = 4;
    let int3_u32: u32 = (int_u8 as u32) + (int2_u8 as u32);
}

fn basic_strings() {
    let mut st1 = String::new(); //new empty string
    st1.push('a'); //add character
    st1.push_str(" word"); //add str
    for word in st1.split_whitespace() {
        println!("{}", word)
    }

    let st2 = st1.replace("a", "Another");
    println!("{}", st2);

    let st3 = String::from("a z z z f g h");//create string with value

    let mut v1: Vec<char> = st3.chars().collect();
    v1.sort();
    v1.dedup();
    for char in v1 {
        println!("{}", char)
    }

    let st4: &str = "random string";
    let mut st5: String = st4.to_string();
    println!("{}", st5);

    let byte_arr1 = st5.as_bytes();
    let st6 = &st5[0..6];
    println!("string length: {}", st6.len());
    st5.clear();
    println!("{}", st5);
    let st6 = String::from("Just some");
    let st7 = String::from("words");
    let st8 = st6 + &st7; //st6 bestaat nu niet meer omdat hij deel uit maakt van st8 st7 wel omdat we een refferentie naar st7 in st8 steken

    for char in st8.bytes() {
        println!("{}", char);
    }
}

fn tuples() {
    let my_tuple: (u8, String, f64) = (47, "tuur".to_string(), 50000.00);
    println!("name: {}", my_tuple.1)
}

fn for_each() {
    let arr_1 = [1, 2, 3, 4, 5, 6, 7, 8, 9];

    for val in arr_1.iter() {
        println!("val : {}", val)
    }
}

fn while_loop() {
    let arr_1 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut loop_index = 0;

    while loop_index < arr_1.len() {
        println!("ARR : {}", arr_1[loop_index]);
        loop_index += 1;
    }
}

fn odd_loop() {
    let arr_1 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("first: {}", arr_1[0]);
    println!("legnth of array: {}", arr_1.len());
    let mut loop_index = 0;
    loop {
        if arr_1[loop_index] % 2 == 0 {
            loop_index += 1;
            continue;
        }
        if arr_1[loop_index] == 9 {
            break;
        }
        println!("val : {}", arr_1[loop_index]);
        loop_index += 1;
    }
}

fn match_cmp() {
    const MY_AGE: i32 = 18;
    const VOTING_AGE: i32 = 18;
    match MY_AGE.cmp(&VOTING_AGE) {
        Ordering::Less => { println!("can't vote") }
        Ordering::Equal => { println!("can vote for the first time") }
        Ordering::Greater => { println!("can vote") }
    }
}

fn match_age() {
    let age2 = 8;
    match age2 {
        1..=18 => println!("important birthday"),
        21 | 50 => println!("important birthday"),
        65..=u32::MAX => println!("important birthday"),
        _ => println!("not important")
    }
}

fn conditionals() {
    //if else
    let age = 8;
    if (age >= 1) && (age <= 18) {
        println!("important birthday");
    } else if (age == 21) || (age == 50) {
        println!("important birthday");
    } else if age >= 65 {
        println!("important birthday");
    } else {
        println!("not important")
    }

    //ternary
    let mut my_age = 47;
    let can_vote = if my_age >= 18 {
        true
    } else {
        false
    };
}

fn gen_random(min: i32, max: i32) -> i32 {
    return rand::thread_rng().gen_range(min..max + 1);
}

fn hello_world() {
    println!("what is your name?");
    let mut name = String::new();
    let greeting = "Nice to meet you";
    io::stdin().read_line(&mut name)
        .expect("Didn't receive input");
    println!("Hello {} {}", name.trim_end(), greeting);
}

fn mutables() {
    const PI: f32 = 3.14593;
    let age = "47";
    let mut age: u32 = age.trim().parse()
        .expect("Age wasn't asigned a number");
    age = age + 1;
    println!("I'm {}", age)
}