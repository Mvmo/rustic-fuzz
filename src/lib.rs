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

    let m = u_chars.len();
    let n = v_chars.len();

    let mut prev_row: Vec<u32> = (0..=n as u32).collect();
    let mut current_row: Vec<u32> = vec![0; n + 1];

    for i in 1..=m {
        current_row[0] = i as u32;
        for j in 1..=n {
            let u_char = u_chars[i - 1];
            let v_char = v_chars[j - 1];

            let replacement_score = prev_row[j - 1] + (
                if u_char == v_char { 0 } else { 1 }
            );

            let insert_score = current_row[j - 1] + 1;
            let delete_score = prev_row[j] + 1;

            current_row[j] = replacement_score
                .min(insert_score)
                .min(delete_score)
        }

        std::mem::swap(&mut prev_row, &mut current_row);
    }

    return prev_row[n];
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
