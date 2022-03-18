// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:





struct GroceryItem {
    // * Use a struct for the grocery item
    // * Use two i32 fields for the quantity and id number
    quantity: i32,
    id: i32,
}

fn display_quant(quant: &GroceryItem) {
    // * Create a function to display the quantity, with the struct as a parameter
    println!("Quantity: {}", quant.quantity)
}

fn display_id(id: &GroceryItem) {
    // * Create a function to display the id number, with the struct as a parameter
    println!("Id: {}", id.id)
}

fn main() {
    let grocery = GroceryItem {
        quantity: 10,
        id: 1,
    };
    display_quant(&grocery);
    display_id(&grocery);
}
