mod buffered_writes;
mod progress_bar;
mod string_vs_str;
mod testing_with_stdout;

fn main() {
    println!("Rust (personal) Recipes gleaned from my own learnings");

    string_vs_str::main();
    buffered_writes::main().unwrap();
    progress_bar::main();
}
