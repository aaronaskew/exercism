pub fn recite(start_bottles: u32, take_down: u32) -> String {
    const NUMBERS: [&str; 11] = [
        "No", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten",
    ];

    let mut bottles = start_bottles;
    let mut take_downs_left = take_down;
    let mut output = String::new();

    while bottles > 0 && take_downs_left > 0 {
        let line1 = format!(
            "{} green bottle{} hanging on the wall,\n",
            NUMBERS[bottles as usize],
            match bottles {
                1 => "",
                _ => "s",
            }
        );

        let line2 = line1.clone();

        let line3 = "And if one green bottle should accidentally fall,\n";

        let line4 = format!(
            "There'll be {} green bottle{} hanging on the wall.\n\n",
            NUMBERS[bottles as usize - 1].to_lowercase(),
            match bottles - 1 {
                1 => "",
                _ => "s",
            },
        );

        bottles -= 1;
        take_downs_left -= 1;

        output.push_str(&line1);
        output.push_str(&line2);
        output.push_str(line3);
        output.push_str(&line4);
    }

    output
}
