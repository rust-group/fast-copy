use std::env;

extern crate fcp_core;

fn main() {
    let mut args = env::args();
    if args.len() > 1 {
        args.next();
        let src_file = args.next().unwrap_or_default();
        let src_file = src_file.as_str();
        let dest_file = args.next().unwrap_or_default();
        let dest_file = dest_file.as_str();
        fcp_core::copy(src_file, dest_file);
    }
}
