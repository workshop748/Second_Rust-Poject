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
//going to prefore the calculator in this option
        }
        Err(e)=>
        {
            println!("error with parsing your choice");
            0;
        }
    }



}
