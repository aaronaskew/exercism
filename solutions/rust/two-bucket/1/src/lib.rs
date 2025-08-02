use std::collections::{HashSet, VecDeque};

#[derive(PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

/// INPUT: Two integers a, b
/// OUTPUT: (gcd, x, y) such that a×x + b×y = gcd(a,b)
fn extended_gcd(a: i32, b: i32) -> (i32, i32, i32) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (gcd, x1, y1) = extended_gcd(b, a % b);
        let x = y1;
        let y = x1 - (a / b) * y1;
        (gcd, x, y)
    }
}

/// Solve the bucket problem
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    // ensure solution exists
    if goal > capacity_1.max(capacity_2) {
        return None;
    }
    let (gcd, _, _) = extended_gcd(capacity_1 as i32, capacity_2 as i32);
    if !goal.is_multiple_of(gcd as u8) {
        return None;
    }

    let mut queue = VecDeque::new();
    queue.push_back((0, 0, vec![]));
    let mut visited = HashSet::new();
    visited.insert((0, 0));

    while !queue.is_empty() {
        let (a, b, path) = queue.pop_front().unwrap();

        if a == goal || b == goal {
            return Some(BucketStats {
                moves: path.len() as u8,
                goal_bucket: if a == goal { Bucket::One } else { Bucket::Two },
                other_bucket: if a == goal { b } else { a },
            });
        }

        let mut next_states = vec![];

        if a == 0 && b == 0 {
            // enforce rule regarding the start bucket to fill
            match start_bucket {
                Bucket::One => {
                    next_states.push((capacity_1, b, "fill A"));
                }
                Bucket::Two => {
                    next_states.push((a, capacity_2, "fill B"));
                }
            }
        } else {
            // Fill bucket A
            if a < capacity_1
                && (*start_bucket == Bucket::One || (*start_bucket == Bucket::Two && b != 0))
            {
                next_states.push((capacity_1, b, "fill A"));
            }
            // Fill bucket B
            if b < capacity_2
                && (*start_bucket == Bucket::Two || (*start_bucket == Bucket::One && a != 0))
            {
                next_states.push((a, capacity_2, "fill B"));
            }
            // Empty bucket A
            if a > 0
                && (*start_bucket == Bucket::Two
                    || (*start_bucket == Bucket::One && b != capacity_2))
            {
                next_states.push((0, b, "empty A"));
            }
            // Empty bucket B
            if b > 0
                && (*start_bucket == Bucket::One
                    || (*start_bucket == Bucket::Two && a != capacity_1))
            {
                next_states.push((a, 0, "empty B"));
            }
            // Pour A into B
            if a > 0 && b < capacity_2 {
                let pour_amount = a.min(capacity_2 - b);
                next_states.push((a - pour_amount, b + pour_amount, "pour A->B"))
            }
            // Pour B into A
            if b > 0 && a < capacity_1 {
                let pour_amount = b.min(capacity_1 - a);
                next_states.push((a + pour_amount, b - pour_amount, "pour B->A"))
            }
        }

        // Add valid next states to queue
        for (next_a, next_b, action) in next_states {
            if !visited.contains(&(next_a, next_b)) {
                visited.insert((next_a, next_b));
                let mut new_path = path.clone();
                new_path.append(&mut vec![(action, next_a, next_b)]);
                queue.push_back((next_a, next_b, new_path));
            }
        }
    }

    None
}
