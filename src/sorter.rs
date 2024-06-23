use std::collections::{HashMap, HashSet};

pub struct Sorter {
    ngram_len: usize,
    cached_ngrams: HashMap<String, Vec<String>>,
    cached_uppers: HashMap<String, HashSet<char>>,
}

impl Sorter {
    /// Creates a new sorter with the given n-gram length
    ///
    /// See https://en.wikipedia.org/wiki/N-gram
    pub fn new(ngram_len: usize) -> Self {
        Self {
            ngram_len,
            ..Self::default()
        }
    }

    /// Returns a list of every n-grams in the given string.
    ///
    /// This method implements a caching system if the n-grams have already been computed.
    ///
    /// See https://en.wikipedia.org/wiki/N-gram
    fn overlapping_ngrams(&mut self, s: &str) -> Vec<String> {
        if let Some(inner) = self.cached_ngrams.get(s) {
            return inner.clone();
        }

        let mut res = vec![String::new(); s.len() - self.ngram_len + 1];

        for (i, item) in res
            .iter_mut()
            .enumerate()
            .take(s.len() + 1 - self.ngram_len)
        {
            *item = s.get(i..i + self.ngram_len).unwrap().to_string();
        }

        self.cached_ngrams.insert(s.to_string(), res.clone());

        res
    }

    /// Returns a set of every uppercase character in the given string.
    ///
    /// This method has a cache and returns early if the string has already been computed.
    fn get_uppers(&mut self, s: &str) -> HashSet<char> {
        if let Some(cache) = self.cached_uppers.get("s") {
            return cache.clone();
        }

        let mut set = HashSet::default();

        for c in s.chars() {
            if c.is_uppercase() {
                set.insert(c);
            }
        }

        self.cached_uppers.insert(s.to_string(), set.clone());

        set
    }

    /// Score a prompt compared to a line.
    ///
    /// Re-implemented from [telescope] [fuzzy_file] sorter.
    ///
    /// [telescope]: https://github.com/nvim-telescope/telescope.nvim
    /// [fuzzy_file]: https://github.com/nvim-telescope/telescope.nvim/blob/f2bfde705ac752c52544d5cfa8b0aee0a766c1ed/lua/telescope/sorters.lua#L211.
    pub fn score(&mut self, prompt: &str, line: &str) -> f64 {
        let n = prompt.len();

        if n == 0 || n < self.ngram_len {
            return 1.;
        }

        let prompt_lower = prompt.to_lowercase();
        let line_lower = line.to_lowercase();

        let prompt_lower_ngram = self.overlapping_ngrams(&prompt_lower);

        // contains pure string
        let contains_string = line_lower.find(&prompt_lower);

        // uppers
        let prompt_uppers = self.get_uppers(prompt);
        let line_uppers = self.get_uppers(line);

        let mut uppers_matching = 0;
        for upper_char in prompt_uppers {
            if line_uppers.contains(&upper_char) {
                uppers_matching += 1;
            }
        }

        // tail
        let splitted_path = line_lower.rsplit_once(std::path::MAIN_SEPARATOR);
        let contains_tail = if let Some((_, tail)) = splitted_path {
            tail.contains(prompt)
        } else {
            false
        };

        // ngram consecutive matches
        let mut consecutive_matches = 0;
        let mut previous_match_index = Some(0);
        let mut match_count = 0;

        for item in &prompt_lower_ngram {
            let match_start = line_lower.find(item);
            if let Some(match_start) = match_start {
                match_count += 1;

                // Slightly different than original code because rust indices start at 0 unlike lua
                // which start at 1. None here represents 0 in lua.
                if match_start > previous_match_index.unwrap_or(0) || previous_match_index.is_none()
                {
                    consecutive_matches += 1;
                }

                previous_match_index = Some(match_start);
            }
        }

        let tail_modifier: f64 = if contains_tail { 2. } else { 1. };

        let denominator = ((10. * match_count as f64 / prompt_lower_ngram.len() as f64)
            + (3 * match_count * self.ngram_len) as f64 / line.len() as f64
            + consecutive_matches as f64
            + n as f64 / contains_string.unwrap_or(2 * line.len()) as f64
            + uppers_matching as f64)
            * tail_modifier;

        if denominator == 0. {
            return -1.;
        }

        if prompt.len() > 2 && denominator < 0.5 {
            return -1.;
        }

        1. / denominator
    }

