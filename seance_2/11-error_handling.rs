use std::fs::File;
use std::io::Error;
use std::io::ErrorKind;

fn main() {
    // Le fichier n'existe potentiellement pas...
    // L'ouverture d'un fichier inexistant peut donc entrainer une erreur !
    // Au lieu de faire planter le programme, on peut récupérer facilement l'erreur pour la traier avec Result.
    let greeting_file_result: Result<File, Error> = File::open("hello.txt");

    // Analysons le résultat
    // Si on a un fichier, on le stock, sinon, si on a une erreur, on arrête le programme avec un message d'erreur.
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}

// Même chose qu'en haut, mais en retournant un message spécifique au type d'erreur.
fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}

fn main() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}
