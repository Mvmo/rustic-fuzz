pub fn fuzzy_sort<T: AsRef<str> + Clone>(to_sort: &[T], input: &str) -> Vec<T> {
    let mut clone: Vec<T> = to_sort.to_vec();
    let input_chars: Vec<char> = input.chars().collect();
    clone.sort_by_key(|s| levenshtein_distance(s.as_ref(), &input_chars));

    clone
}

pub fn fuzzy_sort_in_place<T: AsRef<str>>(to_sort: &mut [T], input: &str) {
    let input_chars: Vec<char> = input.chars().collect();
    to_sort.sort_by_key(|s| levenshtein_distance(s.as_ref(), &input_chars));
}

fn levenshtein_distance(u: &str, v_chars: &[char]) -> u32 {
    let u_chars: Vec<char> = u.chars().collect();

    let m = u.len() + 1;
    let n = v_chars.len() + 1;

    let mut d = vec![vec![0; n]; m];

    for i in 1..m {
        d[i][0] = i;
        for j in 1..n {
            d[0][j] = j;
            let mut replacement_score = d[i - 1][j - 1];
            if u_chars[i - 1] != v_chars[j - 1] {
                replacement_score += 1;
            }

            let insert_score = d[i][j - 1] + 1;
            let delete_score = d[i - 1][j] + 1;

            d[i][j] = replacement_score.min(insert_score).min(delete_score);
        }
    }

    return d[m - 1][n - 1] as u32;
}

#[test]
fn fuzzy_sort_test() {
    let test_cases = vec![
        (
            vec!["xxx".to_string(), "yyy".to_string(), "xx".to_string()],
            "xxx",
            vec!["xxx".to_string(), "xx".to_string(), "yyy".to_string()]
        ),
        (
            vec!["apple".to_string(), "banana".to_string(), "cherry".to_string()],
            "banana",
            vec!["banana".to_string(), "apple".to_string(), "cherry".to_string()]
        ),
        (
            vec!["rust".to_string(), "is".to_string(), "awesome".to_string()],
            "awesome",
            vec!["awesome".to_string(), "rust".to_string(), "is".to_string()]
        ),
    ];

    test_cases.iter().for_each(|(arr, input, expected_sorted)| {
        let sorted = fuzzy_sort(&arr, input);
        assert_eq!(sorted, *expected_sorted);

        let mut mutarr = arr.clone();
        fuzzy_sort_in_place(mutarr.as_mut(), input);
        assert_eq!(mutarr, *expected_sorted);
    });
}

#[test]
fn levenshtein_distance_test() {
    vec![
        ("hello, world", "hello- world", 1),
        ("murice", "maurice", 1),
        ("delete", "insert", 5),
        ("Tier", "Tor", 2),
        ("banana", "cherry", 6)
    ].iter()
    .for_each(|(u, v, expected_distance)| {
        let distance = levenshtein_distance(u, &v.chars().collect::<Vec<_>>());
        assert_eq!(distance, *expected_distance as u32);
    });
}
