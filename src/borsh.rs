use borsh::{BorshDeserialize, BorshSerialize};

// ============================================================
// BORSH = Binary Object Representation Serializer for Hashing
// ============================================================
//
// Borsh converts Rust objects into raw bytes (binary), NOT into
// human-readable formats like JSON or YAML.
//
// Key difference from Serde/JSON:
//   Serde JSON → {"username":"Deepak","password":"123"}  (human-readable text)
//   Borsh      → [6, 0, 0, 0, 68, 101, 101, 112, 97, 107, ...]  (raw bytes)
//
// ============================================================
// WHY Vec<u8>?
// ============================================================
//
// u8 = unsigned 8-bit integer = values from 0 to 255
// A byte is 8 bits, so each u8 represents exactly ONE byte.
// Vec<u8> = a list of bytes = the binary output of serialization.
//
// Example: The letter 'D' = 68 in ASCII = stored as u8 value 68
//
// ============================================================
// DETERMINISTIC (Canonical) Serialization
// ============================================================
//
// This is the key term! Borsh is DETERMINISTIC / CANONICAL.
//
// What does that mean?
//   - Fields are ALWAYS serialized in the EXACT order they are
//     defined in the struct. No exceptions.
//   - The same data ALWAYS produces the EXACT same bytes.
//
// Example:
//   struct User { username: String, password: String }
//
//   Borsh will ALWAYS serialize as: [username bytes] [password bytes]
//   It will NEVER serialize as:     [password bytes] [username bytes]
//
// Why does this matter?
//   - In blockchain (Solana), you hash the serialized bytes to create
//     a unique fingerprint. If field order could change, the hash
//     would change, and verification would FAIL.
//   - JSON does NOT guarantee this! {"a":1,"b":2} and {"b":2,"a":1}
//     are considered equal in JSON but produce DIFFERENT hashes.
//
// If you try to deserialize bytes that were serialized in a
// different field order, Borsh will give WRONG data or ERROR,
// because it reads bytes strictly in struct field order.
//
// ============================================================

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
struct User {
    username: String,
    password: String,
}

fn main() {
    // ---- SERIALIZATION: Rust object → bytes ----
    let u = User {
        username: String::from("Deepak"),
        password: String::from("1234324"),
    };

    let mut v: Vec<u8> = Vec::new(); // Create an empty byte vector to hold the serialized output
    let ans = u.serialize(&mut v); // Serialize the User struct into bytes, appending to v
    match ans {
        Ok(_) => println!("Serialized bytes: {:?}", v),
        Err(_) => println!("Error while serializing"),
    }
    // Output will be something like: [6, 0, 0, 0, 68, 101, 101, 112, 97, 107, 7, 0, 0, 0, ...]
    //                                 ↑ length of "Deepak" (6)  ↑ 'D' 'e' 'e' 'p' 'a' 'k'   ↑ length of "1234324" (7)

    // ---- DESERIALIZATION: bytes → Rust object ----
    // Takes the same bytes and converts them back into a User struct.
    // Borsh reads bytes in STRICT struct field order:
    //   1. First it reads username (because username is the FIRST field)
    //   2. Then it reads password (because password is the SECOND field)
    // If the bytes were in a different order, it would produce WRONG results!

    let deserialized_user = User::try_from_slice(&v); // Deserialize bytes back into User
    match deserialized_user {
        Ok(user) => println!("Deserialized: username={}, password={}", user.username, user.password),
        Err(e) => println!("Error while deserializing: {}", e),
    }
}
