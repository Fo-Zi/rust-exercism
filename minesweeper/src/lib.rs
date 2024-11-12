use std::vec;

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut vec_output: Vec<String> = Vec::new();
    for row_num in 0..minefield.len() {
        let mut row_string: String = String::new();
        for elem_index in 0..minefield[row_num].len(){
            if minefield[row_num].as_bytes()[elem_index] != '*' as u8 {
                let num_of_mines_adjacent = get_nr_adjacent_mines(minefield, row_num, elem_index);
                let mut output_char = ' ';
                if num_of_mines_adjacent > 0 {
                    output_char = char::from_digit(num_of_mines_adjacent as u32, 10).unwrap();
                }            
                row_string.insert(elem_index, output_char);
            }else{
                row_string.insert(elem_index, '*');
            }
        }
        vec_output.push(row_string);
    };
    vec_output
}

pub fn get_nr_adjacent_mines( minefield: &[&str], row_index: usize, elem_index: usize ) -> u8 {
    let mut mine_counter = 0;
    if minefield.len() > 2{
        mine_counter += get_nr_mines_adjacent_to_elem_in_slice(minefield[row_index], elem_index);     
        if row_index == 0 {
            mine_counter += get_nr_mines_in_slice(minefield[row_index+1], elem_index);     
        } else if row_index == minefield.len() - 1 {
            mine_counter += get_nr_mines_in_slice(minefield[row_index-1], elem_index);     
        } else {
            mine_counter += get_nr_mines_in_slice(minefield[row_index-1], elem_index);     
            mine_counter += get_nr_mines_in_slice(minefield[row_index+1], elem_index);     
        };
    }else if minefield.len() == 1{
        mine_counter += get_nr_mines_adjacent_to_elem_in_slice(minefield[row_index], elem_index);     
    }else{
        return 0;
    }
    
    mine_counter
}

pub fn get_nr_mines_adjacent_to_elem_in_slice(row: &str, index: usize) -> u8 {
    let surrounding_elem = match get_slice_of_adjacent_nums(row.as_bytes(), index) {
        Some(slice) => slice,
        None => return 0,
    };
    surrounding_elem.iter().map(|x| if *x as char == '*' { 1 } else { 0 } ).sum()
}

pub fn get_nr_mines_in_slice(row: &str, index: usize) -> u8 {
    let surrounding_elem = match get_slice_of_adjacent_nums(row.as_bytes(), index) {
        Some(slice) => slice,
        None => return 0,
    };
    surrounding_elem.iter().map(|x| if *x as char == '*' { 1 } else { 0 } ).sum()
}

fn get_slice_of_adjacent_nums(slice: &[u8], index: usize) -> Option<&[u8]> {
    // If index is out of bounds, return None
    if slice.len() == 1 {
        return Some(slice);
    }

    if index == 0 {
        slice.get(0..=1)  // Take current and next element if index is 0
    } else if index == slice.len() - 1 {
        slice.get(index - 1..=index)  // Take previous and current element if it's the last element
    } else {
        slice.get(index - 1..=index + 1)  // Take previous, current, and next element
    }
}