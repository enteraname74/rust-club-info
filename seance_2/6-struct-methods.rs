// On définit les informations de la structure...
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// ...puis on implémente des méthodes pour cette dernière.
impl Rectangle {
    // Pour utiliser la méthode sur une instance de Rectangle, il faut faire passer l'instance en question comme premier paramètre.
    // Notez qu'on n'a pas besoin de préciser l'instance en paramètre quand on utilise la méthode.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Fonction associative.
    // Sans préciser &self, on peut utiliser cette fonction comme une fonction statique de Rectangle.
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // Appel de la méthode area() de notre instance rect1 de Rectangle.
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square = Rectangle::square(40);
    println!("Square rectangle: {:?}", square);
}
