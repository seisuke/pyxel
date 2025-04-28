mod generate;

fn main() {
    generate::generate().expect("Failed to generate bindings");
}
