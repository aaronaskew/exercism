pub fn actions(n: u8) -> Vec<&'static str> {
    let mut actions = vec![];

    if n & 1 == 1 {
        actions.push("wink");
    }

    if n >> 1 & 1 == 1 {
        actions.push("double blink");
    }

    if n >> 2 & 1 == 1 {
        actions.push("close your eyes");
    }

    if n >> 3 & 1 == 1 {
        actions.push("jump");
    }

    if n >> 4 & 1 == 1 {
        actions.reverse();
    }

    actions
}
