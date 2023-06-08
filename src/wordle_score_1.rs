use std::collections::{HashMap, HashSet};
use std::cmp::min;

type HashIndexElem = HashSet<usize>;
type HashIndex = HashMap<char, HashIndexElem>;

fn hash_index(
    s: &str,
    shared_keys: &HashSet<&char>,
) -> HashIndex {
    // Return letter-to-index mapping of all letters in both
    // correct and guess.
    let mut d: HashMap<char, HashIndexElem> = HashMap::new();
    for (i, c) in s.chars().enumerate() {
        if shared_keys.contains(&c) {
            d.entry(c).or_insert_with(HashSet::new).insert(i);
        }
    }
    d
}

fn shared_indexes(
    c_hash: &HashIndex,
    g_hash: &HashIndex,
    key: &char,
) -> Vec<usize> {
    let c_indexes = c_hash.get(key).unwrap();
    let g_indexes = g_hash.get(key).unwrap();
    c_indexes.intersection(&g_indexes).map(|x| *x).collect()
}

fn get_unmatched_items(
    c_hash: &HashIndex,
    g_hash: &HashIndex,
    key: &char,
) -> Vec<usize> {
    // Return vector of letter-indexes in g_hash for instances of
    // letter `key` that are unmatched in `c_hash`. Start with
    // minimum index.
    let c_indexes: &HashIndexElem = c_hash.get(key).unwrap();
    let g_indexes: &HashIndexElem = g_hash.get(key).unwrap();
    let m = min(c_indexes.len(), g_indexes.len());

    if m == 0 {
        // If there are no unmatched indexes, return empty vector.
        return vec![];
    }
    else if m == 1 {
        // If there is only one unmatched index, return a
        // vector of the smallest in `g_hash`.
        let x = *g_indexes.iter().min().unwrap();
        return vec![x];
    }
    else if m == g_indexes.len() {
        // If the number of unmatched indexes matches the
        // size of `g_hash`, return all of `g_hash` in a
        // vector.
        return g_indexes.iter().map(|x| *x).collect::<Vec<_>>();
    }
    else {
        let mut tmp = g_indexes.iter().map(|x| *x).collect::<Vec<_>>();
        tmp.sort();
        return tmp[0..m].to_vec();
    }
}

pub fn wordle_score (correct: &str, guess: &str) -> String {
    let mut result = vec!['.'; guess.len()];

    // Get all the letters in common between `correct` and `guess`
    let set1: HashSet<_> = correct.chars().collect();
    let set2: HashSet<_> = guess.chars().collect();
    let shared_keys: HashSet<_> = set1.intersection(&set2).collect();

    if shared_keys.len() != 0 {
        // Get indexes of letters in common
        let mut c_hash: HashIndex = hash_index(correct, &shared_keys);
        let mut g_hash: HashIndex = hash_index(guess, &shared_keys);

        for key in shared_keys.iter() {
            let shared = shared_indexes(&c_hash, &g_hash, &key); 
            for index in &shared {
                if let Some(c) = c_hash.get_mut(key) {
                    c.remove(&index);
                }
                if let Some(g) = g_hash.get_mut(key) {
                    g.remove(&index);
                }
                result[*index] = 'X';
            }

            for index in get_unmatched_items(&c_hash, &g_hash, &key) {
                result[index] = 'x';
            }
        }
    }
    String::from_iter(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mask18() {
        let result = wordle_score(&"eerie", &"sheep");
        assert_eq!(result, "..xx.".to_string());
    }

    #[test]
    fn mask17() {
        let result = wordle_score(&"sheep", &"eerie");
        assert_eq!(result, "xx...".to_string());
    }

    #[test]
    fn mask16() {
        let result = wordle_score(&"arena", &"aorta");
        assert_eq!(result, "X.x.X".to_string());
    }

    #[test]
    fn mask15() {
        let result = wordle_score(&"aorta", &"arena");
        assert_eq!(result, "Xx..X".to_string());
    }

    #[test]
    fn mask14() {
        let result = wordle_score(&"abcde", &"fghij");
        assert_eq!(result, ".....".to_string());
    }

    #[test]
    fn mask13() {
        let result = wordle_score(&"abcde", &"aghij");
        assert_eq!(result, "X....".to_string());
    }

    #[test]
    fn mask12() {
        let result = wordle_score(&"abcde", &"fghia");
        assert_eq!(result, "....x".to_string());
    }

    #[test]
    fn mask11() {
        let result = wordle_score(&"crane", &"toils");
        assert_eq!(result, ".....".to_string());
    }

    #[test]
    fn mask10() {
        let result = wordle_score(&"crane", &"slate");
        assert_eq!(result, "..X.X".to_string());
    }

    #[test]
    fn mask09() {
        let result = wordle_score(&"crane", &"crete");
        assert_eq!(result, "XX..X".to_string());
    }

    #[test]
    fn mask08() {
        let result = wordle_score(&"crane", &"nacre");
        assert_eq!(result, "xxxxX".to_string());
    }

    #[test]
    fn mask07() {
        let result = wordle_score(&"tepee", &"beset");
        assert_eq!(result, ".X.Xx".to_string());
    }

    #[test]
    fn mask06() {
        let result = wordle_score(&"beset", &"tepee");
        assert_eq!(result, "xX.X.".to_string());
    }

    #[test]
    fn mask05() {
        let result = wordle_score(&"erect", &"tepee");
        assert_eq!(result, "xx.x.".to_string());
    }

    #[test]
    fn mask04() {
        let result = wordle_score(&"tepee", &"erect");
        assert_eq!(result, "x.x.x".to_string());
    }

    #[test]
    fn mask03() {
        let result = wordle_score(&"erase", &"chase");
        assert_eq!(result, "..XXX".to_string());
    }

    #[test]
    fn mask02() {
        let result = wordle_score(&"eerie", &"deter");
        assert_eq!(result, ".X.xx".to_string());
    }

    #[test]
    fn mask01() {
        let result = wordle_score(&"eerie", &"chase");
        assert_eq!(result, "....X".to_string());
    }
}

