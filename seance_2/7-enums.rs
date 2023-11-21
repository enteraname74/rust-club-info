enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

impl IpAddr {
    fn afficher(&self) {
        println!("{}", self.address)
    }
}

fn main() {
    let ip_version = IpAddrKind::V4;

    let localhost = IpAddr {
        kind: ip_version,
        address: String::from("127.0.0.1"),
    };

    localhost.afficher();
}

// Le code ci-dessous est équivalent à celui ci-dessus : 

// enum IpAddr {
//     V4(String),
//     V6(String),
// }

// impl IpAddr {
//     fn afficher(&self) {
//         match self {
//             IpAddr::V4(addr) => println!("{}", addr),
//             IpAddr::V6(addr) => println!("{}", addr),
//         }
//     }
// }

// fn main() {
//     let localhost = IpAddr::V4(String::from("127.0.0.1"));

//     localhost.afficher();
// }

////////////
// Option //
////////////

// Soit quelque chose, soit rien.
// enum Option<T> {
//     None,
//     Some(T),
// }

// fn main() {
//     let nombre = Some(5);
//     let caractere = Some('e');

//     let nombre_absent: Option<i32> = None;

//     let x = 2;

//     // La ligne suivante empêche la compilation.
//     // Il faut s'assurer que l'option contienne bel et bien un nombre.
//     let somme = x + nombre;

//     // Solution :
//     // Dans le cas où le nombre a une valeur, on l'ajoute à x, sinon la somme vaut x + 0, soit x.
//     let somme = if let Some(valeur) = nombre { x + valeur } else { x };

//     // Autre solution :
//     // Même principe.
//     let somme = match nombre {
//         Some(valeur) => x + valeur,
//         None => x,
//     };

//     println!("{}", somme);
// }