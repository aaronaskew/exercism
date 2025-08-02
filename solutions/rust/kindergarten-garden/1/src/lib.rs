use std::collections::HashMap;

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let student_index_map: HashMap<&str, usize> = vec![
        ("Alice", 0),
        ("Bob", 2),
        ("Charlie", 4),
        ("David", 6),
        ("Eve", 8),
        ("Fred", 10),
        ("Ginny", 12),
        ("Harriet", 14),
        ("Ileana", 16),
        ("Joseph", 18),
        ("Kincaid", 20),
        ("Larry", 22),
    ]
    .into_iter()
    .collect();

    let letter_plant_map: HashMap<char, &str> = vec![
        ('G', "grass"),
        ('C', "clover"),
        ('R', "radishes"),
        ('V', "violets"),
    ]
    .into_iter()
    .collect();

    let index = student_index_map.get(student).unwrap();

    let mut plants = Vec::<&str>::new();

    diagram.lines().for_each(|row| {
        row.chars().enumerate().for_each(|(i, c)| {
            if i == *index || i == *index + 1 {
                plants.push(letter_plant_map.get(&c).unwrap());
            }
        });
    });

    plants
}
