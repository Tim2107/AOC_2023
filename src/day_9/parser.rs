





#[cfg(test)]
mod tests{
    use super::*;
    use crate::utils::input_output::read_file;

    #[test]
    fn test_read_test_file(){
        let content = read_file("resources/input_day_9_test.txt").unwrap();
        assert!(!content.is_empty());
    }
}
