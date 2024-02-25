#[derive(Debug)]
enum WineRegions {
    Bordeaux,
    Burgundy,
    Champagne,
    Tuscany,
    Rioja,
    NapaValley,
}

struct Wine {
    name: String,
    region: WineRegions,
}

fn supported_regions(w: WineRegions) {
    match w {
        WineRegions::Rioja => println!("Rioja is supported"),
        _ => println!("{:?} Is not supported", w),
    }
}

fn main() {
    let wine1: Wine = Wine {
        name: String::from("Chetau Margaux"),
        region: WineRegions::Bordeaux,
    };

    let wine2: Wine = Wine {
        name: String::from("Barolo"),
        region: WineRegions::Tuscany,
    };

    println!("Wine 1: {} from {:?}", wine1.name, wine1.region);
    println!("Wine 2: {} from {:?}", wine2.name, wine2.region);

    supported_regions(wine1.region);
    supported_regions(wine2.region);
    supported_regions(WineRegions::Rioja);
}