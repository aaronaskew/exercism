use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
};

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let letter_frequency = Arc::new(Mutex::new(HashMap::<char, usize>::new()));

    // dbg!("==============", input.len(), worker_count);

    let input_chunks = dbg!(input.chunks(input.len() / worker_count + 1));

    input_chunks.for_each(|chunk| {
        let letter_frequency = letter_frequency.clone();
        chunk.iter().for_each(|line| {
            let line = String::from(*line);
            line.chars().for_each(|c| {
                if c.is_alphabetic() {
                    *letter_frequency
                        .lock()
                        .unwrap()
                        .entry(c.to_ascii_lowercase())
                        .or_insert(0) += 1;
                }
            })
        });
    });

    letter_frequency.clone().lock().unwrap().clone()
}
