use std::fmt::format;

pub fn verse(n: u32) -> String {
    match n{
        0 => format!("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall."),
        1 => format!("{n} bottle of beer on the wall, {n} bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall."),
        2 => format!("{n} bottles of beer on the wall, {n} bottles of beer.\nTake one down and pass it around, {} bottle of beer on the wall.",n-1),
        _ => format!("{n} bottles of beer on the wall, {n} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.",n-1),
    }

}

pub fn sing(start: u32, end: u32) -> String {
    let mut string_holder = String::new();
    for n_verse in (end..=start).rev() {
        let n_verse_string = verse(n_verse);
        string_holder.push_str(n_verse_string.as_str());
        string_holder.push_str("\n\n");
    }
    string_holder
}
