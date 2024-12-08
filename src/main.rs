// Import necessary modules
use std::collections::HashMap;
use std::io;

// Define a struct for an item
#[derive(Debug)]
struct Item {
    name: String,
    description: String,
    category: String,
    attribute: String,
}

// Implement methods for Item
impl Item {
    // Constructor method
    fn new(name: &str, description: &str, category: &str, attribute: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            category: category.to_string(),
            attribute: attribute.to_string(),
        }
    }

    // Method to display item details
    fn display(&self) -> String {
        format!(
            "Name: {}\nDescription: {}\nCategory: {}\nAttribute: {}",
            self.name, self.description, self.category, self.attribute
        )
    }
}

// Main function
fn main() {
    // A HashMap to store items (grouped by cat)
    let mut inventory: HashMap<String, Vec<Item>> = HashMap::new();

    // Menu loop
    loop {
        // Options
        println!("Choose an option: 1) Add 2) Remove 3) View All 4) Search 5) Exit");

        // Get choice
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => add_item(&mut inventory),
            "2" => remove_item(&mut inventory),
            "3" => view_all_items(&inventory),
            "4" => search_inventory(&inventory),
            "5" => break,
            _ => println!("Invalid choice."),
        }
    }
}

// Function to add item
fn add_item(inventory: &mut HashMap<String, Vec<Item>>) {
    println!("Enter item name:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();

    println!("Enter description:");
    let mut description = String::new();
    io::stdin().read_line(&mut description).unwrap();

    println!("Enter category (Worn Items, Held Items, Consumables):");
    let mut category = String::new();
    io::stdin().read_line(&mut category).unwrap();
    let category = category.trim().to_lowercase();

    println!("Enter attribute (e.g., Defense +5, Damage +10):");
    let mut attribute = String::new();
    io::stdin().read_line(&mut attribute).unwrap();

    // Create a new item and add it to the appropriate category
    let item = Item::new(name.trim(), description.trim(), &category, attribute.trim());
    inventory.entry(category).or_insert_with(Vec::new).push(item);

    println!("Item added!");
}

// Function to remove item
fn remove_item(inventory: &mut HashMap<String, Vec<Item>>) {
    println!("Enter category to remove from:");
    let mut category = String::new();
    io::stdin().read_line(&mut category).unwrap();
    let category = category.trim().to_lowercase();

    if let Some(items) = inventory.get_mut(&category) {
        println!("Enter item name to remove:");
        let mut name = String::new();
        io::stdin().read_line(&mut name).unwrap();

        if let Some(pos) = items.iter().position(|item| item.name.to_lowercase() == name.trim().to_lowercase()) {
            items.remove(pos);
            println!("Item removed!");
        } else {
            println!("Item not found.");
        }
    } else {
        println!("Category not found.");
    }
}

// Function to view all items
fn view_all_items(inventory: &HashMap<String, Vec<Item>>) {
    if inventory.is_empty() {
        println!("Inventory is empty.");
        return;
    }

    for (category, items) in inventory {
        println!("\nCategory: {}", category);
        for item in items {
            println!("{}", item.display());
        }
    }
}

// Function to search for an item
fn search_inventory(inventory: &HashMap<String, Vec<Item>>) {
    println!("Enter category to search in:");
    let mut category = String::new();
    io::stdin().read_line(&mut category).unwrap();
    let category = category.trim().to_lowercase();

    if let Some(items) = inventory.get(&category) {
        println!("Enter item name to search for:");
        let mut name = String::new();
        io::stdin().read_line(&mut name).unwrap();

        if let Some(item) = items.iter().find(|item| item.name.to_lowercase() == name.trim().to_lowercase()) {
            println!("Item found:\n{}", item.display());
        } else {
            println!("Item not found.");
        }
    } else {
        println!("Category not found.");
    }
}