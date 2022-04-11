pub mod hash_table;
use hash_table::HashTable;

fn main() {
    let mut myhashtable: HashTable<i32, String> = HashTable::new(100);

    myhashtable.insert(12, String::from("Hello world"));
    myhashtable.insert(56, String::from("Vim rules!"));
    myhashtable.insert(2432, String::from("Semi Comma"));
    myhashtable.insert(-56, String::from("Water Bottle"));
    myhashtable.insert(0, String::from("Calculus"));
    myhashtable.insert(100, String::from("Breaking Bad"));
    myhashtable.insert(12, String::from("Rubik's Cube"));


    myhashtable.print_hash_table();

    let value = myhashtable.get(12).unwrap();
    println!("The value at {} is {}", 12, value);
}
