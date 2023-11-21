use std::fs::File;
use std::io::Error;
use std::io::ErrorKind;

// unwrap
fn main() {
    // En utilisant unwrap, on essaie de récupérer le côté Ok() du Result.
    // Si une erreur est retourné, panic est appelé automatiquement et le programme s'arrête.
    let greeting_file = File::open("hello.txt").unwrap();
}

// expect
fn main() {
    // Réalise la même chose qu'un unwrap mais on précise le message d'erreur à afficher si panic est appelé.
    let greeting_file =
        File::open("hello.txt").expect("hello.txt should be included in this project");
}

// Utilisation plus poussée du unwrap.
fn main() {
    // On essaie de unwrap le résultat (de récupérer le fichier dans notre cas).
    // Sinon, on analyse l'erreur dans une callback.
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
