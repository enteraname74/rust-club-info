//////////////
// Vecteurs // 
////////////// 

fn main() {
    let nombres_vide: Vec<i32> = Vec::new();

    let mut nombres = vec![1, 2, 3];

    // On ajoute un nombre :
    nombres.push(4);

    // On accède à un élément :
    let troisieme: &i32 = &nombres[2];

    println!("Le troisième élément est : {}.", troisieme);

    // Une meilleur approche, plus sûre :
    let troisieme: Option<&i32> = nombres.get(2);
    match troisieme {
        Some(troisieme) => println!("Le troisième élément est : {}.", troisieme),
        None => println!("Il n'y a pas de troisième élément."),
    }

    //////////////////////////////
    // Itération sur un vecteur //
    //////////////////////////////
    
    // Sans changer les éléments :

    // println!("Les éléments sont :");
    // for nombre in &nombres {
    //     println!("{}", nombre);
    // }

    // // En changeant les éléments :

    // println!("Les nouveaux éléments sont :");
    // for nombre in &mut nombres {
    //     *nombre += 50;
    //     println!("{}", nombre);
    // }

    /////////////////////////////////////////////////
    // Stocker n'importe quel type dans un vecteur //
    /////////////////////////////////////////////////
    
    // enum Element {
    //     Entier(i32),
    //     Flotant(f64),
    //     Texte(String),
    // }

    // let liste = vec![
    //     Element::Entier(3),
    //     Element::Texte(String::from("Rust <3")),
    //     Element::Flotant(10.12),
    // ];
// }

///////////////////////////
// Chaînes de caractères // 
///////////////////////////

// fn main() {
//     // Concaténation :

//     let mut s = String::from("foo");

//     s.push_str("bar");

//     println!("{}", s);

//     // ou 
    
//     let s1 = String::from("Hello, ");
//     let s2 = String::from("world!");
//     let s3 = s1 + &s2;

//     println!("{}", s3);

//     // ou
    
//     let s1 = String::from("tic");
//     let s2 = String::from("tac");
//     let s3 = String::from("toe");

//     let s = format!("{s1}-{s2}-{s3}");

//     println!("{}", s);

//     // Ajout d'un caractère : 

//     let mut s = String::from("lo");

//     s.push('l');

//     println!("{}", s);

//     ////////////////////////////////////////////
//     // Itération sur une chaîne de caractères //
//     ////////////////////////////////////////////

//     for c in "Hello".chars() {
//         println!("{}", c);
//     }

//     // Pour accèder à un élément :

//     // /!\ Impossible, Rust ne le permet pas. /!\
//     // Le type chaîne de caractères est plus compliqué qu'il n'en a l'air.
    
//     // Un moyen de quand même le faire pourrait être le suivant :

//     // Il s'agit d'une option, qui peut donc être vide.
//     // Si on souhaite l'utiliser, il faudra traiter l'option.
//     let ch = "Hello".chars().nth(2);

//     println!("Le caractère est : {:?}", ch);
// }