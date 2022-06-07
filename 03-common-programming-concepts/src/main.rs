fn main() {
    // Variables

    let x = 5;
    println!("The value of x is: {}", x);
    let x = "six";
    println!("The value of x is: {}", x);

    const THIS_IS_A_CONSTANT: u32 = 10_000_000;
    println!("The value of THIS_IS_A_CONSTANT is: {}", THIS_IS_A_CONSTANT);

    // Data types

    // Scalar
    
    // Integer

    let x: u32 = 32;
    println!("The value of x is: {}", x);

    let x: i32 = 0xFF;
    println!("The value of x is: {}", x);

    let x: u128 = 0o11;
    println!("The value of x is: {}", x);

    let x: i128 = 0b1000_0000;
    println!("The value of x is: {}", x);

    let x: u8 = b'!';
    println!("The value of x is: {}", x);

    // Float
    
    let x = 2.0;
    println!("The value of x is: {}", x);

    let x: f32 = 3.0;
    println!("The value of x is: {}", x);

    let x = 2 / 3;
    println!("The value of x is: {}", x);

    let x = 2.0 / 3.0;
    println!("The value of x is: {}", x);

    let x = 2 % 3;
    println!("The value of x is: {}", x);

    let x = 2.0 % 3.0;
    println!("The value of x is: {}", x);

    let x = 2.5 % 3.0;
    println!("The value of x is: {}", x);

    // Boolean

    let x = true;
    println!("The value of x is: {}", x);

    let x = false;
    println!("The value of x is: {}", x);

    // Character

    let x = '0';
    println!("The value of x is: {}", x);

    let x = '😀';
    println!("The value of x is: {}", x);

    // Compound types

    let x = (42, "wow");
    println!("The value of x is: {:?}", x);
    println!("The value of x.0 is: {}", x.0);
    println!("The value of x.1 is: {}", x.1);

    let (num, message) = x;
    println!("The value of num is: {}", num);
    println!("The value of message is: {}", message);

    let x = [1, 2, 3];
    println!("The value of x is: {:?}", x);
    println!("The value of x[0] is: {:?}", x[0]);
    println!("The value of x[1] is: {:?}", x[1]);
    println!("The value of x[2] is: {:?}", x[2]);

    let x = [0; 8];
    println!("The value of x is: {:?}", x);

    // Functions
    let x = f;
    println!("The value of x() is: {:?}", x());

    let x = || 42;
    println!("The value of x() is: {:?}", x());

    let x = double;
    println!("The value of x(4) is: {:?}", x(4));

    if x(2) < 10 {
        println!("x(2) < 10");
    } else {
        println!("else branch");
    }

    let x = if x(4) > 42 { -1 } else { 1 };
    println!("The value of x is: {}", x);

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter;
        }
    };

    println!("Result is: {}", result);

    let mut counter = 3;
    while counter != 0 {
        println!("Counter is: {}", counter);
        counter -= 1;
    }

    for element in [1, 2, 3].iter() {
        println!("Element: {}", element);
    }

    for element in 1..=3 {
        println!("Element: {}", element);
    }

    // This is a comment.
    /* This is also a comment. */
}

fn f() {

}

fn double(n: i128) -> i128 {
    n * 2
}
