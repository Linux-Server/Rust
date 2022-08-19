fn main(){
  hello(Dog{});
  hello(Cat{});
  
}

fn hello(commands: impl Noise){
  commands.make_noise();
}


struct Dog{}

struct Cat{}

trait Noise{
  fn make_noise(&self);
}

impl Noise for Dog{
  fn make_noise(&self) {
   println!("Dog is barking...") 
}
}

impl Noise for Cat{
  fn make_noise(&self) {
   println!("cat is barking...") 
}
}