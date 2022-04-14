//make a calculator


fn main () {
//ask user for a number
    println!("Please enter a number");
//get the number
    let mut number = String::new();
    std::io::stdin().read_line(&mut number).expect("Failed to read line");
//convert the number to a number
    let number: i32 = number.trim().parse().expect("Please type a number!");
//ask user for a second number
    println!("Please enter a second number");
//get the second number
    let mut number2 = String::new();
    std::io::stdin().read_line(&mut number2).expect("Failed to read line");
//convert the number to a number
    let number2: i32 = number2.trim().parse().expect("Please type a number!");
    //ask user for an operation
    println!("Please enter an operation:");
    println!("add:1");
    println!("subtract:2");
    println!("multiply:3");
    println!("divide:4");
//get the operation
    let mut operation = String::new();
    std::io::stdin().read_line(&mut operation).expect("Failed to read line");
//convert the operation to a string
    let operation: String = operation.trim().parse().expect("Please type a number!");

    //if the operation is +
    if number2 == 0 && operation == "4" {
        println!("You gotta be carefull you could've created a black hole!");
    }else if operation == "1" {
        add(number, number2);
    } else if operation == "2" {
        subtract(number, number2);
    } else if operation == "3" {
        multiply(number, number2);
    } else if operation == "4" {
        divide(number, number2);
    } else {
        println!("Please enter a valid operation");
    }


    
}

fn add(number: i32, number2: i32) {
    let output = number + number2;
    if output == 69 {
        println!("{} + {} = {}", number, number2, output);
        println!("69... nice");
    } else if output == 420 {
        println!("{} + {} = {}", number, number2, output);
        println!("420... nice");
    } else {
        println!("{} + {} = {}", number, number2, output);
    }
}

fn subtract(number: i32, number2: i32) {
    let output = number - number2;
    if output == 69 {
        println!("{} - {} = {}", number, number2, output);
        println!("69... nice");
    } else if output == 420 {
        println!("{} - {} = {}", number, number2, output);
        println!("420... nice");
    } else {
        println!("{} - {} = {}", number, number2, output);
    }
}

fn multiply(number: i32, number2: i32) {
    let output = number * number2;
    if output == 69 {
        println!("{} * {} = {}", number, number2, output);
        println!("69... nice");
    } else if output == 420 {
        println!("{} * {} = {}", number, number2, output);
        println!("420... nice");
    } else {
        println!("{} * {} = {}", number, number2, output);
    }
}

fn divide(number: i32, number2: i32) {
    let output = number / number2;
    if output == 69 {
        println!("{} / {} = {}", number, number2, output);
        println!("69... nice");
    
    } else if output == 420 {
        println!("{} / {} = {}", number, number2, output);
        println!("420... nice");
    } else {
        println!("{} / {} = {}", number, number2, output);
    }
}
