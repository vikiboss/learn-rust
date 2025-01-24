use crate::lib::transpose::transpose;

mod lib {
    pub mod transpose;
}

fn main() {
    let mut target = 1232345;

    while target > 17 {
        target = target % 17
    }

    println!("The final value is: {}", target);

    for n in 1..=5 {
        println!("n: {}", n);
    }

    for e in [1, 2, 3] {
        println!("e: {e}");
    }

    let mut num = 0;

    let x = loop {
        num += 1;
        if num == 10 {
            break num * 2;
        }
    };

    println!("The value of x is: {}", x);

    let a = 10;
    println!("before: {a}");

    let str = format!("a is {}", a);
    println!("{}", str);

    {
        let a = "hello";
        println!("inner scope: {a}");

        let a = true;
        println!("shadowed in inner scope: {a}");

        let a = 10;
        println!("shadowed again in inner scope: {a}");
    }

    dbg!(a);

    println!("after: {a}");

    // unreachable!("This code should never be reached");

    // todo!("Implement the rest of the program");

    let res = collatz_length_refined(10);
    println!("Collatz length of 10: {}", res);

    let mut arr: [i32; 3] = [1, 2, 3];
    // let [first, second, third] = arr;
    arr[1] = 4;
    println!("array {:?}", arr);

    let t: (i32, f64, u8, bool) = (500, 6.4, 1, true);
    let (x, y, z, mut active) = t;
    active = active && false;
    println!(
        "Tuple: x: {}, y: {}, z: {}, active: {}, t.1: {}",
        x, y, z, active, t.1
    );

    for a in arr {
        for i in 0..a {
            assert_ne!(i, 4);
        }
    }

    let matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];

    fn print_matrix(matrix: [[i32; 3]; 3]) {
        for row in matrix.iter() {
            for &cell in row.iter() {
                print!("{}, ", cell);
            }
            println!();
        }
    }

    print_matrix(matrix);

    let matrix = [[101, 102, 103], [201, 202, 203], [301, 302, 303]];
    let transposed = transpose(matrix);
    println!("transposed: {:?}", transposed);
}

#[allow(dead_code)]
fn collatz_length(mut n: i32) -> i32 {
    let mut len = 1;
    loop {
        if n == 1 {
            break len;
        }
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        len += 1;
    }
}

fn collatz_length_refined(mut n: i64) -> u64 {
    let mut len = 1u64;
    while n != 1 {
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        len += 1;
    }
    len
}

#[test]
fn test_collatz_length() {
    assert_eq!(collatz_length_refined(11), 15);
}
