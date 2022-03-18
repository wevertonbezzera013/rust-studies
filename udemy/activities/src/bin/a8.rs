// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavor {
    Orange,
    Mango,
    Strawberry
}

struct Drink {
    flavor: Flavor,
    fluid_oz: f64,
}

fn print_drink(drink: Drink) {
    match drink.flavor {
        Flavor::Orange => println!("Flavor: Orange"),
        Flavor::Mango => println!("Flavor: Mango"),
        Flavor::Strawberry => println!("Flavor: Strawberry"),
    }
    println!("oz: {:?}", drink.fluid_oz);

}

fn main() {
    let mango = Drink{
        flavor: Flavor::Orange,
        fluid_oz: 0.1,
    };
    print_drink(mango);
}
