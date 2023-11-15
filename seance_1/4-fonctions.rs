fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

// Une autre manière d'écrire plus "classique" :
//
// fn plus_one(x: i32) -> i32 {
//     return x + 1;
// }