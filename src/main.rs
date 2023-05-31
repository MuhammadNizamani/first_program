fn add_num(num1: i32, num2: i32) -> i32 {
    let sum = num1 + num2;
    sum
}
fn main() {
    let result = add_num(3, 4);
    println!("The sum is: {}", result);
    for number in 1..=3{
        let pi:f32 = 3.14;
        let name: &str = "ishaque";
        println!("{}, My name is {}  {}", number,name, pi)

    }
    let numbers:[i32; 5]= [1,2,3,4,5];
    // numbers[2] = 32;
    let mut frusts: [String; 3] = ["apple".to_string(), "banana".to_string(), "orange".to_string()];
    for ft in frusts.iter(){
        println!("{}",ft);

    }
    
    
    println!("change string values apple to chary");
    frusts[0]= "chary".to_string();
    println!();
    for f in frusts.iter(){
        print!("{} ",f)
    }

    println!("{} on index 3",numbers[3]);

}

