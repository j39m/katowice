use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::io::Write;

fn main() {
    let path = Path::new("/proc/acpi/ibm/led");
    let pdis = path.display();

    let mut fpo = match File::create(&path) {
        Err(why) => panic!("Can't open {}: {}", pdis, why.description()),
        Ok(fpo) => fpo,
    };

    match fpo.write_all(b"0 on\n") {
        Err(why) => panic!("Can't write {}: {}", pdis, why.description()),
        Ok(_) => (),
    }
    match fpo.flush() {
        Err(why) => panic!("Can't flush {}: {}", pdis, why.description()),
        Ok(_) => (),
    }
}
