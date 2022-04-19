pub fn pprint(abc: &str, width: usize) -> String {
    let mut mabc = abc.chars().rev().collect::<String>();
    let act_board: String = mk_template(width)
        .chars()
        .map(|x| if x == 'x' { mabc.pop().unwrap() } else { x })
        .map(|x| if x == '/' { '.' } else { x })
        .collect();
    act_board
}

fn mk_template(width: usize) -> String {
    let mut lines: [String; 4] = [
        String::from("\n  ╔"),
        String::from("  ║"),
        String::from("  ║"),
        String::from("  ╚"),
    ];
    for _ in 1..width {
        lines[0] += "════";
        lines[1] += " x │";
        lines[2] += "───┼";
        lines[3] += "════";
    }
    lines[0] += "═══╗\n";
    lines[1] += " x ║\n";
    lines[2] += "───║\n";
    lines[3] += "═══╝\n";

    let mut tmp = String::from(&lines[0]);
    for _ in 1..width {
        tmp.push_str(&lines[1]);
        tmp.push_str(&lines[2]);
    }
    tmp.push_str(&lines[1]);
    tmp.push_str(&lines[3]);
    tmp
}
