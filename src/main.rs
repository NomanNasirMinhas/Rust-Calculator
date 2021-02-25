use std::io;
fn main() {
    println!("Welcome to Rust Calculator\n------------------------\n");


    let mut option:u8 = 99;
    let mut res:f32 = 0.0;
    let mut num1:f32 = 0.0;
    let mut num2:f32 = 0.0;
    while option !=0{
    let mut MainMenuInput = String::new();
    let mut in1 = String::new();
    let mut in2 = String::new();
    println!("Please Select an Operation to Perform\n---------");
    println!("1- Addition\n2-Subtraction\n3-Multiplication\n4-Division\n0-Exit\n---------------");
    io::stdin().read_line(&mut MainMenuInput).expect("Error in Reading the Option");
    // println!("You Selected Option {}", MainMenuInput);
    option = MainMenuInput.trim().parse().expect("Please Enter a Valid Number");
    // option = option;
    if option > 0 && option <=4
 {   println!("Please Enter Your First Number");
    io::stdin().read_line(&mut in1).expect("Please Enter A Valid Number");
    num1 = in1.trim().parse().expect("Invalid Number");

    println!("Please Enter Your Second Number");
    io::stdin().read_line(&mut in2).expect("Please Enter A Valid Number");
    num2 = in2.trim().parse().expect("Invalid Number");}

    match option {
        1=>{
            res = num1 + num2;
            println!("{} + {} = {}\n", num1, num2, res);
        }

        2=>{
            res = num1 - num2;
            println!("{} - {} = {}\n", num1, num2, res);
        }

        3 =>{
            res = num1 * num2;
            println!("{} x {} = {}\n", num1, num2, res);
        }

        4 =>{
            res = num1 / num2;
            println!("{} / {} = {}\n", num1, num2, res);
        }
        0=>{
            println!("Exiting Calculator...\n");
        }

        _ =>{
            println!("Invalid Option Selected");
        }
    }
}


}
