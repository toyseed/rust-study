

mod mod_test;

use mod_test::hello_world;
use mod_test::hello_rust;

fn main() {
    hello_world::hello_world();
    hello_rust::hello_rust();
}
