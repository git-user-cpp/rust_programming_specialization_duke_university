#[derive(Debug)]
enum DiskType {
    HDD,
    SSD,
}

#[derive(Debug)]
enum DiskSize {
    KB(u32),
    MB(u32),
    GB(u32),
}

fn main() {
    let disk_type: DiskType = DiskType::SSD;
    match disk_type {
        DiskType::HDD => println!("This is HDD"),
        DiskType::SSD => println!("This is SSD"),
    }
    
    let disk_size: DiskSize = DiskSize::GB(512);
    println!("{:?}", disk_size);

    let disk_type: DiskType = DiskType::HDD;
    println!("{:?}", disk_type);

    let disk_size: DiskSize = DiskSize::KB(1024);
    let disk_size2: DiskSize = DiskSize::MB(1024);

    match disk_size {
        DiskSize::KB(size) => println!("{} KibiBytes", size),
        DiskSize::MB(size) => println!("{} MebiBytes", size),
        DiskSize::GB(size) => println!("{} GibiBytes", size),
    }

    match disk_size2 {
        DiskSize::KB(size) => println!("{} KibiBytes", size),
        DiskSize::MB(size) => println!("{} MebiBytes", size),
        DiskSize::GB(size) => println!("{} GibiBytes", size),
    }
}