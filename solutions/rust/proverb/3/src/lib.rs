use itertools::Itertools;

pub fn build_proverb(list: &[&str]) -> String {
    let mut proverb = String::new();

    list.iter()
        .with_position()
        .enumerate()
        .for_each(|(i, (position, word))| match position {
            itertools::Position::First | itertools::Position::Middle => {
                proverb += &format!("For want of a {} the {} was lost.\n", word, list[i + 1]);
            }
            itertools::Position::Last | itertools::Position::Only => {
                proverb += &format!("And all for the want of a {}.", list[0]);
            }
        });

    proverb
}
