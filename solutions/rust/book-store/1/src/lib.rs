use std::collections::HashMap;

fn set_price(set_length: &usize) -> u32 {
    match set_length {
        1 => 800,
        2 => 2 * 800 * 95 / 100,
        3 => 3 * 800 * 90 / 100,
        4 => 4 * 800 * 80 / 100,
        5 => 5 * 800 * 75 / 100,
        _ => 0,
    }
}

pub fn lowest_price(books: &[u32]) -> u32 {
    if books.is_empty() {
        return 0;
    }

    let mut count_map = HashMap::new();

    for book in books {
        count_map
            .entry(*book)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let mut counts = count_map.values().copied().collect::<Vec<u32>>();

    let mut set_lengths = vec![];

    while !counts.is_empty() && counts.iter().any(|count| *count > 0) {
        match counts.len() {
            num_unique_books if (1..=5).contains(&num_unique_books) => {
                set_lengths.push(num_unique_books)
            }
            _ => {}
        }

        // decrement all of the book counts and remove zeros
        counts = counts
            .iter()
            .filter_map(|&count| if count > 1 { Some(count - 1) } else { None })
            .collect();
    }

    // if set_lengths contains a five and a three, convert to two sets of length four
    while set_lengths.contains(&5) && set_lengths.contains(&3) {
        set_lengths.remove(set_lengths.iter().position(|length| *length == 5).unwrap());
        set_lengths.remove(set_lengths.iter().position(|length| *length == 3).unwrap());
        set_lengths.push(4);
        set_lengths.push(4);
    }

    set_lengths.iter().map(set_price).sum()
}
