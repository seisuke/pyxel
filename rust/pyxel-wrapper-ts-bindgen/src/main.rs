mod generate;
mod generate_wrapper_rust;
mod template;

fn main() {
    generate::generate().expect("Failed to generate bindings");
}
