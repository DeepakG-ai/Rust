use std::collections::HashMap;

fn word_count(text:&str)->HashMap<String,usize>{
    let mut map = HashMap::new(); //created new empty map
    for word in text.split_whitespace(){   //loop over whitespace and convert them into lowercase
        let lower_word = word.to_lowercase();
        *map.entry(lower_word).or_insert(0) += 1; //get the key, otherwise return 0
    }
    map



}

fn main() {
    let text = "the quick the lazy THE dog";
    let dict = word_count(text);
    let mut items: Vec<(String, usize)> = dict.into_iter().collect();
    items.sort();
    for (word, count) in items {
        println!("{}: {}", word, count);
    }
}


//Word "the"   → map is empty     → entry("the") → Vacant  → insert 0 → *0 += 1 → map = {"the": 1}
//Word "quick" → "quick" is new   → entry("quick")→ Vacant  → insert 0 → *0 += 1 → map = {"the": 1, "quick": 1}
//Word "the"   → "the" EXISTS     → entry("the") → Occupied → get &mut 1 → *1 += 1 → map = {"the": 2, "quick": 1}
//Word "lazy"  → "lazy" is new    → insert 0 → *0 += 1 → map = {..., "lazy": 1}
//Word "the"   → "the" EXISTS     → get &mut 2 → *2 += 1 → map = {"the": 3, ...}
//Word "dog"   → "dog" is new     → insert 0 → *0 += 1 → map = {..., "dog": 1}
//Each item = (String, usize) = ("the", 3), ("quick", 1), ("lazy", 1), ("dog", 1)
//collect() into Vec<(String,usize)>
// Result :  items = [("the", 3), ("quick", 1), ("lazy", 1), ("dog", 1)]