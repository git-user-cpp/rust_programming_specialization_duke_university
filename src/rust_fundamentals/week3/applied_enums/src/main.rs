use std::{io::Bytes, result};

enum FileSize {
    Bytes(u64),
    KibiBytes(u64),
    MebiBytes(u64),
    GibiBytes(u64),
}

fn format_size(size: u64) -> String {
    let file_size = match size {
        0..=1023 => FileSize::Bytes(size),
        1024..=1_023_999 => FileSize::KibiBytes(size / 1024),
        1_024_000..=1_023_999_999 => FileSize::GibiBytes(size / 1_024_000),
        _ => FileSize::GibiBytes(size / 1_024_000_000)
    };

    match file_size {
        FileSize::Bytes(bytes) => format!("{} bytes", bytes),
        FileSize::KibiBytes(kibibytes) => format!("{:.2} kibibytes", kibibytes as f64 / 1024.0),
        FileSize::MebiBytes(mebibytes) => format!("{:.2} mebibytes", mebibytes as f64 / 1024.0),
        FileSize::GibiBytes(gibibytes) => format!("{:.2} gibibytes", gibibytes as f64 / 1024.0),
    }
}

fn main() {
    let result = format_size(63688883700399);
    println!("{}", result);
}