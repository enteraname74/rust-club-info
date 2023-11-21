// Sans préciser comment gérer la durée de vie d'un type &str,
// Impossible de stocker ce dernier dans une structure.
// Les infos dans une struct doivent être valides jusqu'à ce que l'instance de la struct disparait.
struct User {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: "someusername123",
        email: "someone@example.com",
        sign_in_count: 1,
    };
}
