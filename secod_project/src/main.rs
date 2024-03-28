use std::io;
struct num {
    num1:u32,
    num2:u32,
    getNum:String
}
impl num{
    fn CollectNum(&mut self)
    {
        self.getNum=String::new();

    }
    fn add(x:u32,y:u32)->u32
    {
x+y
    }
    fn subtract(x:u32,y:u32)->u32
    {
x-y
    }
    fn multi(x:u32,y:u32)->u32
    {
x*y
    }
    fn div(x:u32,y:u32) -> Option<u32>
    {

if y == 0x0{
    None
}else {
    Some(x/y)
}
    }

}

fn main() {
    let mut  option : String = String::new();//
    println!("What would you like to enter?
    1. add\n
    2. subtract\n
    3. multiply\n
    4. division\n
 ");
    io::stdin().read_line(&mut option).expect("Failed to read line");
    
    let resault:Result<u32, _> =option.parse();
    let parse_int = match resault{
        Ok(integer) =>
        {
            
            match(integer)
            {
                0x1 =>{
// this is foraddition
                }
                0x2=>{
//this is for  subtraction
                }
                0x3=>{
// this is for multiplication
                }
                0x4=>{
//this is for division
                }
                _ =>{
                    println!("Error you did not enter a valid option")
                
                }

            }
//going to prefore the calculator in this option
        }
        Err(e)=>
        {
            println!("error with parsing your choice");
            0;
        }
    };



}
