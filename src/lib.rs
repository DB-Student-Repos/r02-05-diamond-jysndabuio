pub fn get_diamond(c: char) -> Vec<String> {
    let mut diamond: Vec<String> = Vec::new();
    let my_reference: [char; 26] = ['A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z'];
    let position = my_reference.iter().position(|&x| x == c).unwrap(); // To get the index of the character
    let rows = 2 * position + 1; // For the total rows.

    for i in 0..rows {
        let mut line = String::new();
        let spaces = (position as isize - i as isize).abs() as usize;
        let current_char = if i <= position {
            my_reference[i]
        } else {
            my_reference[2 * position - i]
        };

        // Append spaces before the character
        line += &" ".repeat(spaces);

        // Append the character and spaces after the character
        line.push(current_char);
        if current_char != 'A' {
            line += &" ".repeat((2 * (position - spaces)) - 1);
            line.push(current_char);
        }

        diamond.push(line);
    }

    diamond
}
