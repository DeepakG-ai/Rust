// ============================================================================
// ## Q36 — Warehouse (revision of Q1–Q12: ownership, &mut, Option, Result, ?, enum)
//
// No signatures given on purpose. Design the functions yourself.
//
// Data
// - An item has a name, a quantity (whole number, never negative) and a price (decimal).
// - A stock level is one of three things: out of stock, low (carries the remaining
//   quantity; low means fewer than 5 units), or fine.
// - A warehouse owns a collection of items.
//
// Behaviour to build
// 1. Read one line. Turn a text like "laptop,3,999.5" into an item, or an error
//    message. The error must say which problem it was:
//      - wrong number of fields
//      - quantity is not a valid number (include the bad text)
//      - price is not a valid number (include the bad text)
//      - price below zero
// 2. Load many lines. Build a warehouse from a list of lines. It stops at the first
//    bad line. The error must include the line number (counting from 1) and the
//    underlying reason. Do not repeat the parsing logic here.
// 3. Remove stock. Take a name and qty and change the warehouse in place.
//    Fail with a message if the name doesn't exist, and fail with a message showing
//    available vs requested if there isn't enough stock. On success it says how many
//    are left.
// 4. Stock level. Given an item, work out its stock level as described above.
// 5. Cheapest item. Given the warehouse, return the cheapest item that is in stock,
//    or nothing. It must not copy the item, and the caller must be able to keep
//    using the warehouse after.
// 6. Report. Given the warehouse, produce one text line per item, such as
//    "laptop x3 (LOW: 3)". The caller still uses the warehouse afterwards.
// 7. Close out. Take the warehouse away from the caller completely and return its
//    total value (quantity x price, summed).
//
// Main must show
// - loading good data, then loading data where line 3 has a bad price
// - removing stock: success, unknown item, too much
// - cheapest in-stock item for a warehouse where the cheapest item is out of stock
// - the report
// - the close-out, then a comment holding the compiler error you get when you try
//   to use the warehouse afterwards (the real message, copied)
//
// Expected shape of output (numbers may differ with your data):
// Err("line 3: price below zero: -5")
// Ok(2 left)
// Err("no such item: phone")
// Err("not enough stock: have 2, want 10")
// cheapest in stock: mouse
// laptop x3 (LOW: 3)
// ...
// Total value: 3998.5
// ============================================================================
#![allow(dead_code)]

#[derive(Debug)]
enum StockLevel {
    OutOfStock,
    Low(u32),
    Fine,
}

impl std::fmt::Display for StockLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StockLevel::OutOfStock => write!(f, "OUT OF STOCK"),
            StockLevel::Low(qty) => write!(f, "LOW: {}", qty),
            StockLevel::Fine => write!(f, "FINE"),
        }
    }
}

#[derive(Debug)]
struct Item {
    name: String,
    quantity: u32,
    price: f64,
}

impl Item {
    fn new(name: &str, quantity: u32, price: f64) -> Self {
        Self {
            name: name.to_string(),
            quantity,
            price,
        }
    }

    // 1. Read one line. Turn a text like "laptop,3,999.5" into an item, or an error message.
    fn from_line(line: &str) -> Result<Item, String> {
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() != 3 {
            return Err("wrong number of fields".to_string());
        }

        let name = parts[0].trim().to_string();

        let quantity = parts[1]
            .trim()
            .parse::<u32>()
            .map_err(|_| format!("quantity is not a valid number: {}", parts[1].trim()))?;

        let price = parts[2]
            .trim()
            .parse::<f64>()
            .map_err(|_| format!("price is not a valid number: {}", parts[2].trim()))?;

        if price < 0.0 {
            return Err(format!("price below zero: {}", parts[2].trim()));
        }

        Ok(Item {
            name,
            quantity,
            price,
        })
    }

    // 4. Stock level. Given an item, work out its stock level.
    fn stock_level(&self) -> StockLevel {
        if self.quantity == 0 {
            StockLevel::OutOfStock
        } else if self.quantity < 5 {
            StockLevel::Low(self.quantity)
        } else {
            StockLevel::Fine
        }
    }
}

#[derive(Debug)]
struct Warehouse {
    items: Vec<Item>,
}

impl Warehouse {
    fn new(items: Vec<Item>) -> Self {
        Self { items }
    }

    // 2. Load many lines. Build a warehouse from a list of lines. Stops at the first bad line.
    fn from_lines(lines: &[&str]) -> Result<Warehouse, String> {
        let mut items = Vec::new();
        for (index, line) in lines.iter().enumerate() {
            let line_num = index + 1;
            let item =
                Item::from_line(line).map_err(|err| format!("line {}: {}", line_num, err))?;
            items.push(item);
        }
        Ok(Warehouse { items }) // warehouse.items = [Item { laptop, qty 3,  price 999.5 },Item { mouse,  qty 10, price 25.0  },Item { cable,  qty 0,  price 5.0   },]
    }

