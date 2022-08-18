fn main() {
    let user_name = "Sam";
    let id = find_user(user_name).map(|id| User {
        id,
        name: user_name.to_owned(),
    });

    match id {
        Some(val) => println!("The Id is : {:?}", val),
        None => println!("No user found"),
    }
}

#[derive(Debug)]
#[allow(dead_code)]

struct User {
    id: i32,
    name: String,
}

fn find_user(name: &str) -> Option<i32> {
    let name = name.to_lowercase();
    match name.as_str() {
        "sam" => Some(1),
        "anju" => Some(2),
        "meera" => Some(3),
        _ => None,
    }
}
