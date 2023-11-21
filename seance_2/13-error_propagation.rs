use std::fs;
use std::fs::File;
use std::io::{self, Read};

fn main() {
    let result = read_username_from_file();

    match result {
        Ok(username) => println!("Found username : {}", username),
        Err(error) => println!("Found error : {}", error),
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    // On essaie d'ouvrir un fichier.
    let username_file_result = File::open("hello.txt");

    // Si le fichier n'est pas trouvé, on retourne une erreur.
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    // On analyse le fichier, si une erreur apparait, on la retourne.
    // Sinon, on retourne le username.
    // Notez l'usage du return sans le mot clé et sans ";".
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// Une autre façon de gérer les erreurs dans une fonction est d'utiliser le "?".
// Avec le "?", si une opération retourne une erreur, la fonction s'arrête et renvoie cette dernière (pas besoin de faire un match donc).
// Sinon, le programme continue et, au lieu de récupérer un Result, on récupère le côté Ok() de ce dernier.
// ATTENTION : le "?" ne peut être utilisé que si le type de retour de la fonction (ici, le côté Err du Result)
// correspond au type de retour de la potentiel erreur où on utilise le "?".
fn read_username_from_file_v2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// Poussons encore plus le concept précédent !
// On peut enchainer une nouvelle opération après un "?".
// On est assurer avec le "?" que dans le pire des cas, une erreur sera retournée par la fonction.
fn read_username_from_file_v3() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

// Une dernière version pour la route...
fn read_username_from_file_v4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
