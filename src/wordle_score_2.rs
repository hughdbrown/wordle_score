pub fn wordle_score_2 (correct: &str, guess: &str) -> String {
    let mut result = vec!['.'; guess.len()];
    let mut correct_chars = correct.chars().collect::<Vec<char>>();

    for (i, c) in guess.chars().enumerate() {
        if c == correct_chars[i] {
            result[i] = 'X';
            correct_chars[i] = '-';
        }
    }
    
    // Second pass for letters in the guess that are correct
    // but not in the right place
    for (i, (c, g)) in correct.chars().zip(guess.chars()).enumerate() {
        if c != g {
            if let Some(j) = correct_chars.iter().position(|&x| x == g) {
                result[i] = 'x';
                correct_chars[j] = '-';
            }
        }
    }

    String::from_iter(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mask18_2() {
        let result = wordle_score_2(&"eerie", &"sheep");
        assert_eq!(result, "..xx.".to_string());
    }

    #[test]
    fn mask17_2() {
        let result = wordle_score_2(&"sheep", &"eerie");
        assert_eq!(result, "xx...".to_string());
    }

    #[test]
    fn mask16_2() {
        let result = wordle_score_2(&"arena", &"aorta");
        assert_eq!(result, "X.x.X".to_string());
    }

    #[test]
    fn mask15_2() {
        let result = wordle_score_2(&"aorta", &"arena");
        assert_eq!(result, "Xx..X".to_string());
    }

    #[test]
    fn mask14_2() {
        let result = wordle_score_2(&"abcde", &"fghij");
        assert_eq!(result, ".....".to_string());
    }

    #[test]
    fn mask13_2() {
        let result = wordle_score_2(&"abcde", &"aghij");
        assert_eq!(result, "X....".to_string());
    }

    #[test]
    fn mask12_2() {
        let result = wordle_score_2(&"abcde", &"fghia");
        assert_eq!(result, "....x".to_string());
    }

    #[test]
    fn mask11_2() {
        let result = wordle_score_2(&"crane", &"toils");
        assert_eq!(result, ".....".to_string());
    }

    #[test]
    fn mask10_2() {
        let result = wordle_score_2(&"crane", &"slate");
        assert_eq!(result, "..X.X".to_string());
    }

    #[test]
    fn mask09_2() {
        let result = wordle_score_2(&"crane", &"crete");
        assert_eq!(result, "XX..X".to_string());
    }

    #[test]
    fn mask08_2() {
        let result = wordle_score_2(&"crane", &"nacre");
        assert_eq!(result, "xxxxX".to_string());
    }

    #[test]
    fn mask07_2() {
        let result = wordle_score_2(&"tepee", &"beset");
        assert_eq!(result, ".X.Xx".to_string());
    }

    #[test]
    fn mask06_2() {
        let result = wordle_score_2(&"beset", &"tepee");
        assert_eq!(result, "xX.X.".to_string());
    }

    #[test]
    fn mask05_2() {
        let result = wordle_score_2(&"erect", &"tepee");
        assert_eq!(result, "xx.x.".to_string());
    }

    #[test]
    fn mask04_2() {
        let result = wordle_score_2(&"tepee", &"erect");
        assert_eq!(result, "x.x.x".to_string());
    }

    #[test]
    fn mask03_2() {
        let result = wordle_score_2(&"erase", &"chase");
        assert_eq!(result, "..XXX".to_string());
    }

    #[test]
    fn mask02_2() {
        let result = wordle_score_2(&"eerie", &"deter");
        assert_eq!(result, ".X.xx".to_string());
    }

    #[test]
    fn mask01_2() {
        let result = wordle_score_2(&"eerie", &"chase");
        assert_eq!(result, "....X".to_string());
    }
}
