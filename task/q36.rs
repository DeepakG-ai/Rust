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
// 3. Remove stock. Take a name and an amount and change the warehouse in place.
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

fn main() {}
