// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

// * Use a struct to encapsulate the box characteristics
enum Color {
    Brown,
    Red,
}

impl Color {
    fn print(&self) {
        match self {
            Color::Brown => println!("Brown"),
            Color::Red => println!("Red"),
        }
    }
}

struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}

impl Dimensions {
    fn print(&self) {
        println!("width: {}, height: {}, depth: {}", self.width, self.height, self.depth);
    }
}

struct ShippingBox {
    color: Color,
    weight: f64,
    dimensions: Dimensions,
}

impl ShippingBox {
    fn new(color: Color, weight: f64, dimensions: Dimensions) -> Self {
        Self {
            weight,
            color,
            dimensions,
        }
    }
    fn print(&self) {
        self.color.print();
        self.dimensions.print();
        println!("Weight: {:?}", self.weight);
    }
}


fn main() {
    let small_dimensions = Dimensions {
        width: 1.0,
        height: 1.0,
        depth: 1.0,
    };

    let small_box = ShippingBox::new(Color::Red, 1.0, small_dimensions);
    small_box.print();
}
