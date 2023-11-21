use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Rouge"), 10);
    scores.insert(String::from("Bleu"), 50);

    // Ajout seulement si la clef n'existe pas déjà :
    scores.entry(String::from("Jaune")).or_insert(50);

    let nom_equipe = String::from("Bleu");
    let score = scores.get(&nom_equipe).copied().unwrap_or(0); // Si l'équipe n'existe pas, le résultat sera 0.

    println!("Le score de l'équipe {} est de {}", nom_equipe, score);

    ///////////////
    // Itération //
    ///////////////

    for (clef, valeur) in &scores {
        println!("{}: {}", clef, valeur);
    }
}