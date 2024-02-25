enum FileSize {
    Bytes(f64),
    Kilobytes(f64),
    Megabytes(f64),
    Gigabytes(f64),
    Terabytes(f64),
}

fn format_size(size: u64) -> String {
    let filesize = match size {
        0..=999 => FileSize::Bytes(size as f64),
        1000..=999_999 => FileSize::Kilobytes((size as f64) / 1000.0),
        1_000_000..=999_999_999 => FileSize::Megabytes((size as f64) / 1_000_000.0),
        1_000_000_000..=999_999_999_999 => FileSize::Gigabytes((size as f64) / 1_000_000_000.0),
        _ => FileSize::Terabytes((size as f64) / 1_000_000_000_000.0),
    };

    match filesize {
        FileSize::Bytes(bytes) => format!("{} bytes", bytes),
        FileSize::Kilobytes(kb) => format!("{:.2} KB", kb),
        FileSize::Megabytes(mb) => format!("{:.2} MB", mb),
        FileSize::Gigabytes(gb) => format!("{:.2} GB", gb),
        FileSize::Terabytes(tb) => format!("{:.2} TB", tb),
    }
}

fn largest_unit_format(size: u64) -> String {
    format_size(size)
}

fn main() {
    let terabyte_size = 1_234_567_890_123_456;
    let terabyte_result = format_size(terabyte_size);
    println!("Terabyte format: {}", terabyte_result);

    let test_cases = [2500, 1_048_576, 2_000_000_000];
    for size in test_cases {
        let result = largest_unit_format(size);
        println!("Largest unit format for {} bytes: {}", size, result);
    }
}