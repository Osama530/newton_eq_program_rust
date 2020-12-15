use std::io;

fn map(value: u16, lower_bound: u16, upper_bound: u16, lower_constrain: u16, upper_constrain: u16)-> u16 {
    let diff_bound = upper_bound - lower_bound;
    let diff_constrain = upper_constrain - lower_constrain;

    let result = (diff_constrain as f32 / diff_bound as f32) * value as f32;
    result as u16
}
fn main() {

    // menu();
    // let choice = get_input_u32();
    // println!("your selection is {}", choice);

    // let result = match choice {
    //     1 => first_equation(),
    //     2 => second_equation(),
    //     3 => third_equation(),
    //     _ => undefine(),
    // };

    // println!("result = {}",result);
    let upper_bound = 4096;
    let upper_constrain = 8;
    let lower_bound = 0;
    let lower_constrain =0;


    let diff_bound = upper_bound - lower_bound;
    println!("diff bound = {}", diff_bound);
    let diff_constrain = upper_constrain - lower_constrain;
    println!("diff constrain = {}", diff_constrain);

    let test_value = 2055;
    
    let result: f32 = (diff_constrain as f32 / diff_bound as f32)* test_value as f32 ;
    println!("result = {}", result as u16);

    let test_result = map(test_value, 0, 4096, 0, 8);
    println!("test result = {}", test_result);  


}

fn menu (){

    println!("******Selecte fron the list od equations*******");
    println!("");
    println!("1. S = vt");
    println!("2. S = vit + 1/2at^2 ");
    println!("3. 2aS = vf^2 - vi^2");
    println!("");
}

fn get_input_u32()-> u32{
    let mut num = String::new();
	io::stdin().read_line(&mut num);
    let num:u32 = num.trim().parse().unwrap();
    num
}

fn get_input_f32()->f32{
    let mut num = String::new();
	io::stdin().read_line(&mut num);
    let num:f32 = num.trim().parse().unwrap();
    num
}

fn first_equation()->String{
    println!("enter the velocity in m/s");
    let velocity = get_input_f32();
    println!("enter the time in sec");
    let time = get_input_f32();

    let distance = velocity*time;
    distance.to_string()
}

fn second_equation()->String{
    println!("enter initial velocity in m/s");
    let velocity = get_input_f32();
    println!("enter the time in sec");
    let time = get_input_f32();
    println!("enter the acceleration in m/s^2");
    let acc = get_input_f32();

    let distance = velocity*time + 0.5*acc*(time*time);
    distance.to_string()
}

fn third_equation()->String{
    println!("enter final velocity in m/s");
    let velocity_f = get_input_f32();
    println!("enter inital velocity in m/s");
    let velocity_i = get_input_f32();

    let result = (velocity_f*velocity_f) - (velocity_i*velocity_i);
    result.to_string()
}

fn undefine ()->String {
    "undefine argument please selecte fron the given options".to_string()
}