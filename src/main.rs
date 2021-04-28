use std::{io, usize};

#[allow(dead_code)]
fn gets() -> String {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}

#[allow(unused_macros)]
macro_rules! read_vec {
    ($ty:ty) => {{
        let line = gets();
        let mut it = line.split_whitespace();
        let mut v = Vec::new();
        while let Some(value) = it.next() {
            v.push(value.parse::<$ty>().unwrap());
        }
        v
    }};
}

#[allow(unused_macros)]
macro_rules! read {
    ( $ty:ty) => {
        gets().split_whitespace().next().unwrap().parse::<$ty>().unwrap()
    };
    ( $($ty:ty),* ) => {{
        let line = gets();
        let mut it = line.split_whitespace();
        ( $(it.next().unwrap().parse::<$ty>().unwrap(),)* )
    }};
    ( $( $ty:ty ),+ ,) => {
		read![ $( $ty ),* ]
    };
}
fn main() {
    for _ in 0..read!(usize) {
        let x = read!(u64);
        if x % 2050 == 0 {
            let s = (x / 2050).to_string();
            let v: Vec<u32> = s.chars().map(|c| c.to_digit(10).unwrap()).collect();
            let mut ans = 0;
            for x in v {
                ans += x;
            }
            println! {"{}", ans};
        } else {
            println!("-1");
        }
    }
}
