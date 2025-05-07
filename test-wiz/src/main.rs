use wizify::Wizard;

#[derive(Wizard)]
struct Testing {
    name: String,
    id: String,
    test: i32,
}

fn main() {
    Testing::wizard();
}
