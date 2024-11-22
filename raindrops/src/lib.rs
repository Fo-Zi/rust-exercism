pub fn raindrops(n: u32) -> String {
    let mut drop_sound = String::new();
    let mut is_divisable = false;
    if n % 3 == 0 {
        drop_sound += &format!("Pling");
        is_divisable = true;
    }

    if n % 5 == 0 {
        drop_sound += &format!("Plang");
        is_divisable = true;
    }

    if n % 7 == 0 {
        drop_sound += &format!("Plong");
        is_divisable = true;
    }

    if !is_divisable {
        drop_sound = n.to_string();
    }

    drop_sound
}
