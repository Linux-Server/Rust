fn main(){
  println!("heyyy");
  let _list = vec![1,2,3,4,5,6,7,8,9,10];
  let _target = 5;
  let outs = search(&_list, &_target);
  println!("{:?} = {_target} = {:?}", outs, _list);
  
}

fn search(_list:&Vec<i32>, _target: &i32)->Option<i32>{
   for (index, &item) in _list.iter().enumerate(){
     println!("List contains: {item} and index: {index}");
     if item == *_target{
       return Some(item);
     }
   }
  None
  
  
}