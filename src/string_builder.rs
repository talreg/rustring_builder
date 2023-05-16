use std::ops::{Add, AddAssign};

///
/// ## String Builder - reusable char buffer
/// * The StringBuilder object allows you to add any object (including custom types)
/// as long as they implement the ToString trait.
/// * StringBuilder doesn't consume the object appended
/// * StringBuilder is not consumed by calling the to_string() method.
/// * StringBuilder can be appended to another StringBuilder.
/// ## Consuming methods:
/// * Operators, such as + or += will consume the respective value.
/// * Iterator will also consume the value
/// * Direct access (nth) will **not** consume the value.

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
        return sb;
    }
    pub fn append<T: ToString>(&mut self, what: T) {
        for ch in what.to_string().chars() {
            self.data.push(ch)
        }
    }

    pub fn replace_content_with<T: ToString>(&mut self, what: T) {
        self.data.clear();
        self.append(what);
    }

    pub fn append_line<T: ToString>(&mut self, what: T) {
        self.append("\n");
        self.append(what);
    }

    pub fn clear_buffer(&mut self) {
        self.data.clear();
    }

    pub fn get_buffer_size(&self) -> usize {
        return self.data.len();
    }

    pub fn set_value<T: ToString>(&mut self, what: T) {
        self.data.clear();
        self.append(what);
    }
}

impl PartialEq for StringBuilder{
    fn eq(&self, other: &Self) -> bool {
        if self.data.len()!=other.data.len(){
            return false;
        }
        for(i,val) in self.data.iter().enumerate(){
            if other.data[i]!=*val {return false; }
        }
        return true;
    }
}

impl Default for StringBuilder {
    fn default() -> Self {
        StringBuilder::new()
    }
}

impl ToString for StringBuilder {
    fn to_string(&self) -> String {
        let mut output: String = String::new();
        for ch in &self.data {
            output.push(*ch);
        }
        output
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
        return None;
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
        return sb;
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
        assert_eq!(0, builder.get_buffer_size());
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