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
fn levenshtein_distance_test() {
    vec![
        ("hello, world", "hello- world", 1),
        ("murice", "maurice", 1),
        ("delete", "insert", 5),
        ("Tier", "Tor", 2)
    ].iter()
    .for_each(|(u, v, expected_distance)| {
        let distance = levenshtein_distance(u, v);
        assert_eq!(distance, *expected_distance as u32);
    });
}
