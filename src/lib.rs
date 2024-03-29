pub fn get_diamond(c: char) -> Vec<String> {
    let mut diamond: Vec<String> = Vec::new();
    let my_reference: [char; 26] = ['A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z'];
    let position = my_reference.iter().position(|&x| x == c).unwrap(); // To get the index of the character
    let rows = 2 * position + 1; // For the total rows.

    if c == 'A' {
        return vec!['A'.to_string()];
    }

    for i in 1..rows {
        if i < rows/2 {
            for x in 'A'..c {
                let spaces_outside = ' ' * position;
                let spaces_inside = ' ' * ((2 * position) -1);
                let current_char = x.to_uppercase();
                let half = format!("{}{}{}{}{}", spaces_outside,current_char,spaces_inside,current_char,spaces_outside)
                diamond.push(half);
            }
        }
}
