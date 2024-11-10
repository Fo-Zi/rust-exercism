#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}
pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    let len1 = first_list.len();
    let len2 = second_list.len();

    // Determine the smaller and larger list
    let (smallest_list, largest_list, comparison_if_subset) = if len1 <= len2 {
        (first_list, second_list, Comparison::Sublist)
    } else {
        (second_list, first_list, Comparison::Superlist)
    };

    // Check for equality
    if first_list == second_list {
        return Comparison::Equal;
    }

    // Check if smallest_list is a sublist of largest_list
    let smallest_len = smallest_list.len();
    let largest_len = largest_list.len();
    
    for index in 0..=largest_len - smallest_len {
        if &largest_list[index..index + smallest_len] == smallest_list {
            return comparison_if_subset;
        }
    }

    // If no match, return Unequal
    Comparison::Unequal
}