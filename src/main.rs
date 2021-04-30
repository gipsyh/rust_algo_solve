use std::io;

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
    let n = read!(usize);
    let a = read_vec!(i64);
    let b = read_vec!(i64);
    let mut sum = 0 as i64;
    for i in 0..n {
        sum += a[i] * b[i];
    }
    let mut ans = sum;
    for i in 0..n {
        let mut tmp = sum;
        for j in 1..i.min(n - i - 1) + 1 {
            tmp = tmp - a[i - j] * b[i - j] - a[i + j] * b[i + j]
                + a[i - j] * b[i + j]
                + a[i + j] * b[i - j];
            ans = ans.max(tmp);
        }
        let mut tmp = sum;
        if i == n - 1 {
            break;
        }
        for j in 0..i.min(n - i - 2) + 1 {
            tmp = tmp - a[i - j] * b[i - j] - a[i + j + 1] * b[i + j + 1]
                + a[i - j] * b[i + j + 1]
                + a[i + j + 1] * b[i - j];
            ans = ans.max(tmp);
        }
    }
    println!("{}", ans);
}
