mod rust_with_examples_activities;

include!(concat!(env!("OUT_DIR"), "/run_all.rs"));

fn main() {
    run_all();
}
