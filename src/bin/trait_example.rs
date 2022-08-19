//calculate perimeter of square and triangle
// square perimeter = length of any side * 4
//triangle = sum of 3 sides (a+b+c)

fn main(){
  let triangle = Triangle{
    side_a: 10,
    side_b : 10,
    side_c: 10
  };

  let square = Square{
    side : 10
  };

  calcs(triangle);
  calcs(square);
  
}

fn calcs(data: impl Perimeter){
  data.calculate_perimeter();
}



struct Square{
  side: i32
}

struct Triangle{
  side_a: i32,
  side_b : i32,
  side_c : i32
}

trait Perimeter{
  fn calculate_perimeter(&self);
}

impl Perimeter for Square{
  fn calculate_perimeter(&self) {
    println!("The peri of Square is : {:?} ", self.side*4);
}
}

impl Perimeter for Triangle{
  fn calculate_perimeter(&self){
    println!("The perimeter of Traingle is : {:?} ", self.side_a + self.side_b + self.side_c);
  }
}