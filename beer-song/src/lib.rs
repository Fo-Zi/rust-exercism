use std::fmt::format;

enum Sentence{
    BottlesInTheWall,
    Bottles,
    MidSentence,
    LastSentence,
}

pub fn sentence_builder(nr_beers:u32, sentence: Sentence) -> String{
    match sentence{
        Sentence::BottlesInTheWall => match nr_beers {
            0 => String::from("No more bottles of beer on the wall"),
            1 => String::from("1 bottle of beer on the wall"),
            nr_bottles => format!("{nr_bottles} bottles of beer on the wall"),
        },
        Sentence::Bottles => match nr_beers {
            0 => String::from("no more bottles of beer"),
            1 => String::from("1 bottle of beer"),
            nr_bottles => format!("{nr_bottles} bottles of beer"),
        },
        Sentence::MidSentence => match nr_beers{
            0 => String::from("Go to the store and buy some more"),
            1 => String::from("Take it down and pass it around"),
            _ => String::from("Take one down and pass it around"),
        },
        Sentence::LastSentence => match nr_beers{
            0 => String::from("no more bottles of beer on the wall"),
            1 => String::from("1 bottle of beer on the wall"),
            nr_bottles => format!("{nr_bottles} bottles of beer on the wall"),
        },
    }
}

macro_rules! first_sentence {
    ($x: expr) => {
        sentence_builder($x, Sentence::BottlesInTheWall)
    };
}

macro_rules! second_sentence {
    ($x: expr) => {
        sentence_builder($x, Sentence::Bottles)
    };
}

macro_rules! third_sentence {
    ($x: expr) => {
        sentence_builder($x, Sentence::MidSentence)
    };
}

macro_rules! fourth_sentence {
    ($x: expr) => {
        sentence_builder($x, Sentence::LastSentence)
    };
}

pub fn verse(n: u32) -> String {
    match n{
        0 => format!("{}, {}.\n{}, {}.\n",first_sentence!(0), second_sentence!(0), third_sentence!(0), fourth_sentence!(99) ),
        x => format!("{}, {}.\n{}, {}.\n",first_sentence!(x), second_sentence!(x), third_sentence!(x), fourth_sentence!(x-1) ),
    }
}

pub fn sing(start: u32, end: u32) -> String {
     (end..=start)
        .rev()
        .map(verse)
        .collect::<Vec<String>>()
        .join("\n")
}
