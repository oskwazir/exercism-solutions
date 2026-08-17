

pub fn raindrops(n: u32) -> String {
    let mut raindrops = String::new();

    if n % 3 == 0 {
        raindrops.push_str("Pling");
    }

    if n % 5 == 0 {
       raindrops.push_str("Plang");
    }

    if n % 7 == 0 {
        raindrops.push_str("Plong");
    }

    if raindrops.is_empty(){
        return n.to_string();
    }

    raindrops
}