    /// Sorts the given array by fuzzy similarity with the prompt.
    ///
    /// The search used is unstable, meaning item of equal score may be reordered. This is slightly
    /// faster.
    pub fn sort<I>(&mut self, array: &[I], prompt: &str) -> Vec<I>
    where
        I: ToString + Clone,
    {
        let mut tmp: Vec<_> = array
            .iter()
            .filter_map(|line| {
                let score = self.score(prompt, &line.to_string());

                if score < 0. {
                    None
                } else {
                    Some((line, (score * 1000.) as i16))
                }
            })
            .collect();

        tmp.sort_unstable_by_key(|(_, score)| *score);

        tmp.into_iter().map(|(item, _)| item.clone()).collect()
    }
}

impl Default for Sorter {
    fn default() -> Self {
        Self {
            ngram_len: 2,
            cached_ngrams: Default::default(),
            cached_uppers: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod cache {
        use super::*;

        #[test]
        fn test_create_ngram_cache() {
            let mut sorter = Sorter::default();

            sorter.overlapping_ngrams("fp");

            assert!(sorter.cached_ngrams.get("fp").is_some());
        }

        #[test]
        fn test_create_uppers_cache() {
            let mut sorter = Sorter::default();

            sorter.get_uppers("EgG");

            assert!(sorter.cached_uppers.get("EgG").is_some());
        }
    }

    mod ngram {
        use super::*;
        #[test]
        fn test_ngram_2_prompt_2() {
            let mut sorter = Sorter::default();

            let res = sorter.overlapping_ngrams("fp");

            assert_eq!(res, vec!["fp"]);
        }

        #[test]
        fn test_ngram_2_prompt_6() {
            let mut sorter = Sorter::default();

            let res = sorter.overlapping_ngrams("search");

            assert_eq!(res, vec!["se", "ea", "ar", "rc", "ch"]);
        }

        #[test]
        fn test_ngram_3_prompt_3() {
            let mut sorter = Sorter::new(3);

            let res = sorter.overlapping_ngrams("tet");

            assert_eq!(res, vec!["tet"]);
        }

        #[test]
        fn test_ngram_3_prompt_4() {
            let mut sorter = Sorter::new(3);

            let res = sorter.overlapping_ngrams("mule");

            assert_eq!(res, vec!["mul", "ule"]);
        }
    }

    mod uppers {
        use super::*;

        #[test]
        fn test_no_uppers() {
            let mut sorter = Sorter::default();

            let res = sorter.get_uppers("lowercase string");

            assert_eq!(res, HashSet::from([]));
        }

        #[test]
        fn test_one_upper() {
            let mut sorter = Sorter::default();

            let res = sorter.get_uppers("lowercase String");

            assert_eq!(res, HashSet::from(['S']));
        }

        #[test]
        fn test_two_different_upper() {
            let mut sorter = Sorter::default();

            let res = sorter.get_uppers("lowercasE String");

            assert_eq!(res, HashSet::from(['E', 'S']));
        }

        #[test]
        fn test_two_same_upper() {
            let mut sorter = Sorter::default();

            let res = sorter.get_uppers("lowErcasE string");

            assert_eq!(res, HashSet::from(['E']));
        }

        #[test]
        fn test_all_upper() {
            let mut sorter = Sorter::default();

            let res = sorter.get_uppers("UPPERCASE");

            assert_eq!(res, HashSet::from(['U', 'P', 'E', 'R', 'C', 'A', 'S']));
        }
    }
}