    // 3. Remove stock. Take a name and an amount and change the warehouse in place.
    fn remove_stock(&mut self, name: &str, amount: u32) -> Result<String, String> {
        for item in &mut self.items {
            if item.name == name {
                if amount > item.quantity {
                    return Err(format!(
                        "not enough stock: have {}, want {}",
                        item.quantity, amount
                    ));
                }
                item.quantity -= amount;
                return Ok(format!("{} left", item.quantity));
            }
        }
        Err(format!("no such item: {}", name))
    }

    // 5. Cheapest item. Given the warehouse, return the cheapest item that is in stock, or nothing.
    fn cheapest_in_stock(&self) -> Option<&Item> {
        // Iterator version (same result):
        // self.items
        //     .iter()
        //     .filter(|item| item.quantity > 0)
        //     .min_by(|a, b| {
        //         a.price
        //             .partial_cmp(&b.price)
        //             .unwrap_or(std::cmp::Ordering::Equal)
        //     })

        let mut cheapest: Option<&Item> = None; // nothing found yet

        for item in &self.items {
            if item.quantity == 0 {
                continue; // out of stock, ignore it
            }

            match cheapest {
                None => cheapest = Some(item), // first in-stock item becomes the cheapest so far
                Some(best) => {
                    if item.price < best.price {
                        cheapest = Some(item); // found a cheaper one
                    }
                }
            }
        }

        cheapest
    }

    // 6. Report. Given the warehouse, produce one text line per item.
    fn report(&self) -> Vec<String> {
        // in report(), above `let mut lines`
        // Iterator version (same result):
        // self.items
        //     .iter()
        //     .map(|item| format!("{} x{} ({})", item.name, item.quantity, item.stock_level()))
        //     .collect()

        let mut lines: Vec<String> = Vec::new();

        for item in &self.items {
            let line = format!("{} x{} ({})", item.name, item.quantity, item.stock_level());
            lines.push(line);
        }

        lines
    }

    // 7. Close out. Take the warehouse away from the caller completely and return its total value.
    fn close_out(self) -> f64 {
        // in close_out(), above `let mut total`
        // Iterator version (same result):
        // self.items
        //     .into_iter()
        //     .map(|item| item.quantity as f64 * item.price)
        //     .sum()

        let mut total: f64 = 0.0;

        for item in self.items {
            total += item.quantity as f64 * item.price;
        }

        total
    }
}

fn main() {
    // 1. Loading bad data where line 3 has a bad price
    let bad_lines = ["laptop,3,999.5", "mouse,10,25.0", "phone,2,-5"];
    let bad_res = Warehouse::from_lines(&bad_lines);
    println!("{:?}", bad_res);

    // 2. Loading good data (including a cheaper item that is out of stock)
    let good_lines = ["laptop,3,999.5", "mouse,10,25.0", "cable,0,5.0"];
    let mut warehouse = Warehouse::from_lines(&good_lines).expect("valid warehouse data");

    // 3. Removing stock: success, unknown item, too much
    println!("{:?}", warehouse.remove_stock("laptop", 1));
    println!("{:?}", warehouse.remove_stock("phone", 1));
    println!("{:?}", warehouse.remove_stock("laptop", 10));

    // 4. Cheapest in-stock item (cable is 5.0 but 0 qty, so mouse is cheapest in stock)
    if let Some(item) = warehouse.cheapest_in_stock() {
        println!("cheapest in stock: {}", item.name);
    }

    // 5. The report
    for line in warehouse.report() {
        println!("{}", line);
    }

    // 6. Close out (takes warehouse away completely)
    let total = warehouse.close_out();
    println!("Total value: {}", total);

    // 7. Trying to use `warehouse` after `close_out`:
    // warehouse.report();
    //
    // Compiler error (real message copied):
    // error[E0382]: borrow of moved value: `warehouse`
    //    --> task\q36.rs:229:5
    //     |
    // 208 |     let mut warehouse = Warehouse::from_lines(&good_lines).expect("valid warehouse data");
    //     |         ------------- move occurs because `warehouse` has type `Warehouse`, which does not implement the `Copy` trait
    // ...
    // 226 |     let total = warehouse.close_out();
    //     |                           ----------- `warehouse` moved due to this method call
    // ...
    // 229 |     warehouse.report();
    //     |     ^^^^^^^^^ value borrowed here after move
    //     |
    // note: `Warehouse::close_out` takes ownership of the receiver `self`, which moves `warehouse`
    //    --> task\q36.rs:184:18
    //     |
    // 184 |     fn close_out(self) -> f64 {
    //     |                  ^^^^
}
