#[path = "../rust_with_examples_activities/mod.rs"]
mod rust_with_examples_activities;

fn main() {
    // Call the function from the module
    // print a line of "-" with 80 char
    println!("--------------------------------------------------------------------------------");
    println!("Running rust_with_examples_activities::ch01_hello_world::display...\n");
    rust_with_examples_activities::ch01_hello_world::display::run();
    println!("--------------------------------------------------------------------------------");
    println!("Running rust_with_examples_activities::ch01_hello_world::display_testcase_list...\n");
    rust_with_examples_activities::ch01_hello_world::display_testcase_list::run();
    println!("--------------------------------------------------------------------------------");
    println!("Running rust_with_examples_activities::ch01_hello_world::formatted_print...\n");
    rust_with_examples_activities::ch01_hello_world::formatted_print::run();
    println!("--------------------------------------------------------------------------------");
    println!("Running rust_with_examples_activities::ch01_hello_world::formatting...\n");
    rust_with_examples_activities::ch01_hello_world::formatting::run();

}
