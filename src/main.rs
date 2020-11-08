mod output_log;
mod buffered_writes;
mod progress_bar;
mod string_vs_str;
mod testing_with_stdout;

fn main() {
    println!("Rust (personal) Recipes gleaned from my own learnings");

    println!("\n--- strings vs string slices ---\n");
    string_vs_str::main();

    println!("\n--- buffered writes ---\n");
    buffered_writes::main().unwrap();

    println!("\n--- progress bar ----\n");
    progress_bar::main();

    println!("\n--- logging ---\n");
    output_log::main();
}
