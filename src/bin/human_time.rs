use std::time::Duration;
use humantime::format_duration;

//here we are using external crate

fn main(){
let val1 = Duration::new(100, 0);
println!("The change is : {:?}", format_duration(val1).to_string());
// assert_eq!(format_duration(val1).to_string(), "2h 37m");
// let val2 = Duration::new(0, 32_000_000);
// assert_eq!(format_duration(val2).to_string(), "32ms");
  
}
