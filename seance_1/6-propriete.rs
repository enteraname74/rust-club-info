// Ce code ne compile pas, c'est normal.

fn calculate_length(s: String) -> usize {
    s.len()
}

fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(s1);
    
    println!("The length of s1 is {}.", len);
    println!("S1: {}", s1);
}

// La version corrigé :
//
// fn calculate_length(s: &String) -> usize { // On pourrait aussi mettre &str (c'est même mieux).
//     s.len()
// }

// fn main() {
//     let s1 = String::from("hello");

//     let len = calculate_length(&s1);

//     println!("The length of s1 is {}.", len);
//     println!("S1: {}", s1);
// }