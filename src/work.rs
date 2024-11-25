 
pub fn greeting() {
    println!("Greetings earthling!");

    let numbers: [i32; 5] = [1,2,3,4,5];
    println!("{:?}", numbers);

    //tuples
    let tups: (i32, bool, char) = (10, true, 'A');
    let tups2: (i64, bool, &str) = (20, false, "hello");
    let tups3: (i64, bool, &str) = (20, false, "King");

    // Print elements of tups
    println!("tups: ({}, {}, '{}')", tups.0, tups.1, tups.2);
    // Print elements of tups2
    println!("tups2: ({}, {}, \"{}\")", tups2.0, tups2.1, tups2.2);
    println!("tups3: {:?}", tups3);
    //String
    let letter = String::from("i am a letter");
    println!("{}", letter);

   

    let yello: i32 = 100;
    let thing2: i32 = 200 + yello;
    println!("{thing2}" )

}