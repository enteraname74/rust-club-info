fn main() {
    ///////////////////////
    /// Nombres entiers ///
    ///////////////////////
    let a = 12; // i32

    let b: i64 = -14; // i64 permet de stocker des nombres plus grands.

    let c: u32 = 16; // Entier non signé, ne peut pas être négatif.

    ////////////////////////
    /// Nombres décimaux ///
    ////////////////////////
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    //////////////////
    /// Opérations ///
    //////////////////

    // Addition
    let somme = 5 + 10;

    // Soustraction
    let difference = 95.5 - 4.3;

    // Multiplication
    let produit = 4 * 30;

    // Division
    let quotient = 56.7 / 32.2;

    // Modulo
    let modulo = 43 % 5;

    ////////////////
    /// Booléens ///
    ////////////////
    let t = true;

    let f: bool = false;

    //////////////////
    /// Caractères ///
    //////////////////
    let ch = 'z';
    let z: char = 'ℤ';
    let emoji_chat = '😻';

    /////////////////////////////
    /// Chaînes de caractères ///
    /////////////////////////////
    let phrase = "Il était une fois..."; // &str est immutable

    let autre_phrase: String =
        String::from("... et ils vécurent heureux et eurent beaucoup d'enfants.");

    ////////////////
    /// Tableaux ///
    ////////////////
    let tableau: [i32; 5] = [0, 1, 2, 3, 4];

    let vecteur: Vec<i32> = vec![1, 2, 3];
}
