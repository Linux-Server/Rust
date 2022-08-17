fn main() {
    println!("Welcome to Binary Search");
    let _list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let _target = 11;
    let outs = search(&_list, &_target);
     println!("Matching Item is :{:?} ", outs);
}

fn search(_list: &Vec<i32>, _target: &i32)-> Option<i32> {
   // 0(first) - - - 4- - - - 8(last)
    let mut first: i32 = 0;
    let mut last: i32 = _list.len() as i32 -1;
    let mut count = 0;
    while first <= last{
        count += 1;
        println!("Number of operations:{count} ");

        let mid_point: i32 = (last+first)/2;
        if mid_point == *_target{
          return  Some(mid_point);
        }else if mid_point< *_target{
           first =  mid_point+1;
        }else{
         last = mid_point - 1;
        }

    }
  None
      
}
