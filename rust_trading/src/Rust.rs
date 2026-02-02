use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    println!("Let's execute an order!");
    let mut buy_orders: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut sell_orders: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut order_fulfilled = false;
    const SELL_ACTION: &str = "SELL";
    const BUY_ACTION: &str = "BUY";

    while !order_fulfilled {
        let action = get_input("\n(0) BUY, (1) SELL, (2) EXIT: ");
        
        if action == 2 { break; }
        if action > 2 { println!("Invalid selection!"); continue; }

        let price = get_input("Enter price: ");
        let quantity = get_input("Enter quantity: ");

        // Determine which side is the "market" (to check) and which is the "book" (to store)
        // If I BUY (0), I search SELLS. If I SELL (1), I search BUYS.
        if action == 0 {
            order_fulfilled = match_or_post(price, quantity, &mut sell_orders, &mut buy_orders, SELL_ACTION);
        } else {
            order_fulfilled = match_or_post(price, quantity, &mut buy_orders, &mut sell_orders, BUY_ACTION);
        }
    }
}

/// Core Logic: Tries to match an order against a search_map. 
/// If no match, it adds it to the post_map.
fn match_or_post(
    price: u32, 
    quantity: u32, 
    search_orders: &mut HashMap<u32, Vec<u32>>, 
    post_orders: &mut HashMap<u32, Vec<u32>>,
    action_name: &str
) -> bool {
    // 1. Try to find a match
    if let Some(orders) = search_orders.get_mut(&price) {
        if let Some(index) = orders.iter().position(|&q| q == quantity) {
            println!("Corresponding {action_name} order found!");
            orders.remove(index);
            
            // Clean up empty price levels
            if orders.is_empty() {
                search_orders.remove(&price);
            }
            
            println!("Order for {quantity} at ${price} fulfilled!");
            return true; 
        }
        println!("Order at ${price} exists, but quantity does not match.");
    } else {
        println!("No {action_name} orders exist at this price.");
    }

    // 2. If no match found, post to our side of the book
    post_orders.entry(price).or_default().push(quantity);
    false
}

fn get_input(prompt: &str) -> u32 {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().unwrap_or(0)
}