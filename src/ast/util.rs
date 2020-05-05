//! The templates of functions for reuse shared code between modules.
pub struct Util;

impl Util {
    /// Detect one word in string.
    /// ```
    /// use qdb_ast::ast::util::*;
    /// assert_eq!(true,Util::is_single_word("myVar".to_string()));
    /// assert_eq!(false,Util::is_single_word("it's not var".to_string()));
    /// assert_eq!(false,Util::is_single_word("1var".to_string()));
    /// ```
    pub fn is_single_word(var_name: String) -> bool {
        let words: Vec<&str> = var_name.split(' ').collect();
        let words: Vec<&&str> = words.iter().filter(|e| !e.is_empty()).collect();
        if words.len() > 1 {
            return false;
        }

        let mut word = words[0].to_owned().char_indices();
        let (_, c) = word.next().unwrap();
        if c.is_numeric() {
            return false;
        }

        return true;
    }

    /// Identify type from string value
    /// ```
    /// use qdb_ast::ast::util::*;
    /// assert_eq!("int".to_string(),Util::identify_type(&"32".to_string()))
    /// ```
    pub fn identify_type(term: &String) -> String {
        use crate::ast::types_annotations::{BOOL, INT, NULL, REAL, SYMBOL, TEXT};

        let term = term.chars().collect::<Vec<char>>();

        if term.iter().fold(true, |acc, e| acc && (e.is_numeric())) {
            return INT.to_string();
        };
        if term
            .iter()
            .fold(true, |acc, e| acc && (e.is_numeric() || e == &'.'))
        {
            return REAL.to_string();
        };

        match term[..] {
            ['n', 'u', 'l', 'l'] => NULL.to_string(),
            ['t', 'r', 'u', 'e'] | ['f', 'a', 'l', 's', 'e'] => BOOL.to_string(),
            ['\'', .., '\''] => TEXT.to_string(),
            _ => SYMBOL.to_string(),
        }
    }
}
