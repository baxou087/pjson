
mod data;

use crate::data::*;

fn main() {
    let data = Data::new(String::from("text_test"), TypeVal::text(String::from("val1")));
    println!("{}", data.to_string());
    let data1 = Data::new(String::from("float_test"), TypeVal::number(1.0));
    println!("{}", data1.to_string());
    let data2 = Data::new(String::from("true_test"), TypeVal::boolean(true));
    println!("{}", data2.to_string());
    let data3 = Data::new(String::from("false_test"), TypeVal::boolean(false));
    println!("{}", data3.to_string());
    let data4 = Data::new(String::from("null"), TypeVal::null);
    println!("{}", data4.to_string());

    // let num = "1.0".parse::<f64>();
    // match num {
        // Ok(val) => println!("Yes, it was a number ({})", val),
        // Err(why) => println!("Doesn't look like a number ({})", why),
    // }
}
