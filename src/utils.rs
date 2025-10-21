pub fn normalize_args(raw_args: Vec<String>) -> Vec<String> {
    raw_args
        .iter()
        .skip(1)
        .fold(Vec::<String>::new(), |mut acc, current| {
            let is_short_flag = current.starts_with("-") && !current.starts_with("--");
            if is_short_flag {
                if current.contains("=") {
                    let split_flag: Vec<String> =
                        current.splitn(2, '=').map(|s| s.to_string()).collect();
                    let short_named = split_flag[0].chars().skip(1).map(|c| format!("-{}", c));
                    acc.append(&mut short_named.collect());
                    acc.append(&mut split_flag[1..].to_vec());
                } else {
                    let short_named = current.chars().skip(1).map(|c| format!("-{}", c));
                    acc.append(&mut short_named.collect());
                }
            } else if current.starts_with("-") && current.contains("=") {
                let mut split_flag: Vec<String> =
                    current.splitn(2, "=").map(|s| s.to_string()).collect();
                acc.append(&mut split_flag);
            } else {
                acc.push(current.to_string());
            }

            acc
        })
}
