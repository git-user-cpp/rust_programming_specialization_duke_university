enum WineGrapes {
    CabernetFranc,
    Tannat,
    Merlot,
}

fn taste_wine(grapes: WineGrapes) {
    match grapes {
        WineGrapes::CabernetFranc => println!("This is a Cabernet Franc wine."),
        _ => println!("This is not a Cabernet Franc wine."),
        // WineGrapes::Merlot => println!("This is a Merlot wine."),
        // WineGrapes::Tannat => println!("This is a Tannat wine."),
    }
}

fn main() {
    taste_wine(WineGrapes::CabernetFranc);
}