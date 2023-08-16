use std::io::stdin;

fn main() {
    let items: Vec<String> = stdin().lines()
        .filter_map(|line| line.ok())
        .collect();

    let distance = levenshtein_distance("hallo wxlt".to_string(), "hallo welt".to_string());
    println!("{distance}");
}

fn levenshtein_distance(u: String, v: String) -> u32 {
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
