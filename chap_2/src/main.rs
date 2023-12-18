
fn my_function(a: i32, b: i32) -> i32 {
    println!("Another function");
    println!("The value of a is: {}", a);
    println!("The value of b is: {}", b);
    let sum: i32 = a + b;

    // return sum;
    sum
}

fn main() {
    // VARIABLES
    let x = 5;
    println!("The value of x is: {}", x);

    // shadowing
    let x = "6";
    println!("The value of x is: {}", x);
    
    // const must be type annotated
    const HELO: u32 = 100_000;
    println!("The value of HELO is: {}", HELO);




    // DATA TYPES
    // Scalar datatypes 
    let _a: i32 = 34;
    let _f: u8 = 255; // 256 -> 0 | error
    let _fl: f32 = 3.0;
    let _b: bool = false;
    let _c: char = 'x';
    
    // Compound datatypes
    let tup: (&str, i32) = ("Let's go", 100);
    let (_first, _second) = tup;
    let _first = tup.1;

    let mut array: [i32; 5] = [1, 2, 3, 4, 5];
    let _g = array[2];
    array[1] = 25;

    let _byte = [0; 8];





    // FUNCTIONS
    let sum: i32 = my_function(11, 22); 
    println!("The sum is: {}", sum);





    // CONTROL FLOW
    let number: i32 = 5;
    
    if number < 10 {
        println!("first");
    } else if number < 30 && number > 15 {
        println!("second");
    } else {
        println!("third");
    }

    let condition: bool = true;
    let mut number = if condition { 5 } else { 69 };
    println!("{}", number);

    // Loops
    let mut counter = 0;
    let result = loop{
        counter += 1;
        if counter > 2 {
            break counter;
        }
    };
    println!("{}", result);


    while number != 0 {
        counter -= 1;
        number -= 1;
    }
    println!("{}", counter);


    let arrays = [10, 20, 30, 40, 50];
    for element in arrays.iter() {
        println!("The value is: {}", element);
    }

    for number in 1..4 {
        println!("{}!", number);
    }
}
