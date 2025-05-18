use wizify::Wizard;

#[derive(Wizard, Debug)]
#[wizard(
    begin_msg = "✨ Hello, and welcome to the wizify creation wizard 🐧\n\n",
    closing_msg = "\nThat was the demonstration of the wizify creation wizard. 🌛",
    prefix = " ❓ => "
)]
struct Testing {
    #[wizard(prompt = "Enter your name (Optional)")]
    name: Option<String>,
    #[wizard(prompt = "Enter your favorite color")]
    favorite_color: String,
    #[wizard(prompt = "Enter your favorite number between 0 and 9", validate = input < 10)]
    favorite_number: i32,
}

fn main() {
    let object = Testing::wizard();
    println!("\n\nName: {}", object.name.unwrap_or("".to_string()));
    println!("Favorite Color: {}", object.favorite_color);
    println!("Favorite Number: {}", object.favorite_number);
}
