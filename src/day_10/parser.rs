pub struct Parser {
   pub map: Vec<Vec<char>>,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        Parser {
            map: input.lines()
                .map(|line| line.chars().collect())
                .collect(),
        }
    }

    pub fn find_starting_position(&self) -> Option<(usize, usize)> {
        self.map.iter().enumerate().find_map(|(y, row)| {
            row.iter().enumerate().find_map(|(x, &ch)| {
                if ch == 'S' { Some((x, y)) } else { None }
            })
        })
    }


}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::input_output::read_file;

    #[test]
    fn test_read_file() {
        let content = read_file("resources/input_day_10_test_a.txt").unwrap();
        assert!(!content.is_empty());
    }

    #[test]
    fn test_find_starting_position_a() {
        test_find_starting_position_helper("resources/input_day_10_test_a.txt",(1,1));
    }

    #[test]
    fn test_find_starting_position_b() {
        test_find_starting_position_helper("resources/input_day_10_test_b.txt",(1,1));
    }

    #[test]
    fn test_find_starting_position_c() {
        test_find_starting_position_helper("resources/input_day_10_test_c.txt",(0,2));
    }

    #[test]
    fn test_find_starting_position_d() {
        test_find_starting_position_helper("resources/input_day_10_test_d.txt",(0,2));
    }

    fn test_find_starting_position_helper(input_file: &str, expected_position: (usize,usize)) {
        let content = read_file(input_file).unwrap();
        let parser = Parser::new(&content);
        assert_eq!(parser.find_starting_position().unwrap(), expected_position);
    }
}
