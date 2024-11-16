mod work;

fn main() {
  work::greeting();


    println!("Hello, world!");
    let  x: i32 = 5;
    println!("x is = {}", x);
    let y: i32 = 10;
    println!("x + y is = {}", x + y);

//CHAR and &str are different
    //str accepts only letters
    const NUMBER: &str = "i am the number:";
    println!("{}",  NUMBER);
    //char accepts anything if used with single quotation mark 
    let character: char = 'A';
    println!("{}", character);

    // for numbers we use i32 or u32 or float
    const ACTUAL_NUMBER: i32 = 20;
    println!("Number:{}", ACTUAL_NUMBER);
    println!("{} {}", NUMBER, ACTUAL_NUMBER);

  //tuples
    let tups: (i32, bool, char) = (10, true, 'A');
    let tups2: (i64, bool, &str) = (20, false, "hello");

    // Print elements of tups
    println!("tups: ({}, {}, '{}')", tups.0, tups.1, tups.2);
    // Print elements of tups2
    println!("tups2: ({}, {}, \"{}\")", tups2.0, tups2.1, tups2.2);

  

}
