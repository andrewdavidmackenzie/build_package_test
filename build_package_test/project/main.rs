use std::io;

pub fn main() -> io::Result<()>{
    println!("build_package_test binary");
    println!("List of modules loaded via generated.rs file");

    build_package_test::list_modules();

    Ok(())
}