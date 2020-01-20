/*============ Assignment7 Question3 =============
Q3. Write a Rust library,
● Make a library package
● Make a module with suitable name in library
● Make a sub module with an other name in first module
● Define a simple function in sub module
● make a binary package
● add your library in dependencies in cargo.toml
● use your library in main.rs
● Call that function from fn main
*/

use lib;
fn main() {
    military::weapon::machinegun();
}
