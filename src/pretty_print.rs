/// return a pretty table ( as String )
///
/// String abc are the cell-values to use
///  - 0
///  - 1
///  - / or . for an empty set_field
///
/// width is the width ( and height ) of the Board
///
/// # Examples
///
/// ```
/// let tbl = pprint(".1.0..0..0..11.0", 4);
/// assert_eq!("  ╔═══════════════╗
///  ║ . │ 1 │ . │ 0 ║
///  ║───┼───┼───┼───║
///  ║ . │ . │ 0 │ . ║
///  ║───┼───┼───┼───║
///  ║ . │ 0 │ . │ . ║
///  ║───┼───┼───┼───║
///  ║ 1 │ 1 │ . │ 0 ║
///  ╚═══════════════╝", tbl);
/// ```
pub fn pprint(abc: &str, width: usize) -> String {
    let mut mabc = abc.chars().rev().collect::<String>();
    let act_board: String = mk_template(width)
        .chars()
        .map(|x| if x == 'x' { mabc.pop().unwrap() } else { x })
        .map(|x| if x == '/' { '.' } else { x })
        .collect();
    act_board
}

/// creates a board-template for pprint
///
/// internal function, returns
/// * a String, showing a board of dimension width x width
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pprint_4x4() {
        let tbl = pprint(".1.0..0..0..11.0", 4);
        assert_eq!(
            "
  ╔═══════════════╗
  ║ . │ 1 │ . │ 0 ║
  ║───┼───┼───┼───║
  ║ . │ . │ 0 │ . ║
  ║───┼───┼───┼───║
  ║ . │ 0 │ . │ . ║
  ║───┼───┼───┼───║
  ║ 1 │ 1 │ . │ 0 ║
  ╚═══════════════╝\n",
            tbl
        );
    }

    #[test]
    fn test_template_2() {
        let tmpl = mk_template(2);
        assert_eq!(
            "\n  ╔═══════╗\n  ║ x │ x ║\n  ║───┼───║\n  ║ x │ x ║\n  ╚═══════╝\n",
            tmpl
        );
    }
}
