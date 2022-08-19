
fn main() {
  let sam = all_caps("sachin");
  println!("{sam}");
}

fn all_caps(data: &str) -> String {
    data.to_uppercase()
}


//test case can be run by command: "cargo test"
//test case should be put inside a mod with #[cfg(test)]
// test mod cannot red outside functions. So use use crate::*;
  //  #[test] denotes its a test function

#[cfg(test)]
mod test {
    use crate::*;
    #[test]
   fn test_all_caps() {
        let result = all_caps("billa");
        let expected = "BILLA".to_owned();
        assert_eq!(result, expected);
    }
}
