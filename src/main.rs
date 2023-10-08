use std::process::exit;

use byte_unit::Byte;
use fs_extra::dir;

fn main() -> fs_extra::error::Result<()> {
    let red = "\x1b[91m";
    let yellow = "\x1b[93m";
    let end = "\x1b[0m";

    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!(
            "\n\t{red}Usage: {} <path/to/get/recursively/size/from>{end}\n",
            &args[0]
        );
        exit(1);
    }
    let folder = &args[1];
    println!(
        "\n{folder}: {yellow}{}{end}\n",
        Byte::from_bytes(dir::get_size(folder)?.into()).get_appropriate_unit(false)
    );
    Ok(())
}
