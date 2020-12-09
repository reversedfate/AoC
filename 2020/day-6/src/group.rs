pub mod group {
    use std::collections::HashMap;
    pub struct Group {
        letter_count: HashMap <char, u32>,
        entries: u32
    }

    impl Group {
        pub fn new() -> Group {
            Group {
                letter_count: HashMap::new(),
                entries: 0
            }
        }

        pub fn add_line(&mut self, line: &str) {
            for ch in line.chars(){
                if ch != ' ' {
                    if self.letter_count.contains_key(&ch) {
                        *self.letter_count.get_mut(&ch).unwrap() +=1;
                    } else {
                        self.letter_count.insert(ch, 1);
                    }
                }
            }
            self.entries += 1;
        }

        pub fn get_unique_letter_count(&self) -> usize {
            return self.letter_count.keys().len();
        }

        pub fn get_letter_count_in_all(&self) -> usize {
            let mut total = 0;
            for ch in self.letter_count.keys() {
                if self.letter_count[ch] == self.entries {
                    total += 1;
                };
            }
            return total;
        }
    }   
}

#[cfg(test)]
mod tests {
    use crate::group::group::Group;

    #[test]
    fn group_letter_count_0() {
        let mut test_group = Group::new(); 
        assert_eq!(0, test_group.get_unique_letter_count());
        assert_eq!(0, test_group.get_letter_count_in_all());
    }
    #[test]
    fn group_letter_count_1() {
        let mut test_group = Group::new(); 
        test_group.add_line("a"); 
        assert_eq!(1, test_group.get_unique_letter_count());
        assert_eq!(1, test_group.get_letter_count_in_all());
    }
    #[test]
    fn group_letter_count_1_same() {
        let mut test_group = Group::new(); 
        test_group.add_line("a"); 
        test_group.add_line("a"); 
        test_group.add_line("b"); 
        test_group.add_line("c d e"); 
        assert_eq!(5, test_group.get_unique_letter_count());
        assert_eq!(0, test_group.get_letter_count_in_all());
    }
    #[test]
    fn group_letter_count_2() {
        let mut test_group = Group::new(); 
        test_group.add_line("a"); 
        test_group.add_line("b"); 
        assert_eq!(2, test_group.get_unique_letter_count());
        assert_eq!(0, test_group.get_letter_count_in_all());
    }
    #[test]
    fn group_letter_count_5() {
        let mut test_group = Group::new(); 
        test_group.add_line("a b c d"); 
        test_group.add_line("d e"); 
        assert_eq!(5, test_group.get_unique_letter_count());
        assert_eq!(1, test_group.get_letter_count_in_all());
    }
}