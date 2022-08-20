//display current date and time
// use external crate 'crono'

use chrono::prelude::*;

fn main(){
  
let utc: DateTime<Utc> = Utc::now();       // e.g. `2014-11-28T12:45:59.324310806Z`
let local: DateTime<Local> = Local::now();

  println!("{:?}", utc.format("%a %b %e %T %Y").to_string());
  println!("{:?}", local);

  
}