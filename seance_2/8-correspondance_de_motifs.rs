fn main() {
    let lancer_de_des = 9;

    match lancer_de_des {
        3 => println!("Victoire"),
        7 => println!("Neutre"),
        autre => println!("Défaite : {}", autre), // Nécessaire si on ne traite pas tous les cas.
    }
}

// Peut permettre de donner des valeurs aux éléments d'une énumération.

// #[derive(Debug)]
// enum EtatUs {
//     Alabama,
//     Alaska,
//     // ...
// }

// enum Piece {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(EtatUs),
// }

// fn main() {
//     let piece = Piece::Quarter(EtatUs::Alabama);

//     let valeur_piece = match piece {
//         Piece::Penny => 1,
//         Piece::Nickel => 5,
//         Piece::Dime => 10,
//         Piece::Quarter(etat) => {
//             println!("Pièce de l'État de {:?}", etat);
//             25
//         }
//     };

//     println!("La pièce vaut : {}", valeur_piece);
// }