fn fuzzy_sort(to_sort: Vec<String>, input: &str) -> Vec<String> {
    let mut clone = to_sort.clone();
    clone.sort_by_key(|s| levenshtein_distance(s, input));

    clone
}

fn fuzzy_sort_in_place(to_sort: &mut Vec<String>, input: &str) {
    to_sort.sort_by_key(|s| levenshtein_distance(s, input));
}

fn levenshtein_distance(u: &str, v: &str) -> u32 {
    let u_chars: Vec<char> = u.chars().collect();
    let v_chars: Vec<char> = v.chars().collect();

    let m = u.len() + 1;
    let n = v.len() + 1;

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

            d[i][j] = *vec![replacement_score, insert_score, delete_score].iter()
                .min()
                .unwrap();
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
        let sorted = fuzzy_sort(arr.clone(), input);
        assert_eq!(sorted, *expected_sorted);
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
        let distance = levenshtein_distance(u, v);
        assert_eq!(distance, *expected_distance as u32);
    });
}
