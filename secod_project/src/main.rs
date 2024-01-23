use std::io;
fn add(x:u32,y:u32) -> u32{
return x+y;
}
fn sub(x:u32,y:u32)-> u32{
    return x-y;
}
fn mult(x:u32,y:u32)->u32{
    return x*y;
}
fn div(x:u32,y:u32)->u32{
    return x/y;
}

fn main() {
    let mut option : String = String::new();
    println!("What would you like to enter? ");
    io::stdin().read_line(&mut option);
   let myOption =option.to_lowercase().trim().to_string();
   
    if myOption == "add"
    {

    }
    else if myOption == "subtract"
    {

    } else if myOption == "multiply"
    {

    }else if myOption == "divide"
    {

    } else {
        println!("you do not enter one of the options");
    }
}
