mod work;

fn main() {
  work::greeting();


    println!("Hello, world!");
    let  x: i32 = 5;
    println!("x is = {}", x);
    let y: i32 = 10;
    println!("x + y is = {}", x + y);

//CHAR and &str are different
    //str accepts letters and is unmutable 
    const NUMBER: &str = "i am the number:";
    println!("{}",  NUMBER);
    
    //char accepts anything if used with single element quotation mark 
    let character: char = 'A';
    println!("{}", character);

    // for numbers we use i32 or u32 or float
    const ACTUAL_NUMBER: i32 = 20;
    println!("Number:{}", ACTUAL_NUMBER);
    println!("{} {}", NUMBER, ACTUAL_NUMBER);

}
