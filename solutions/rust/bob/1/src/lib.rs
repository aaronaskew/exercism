pub fn reply(message: &str) -> &str {

    if message.split_whitespace().count() == 0 {
        return "Fine. Be that way!";
    }

    let alpha = message
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect::<String>();

    let is_shouting = !alpha.is_empty() && alpha.chars().all(|c| c.is_uppercase());

    match (message.trim().ends_with('?'), is_shouting) {
        (true, true) => "Calm down, I know what I'm doing!",
        (false, true) => "Whoa, chill out!",
        (true, false) => "Sure.",
        _ => "Whatever.",
    }
}
