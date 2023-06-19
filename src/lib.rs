use std::fmt::Display;
use std::ops::{Add, AddAssign};

#[doc = include_str!("../readmes/stringbuilder.md")]
pub struct StringBuilder {
    data: Vec<char>,
    iterator_ptr: usize,
}

impl StringBuilder {
    pub fn new() -> Self {
        StringBuilder { data: Vec::new(), iterator_ptr: 0 }
    }
    pub fn new_with_value(value: impl ToString) -> Self {
        let mut sb = StringBuilder::new();
        sb.append(value);
        sb
    }
    #[doc = include_str!("../readmes/append.md")]
    pub fn append<T: ToString>(&mut self, what: T) {
        for ch in what.to_string().chars() {
            self.data.push(ch)
        }
    }
    #[doc = include_str!("../readmes/replace_content_with.md")]
    pub fn replace_content_with<T: ToString>(&mut self, what: T) {
        self.data.clear();
        self.append(what);
    }
    /// adds a newline character and then the value
    pub fn append_line<T: ToString>(&mut self, what: T) {
        self.append("\n");
        self.append(what);
    }

    /// Same as clear(). remove all content from the buffer
    pub fn clear_buffer(&mut self) {
        self.data.clear();
    }

    /// remove the content of the buffer and replace it with the value
    pub fn set_value<T: ToString>(&mut self, what: T) {
        self.data.clear();
        self.append(what);
    }
    /// Returns the length of the buffer
    pub fn len(&self) -> usize {
        self.data.len()
    }
    /// Returns true if the buffer is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    /// Remove all content from the buffer
    pub fn clear(&mut self) {
        self.data.clear();
    }
    /// Push a single character to the buffer
    pub fn push(&mut self, ch: char) {
        self.data.push(ch);
    }
}

impl Display for StringBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output: String = String::new();
        for ch in &self.data {
            output.push(*ch);
        }
        write!(f, "{}", output)
    }
}

impl PartialEq for StringBuilder {
    fn eq(&self, other: &Self) -> bool {
        if self.data.len() != other.data.len() {
            return false;
        }
        for (i, val) in self.data.iter().enumerate() {
            if other.data[i] != *val { return false; }
        }
        true
    }
}

impl Default for StringBuilder {
    fn default() -> Self {
        StringBuilder::new()
    }
}

impl Clone for StringBuilder {
    fn clone(&self) -> Self {
        let mut result = Self::new();
        result.data = self.data.clone();
        result
    }
}

impl Iterator for StringBuilder {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        let mut result = Option::None;
        if self.iterator_ptr < self.data.len() {
            result = Some(self.data[self.iterator_ptr]);
            self.iterator_ptr += 1;
        }
        result
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        if n < self.data.len() {
            return Option::Some(self.data[n]);
        }
        None
    }
}

impl Add for StringBuilder {
    type Output = StringBuilder;

    fn add(mut self, mut rhs: Self) -> Self::Output {
        let mut sb = StringBuilder {
            data: vec!(),
            iterator_ptr: 0,
        };
        sb.data.append(&mut self.data);
        sb.data.append(&mut rhs.data);
        sb
    }
}

impl AddAssign for StringBuilder {
    fn add_assign(&mut self, mut rhs: Self) {
        self.data.append(&mut rhs.data);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append_str() {
        let mut builder = StringBuilder::new();
        builder.append("test");
        assert_eq!("test".to_string(), builder.to_string())
    }

    #[test]
    fn test_len() {
        let builder = StringBuilder::new();
        assert_eq!(0, builder.len());
    }

    #[test]
    fn test_appending_other_values() {
        let mut builder = StringBuilder::new();
        builder.append(3);
        assert_eq!("3", builder.to_string());
        builder.clear_buffer();
        assert_eq!("", builder.to_string());
        builder.append(3.14);
        assert_eq!("3.14", builder.to_string());
    }

    #[test]
    fn test_compound_values() {
        let mut builder = StringBuilder::new();
        builder.append(3);
        builder.append(" little piggies");
        assert_eq!("3 little piggies", builder.to_string());
    }

    #[test]
    fn test_two_builders() {
        let mut builder1 = StringBuilder::new();
        let mut builder2 = StringBuilder::new();
        builder1.append(3);
        builder2.append(" little piggies");
        builder1.append(builder2);
        assert_eq!("3 little piggies", builder1.to_string());
    }

    #[test]
    fn test_iteration() {
        let mut builder1 = StringBuilder::new();
        builder1.append("hello world!");
        let mut data: String = String::new();
        for ch in builder1 {
            data.push(ch);
        }
        assert_eq!("hello world!", data);
    }

    #[test]
    fn test_cloning() {
        let mut builder1 = StringBuilder::new();
        builder1.append("hello world!");
        let mut builder2 = builder1.clone();
        assert_eq!("hello world!", builder2.to_string());
        builder2.append(123);
        assert_eq!("hello world!123", builder2.to_string());
    }

    #[test]
    fn test_setting() {
        let mut builder1 = StringBuilder::new();
        builder1.append("hello world!");
        let mut builder2 = builder1.clone();
        assert_eq!("hello world!", builder2.to_string());
        builder2.set_value("what's up?");
        assert_eq!("what's up?", builder2.to_string());
    }

    #[test]
    fn test_add_char() {
        let mut builder1 = StringBuilder::new();
        builder1.append('a');
        assert_eq!("a", builder1.to_string());
        builder1.append('😄');
        assert_eq!("a😄", builder1.to_string());
    }

    #[test]
    fn test_unicode_char() {
        let mut builder1 = StringBuilder::new();
        builder1.append('\u{D2FF}');
        assert_eq!("틿", builder1.to_string());
        builder1.append('\u{0913}');
        assert_eq!("틿ओ", builder1.to_string());
    }

    #[test]
    fn test_nth_value() {
        let mut builder1 = StringBuilder::new();
        builder1.append("hello world");
        assert_eq!('e', builder1.nth(1).unwrap());
    }
}

