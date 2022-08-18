fn main(){

  // map is used to change the value from one to another
  //we can use chainned mapping
  //inside map a closure can be provided
  let data = [1,2,3];
  let sam = data.map(|num| num+1);
  println!("{:?}", sam);
}