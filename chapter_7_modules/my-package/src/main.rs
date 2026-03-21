fn main() {
    println!("Hello, world!");
    // if you have main.rs by default you have a binary create with same name as package with main.rs as create root
    // crate root is a source file that the rust compiler starts at when building a crate
    //  also makes up the root module of the crate
    //

    // if lib.rs is defined in the root of the src directory the rust will automatically create a library crate with same name as the package and lib.rs will be the crate root

    // Rules regarding crate
    /*
    1. A package must have atleast one crate
    2. A package could have either 0 lib crates or 1 lib crates
    3. A package can have any number of binary crates.
    4. To create more binary crates create a folder called bin

    */

    // To create new library create use --lib while creating new project
}
