mod lib;

use std::mem;

fn main() {
    let message = lib::create_message("Hello world!".to_string());
    println!("Created message struct: {:?}, byte_size: {}", message, mem::size_of_val(&message));

    let serialized_message = lib::serialize_message(&message);
    println!("Serialized message: {:?}, byte_size: {}", serialized_message, mem::size_of_val(&serialized_message));

    let deserialized_message = lib::deserialize_message(&serialized_message);
    println!("Deserialized message: {:?}, byte_size: {}", deserialized_message, mem::size_of_val(&deserialized_message));
}
