#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }
    match num as i64 - aliquot_sum(num) as i64 {
        0 => Some(Classification::Perfect),
        x if x > 0 => Some(Classification::Deficient),
        _ => Some(Classification::Abundant),
    }
}

fn aliquot_sum(num: u64) -> u64 {
    (1..num).filter(|i| num % i == 0).sum()
}
