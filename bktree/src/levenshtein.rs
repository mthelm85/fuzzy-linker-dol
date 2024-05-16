pub fn levenshtein_distance(s: &str, t: &str) -> u32 {
    let s_chars: Vec<_> = s.chars().collect();
    let t_chars: Vec<_> = t.chars().collect();
    let (n, m) = (s_chars.len() as u32, t_chars.len() as u32);

    if n == 0 {
        return m;
    }
    if m == 0 {
        return n;
    }

    let mut current_row: Vec<u32> = (0..=m).collect();
    let mut previous_row: Vec<u32> = vec![0; (m + 1) as usize];

    for (i, s_ch) in s_chars.iter().enumerate() {
        previous_row.copy_from_slice(&current_row);
        current_row[0] = i as u32 + 1;

        for (j, t_ch) in t_chars.iter().enumerate() {
            let cost = if s_ch == t_ch { 0 } else { 1 };
            current_row[(j + 1) as usize] = (previous_row[j as usize] + cost)
                .min(previous_row[(j + 1) as usize] + 1)
                .min(current_row[j as usize] + 1);
        }
    }

    current_row[m as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_levenshtein_distance() {
        assert_eq!(levenshtein_distance("kitten", "sitting"), 3);
        assert_eq!(levenshtein_distance("book", "back"), 2);
        assert_eq!(levenshtein_distance("rust", "rust"), 0);
        assert_eq!(levenshtein_distance("rust", ""), 4);
        assert_eq!(levenshtein_distance("", "rust"), 4);
    }
}