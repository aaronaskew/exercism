const STUDENTS: [&str; 12] = [
    "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
    "Kincaid", "Larry",
];

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let index = STUDENTS.iter().position(|s| *s == student).unwrap() * 2;

    diagram
        .lines()
        .flat_map(|row| {
            row[index..=index + 1]
                .chars()
                .map(|plant_char| match plant_char {
                    'G' => "grass",
                    'C' => "clover",
                    'R' => "radishes",
                    'V' => "violets",
                    _ => "",
                })
        })
        .collect()
}
