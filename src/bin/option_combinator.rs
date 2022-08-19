fn main(){
  let sam = vec![1,2,3,4,5];
  println!("{:?}", sam);
  
  let mut billa: Vec<_> = sam.iter().map(|num| num+10).collect();
  println!("{:?}", billa);

  let less_than_four: Vec<_> = sam.iter().filter(|num| num>&&4).collect();
    println!("{:?}", less_than_four);

  let big = sam.iter().find(|num| num==&&2);
    println!("{:?}", big);

  //triple the value and keep only > 10

  let triple: Vec<_> =  sam.iter().map(|num| num*3).filter(|num| num> &10).collect();

  println!("{:?}", triple);



}