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
        println!("e: {}", e);
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
}
