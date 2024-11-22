macro_rules! rhyme {
    ($x: expr) => {
        format!("And all for the want of a {}.",$x)
    };
    ($x: expr, $y: expr) => {
        format!("For want of a {} the {} was lost.\n",$x,$y)
    };
}

pub fn build_proverb(list: &[&str]) -> String {
    let mut output_holder = String::new();
    list.windows(2).for_each(|pair| {
        if let [prev, curr] = pair {
            output_holder += &rhyme!(prev, curr);
        }
    });
    if let Some(first) = list.first() {
        output_holder += &rhyme!(first);
    }
    output_holder
}