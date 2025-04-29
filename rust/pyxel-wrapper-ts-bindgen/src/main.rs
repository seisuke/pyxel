mod generate;
mod generate_wrapper_rust;

fn main() {
    generate::generate().expect("Failed to generate bindings");
}
