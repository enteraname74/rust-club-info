fn main() {
    let potentialNumber: Option<u32> = getPotentialNumber();

    match potentialNumber {
        Some(number) => println!("Has number : {}", number),
        None => println!("No number found!"),
    }
}

/// Fonction qui peut retourner un nombre ou rien
fn getPotentialNumber() -> Option<u32> {
    return None;
}
