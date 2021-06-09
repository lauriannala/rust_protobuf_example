mod lib;

fn main() {
    let message = lib::create_message("Hello world!".to_string());
    println!("Created message struct: {:?}", message);

    let serialized_message = lib::serialize_message(&message);
    println!("Serialized message: {:?}", serialized_message);

    let deserialized_message = lib::deserialize_message(&serialized_message);
    println!("Deserialized message: {:?}", deserialized_message);
}
