use std::env::args;

use zippyst::file::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    for link in args().skip(1) {
        let info = File::new(&link).get_information()?;
        println!("{}", info.full_link());
    }
    Ok(())
}
