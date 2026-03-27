use std::collections::HashMap;

fn scramble(s1: &str, s2: &str) -> bool {
    let mut map: HashMap<char, u32> = HashMap::new();

    for c in s1.chars() {
        *map.entry(c).or_insert(0) += 1;
    }

    for c in s2.chars() {
        match map.get_mut(&c) {
            Some(count) => {
                if *count > 0 {
                    *count -= 1
                } else {
                    return false;
                }
            }
            None => return false,
        }
    }

    true
}

#[cfg(test)]
mod sample_tests {
    use super::scramble;

    fn assert_scramble(s1: &str, s2: &str, expected: bool) {
        assert_eq!(expected, scramble(s1, s2));
    }

    #[test]
    fn basic_tests() {
        assert_scramble("rkqodlw", "world", true);
        assert_scramble("cedewaraaossoqqyt", "codewars", true);
        assert_scramble("katas", "steak", false);
        assert_scramble("scriptjavx", "javascript", false);
        assert_scramble("scriptingjava", "javascript", true);
        assert_scramble("rkqodlw", "world", true);
        assert_scramble("scriptsjava", "javascript", true);
        assert_scramble("javscripts", "javascript", false);
        assert_scramble("aabbcamaomsccdd", "commas", true);
        assert_scramble("commas", "commas", true);
        assert_scramble("sammoc", "commas", true);
    }
}
