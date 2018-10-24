pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let n = minefield.len();
    let mut v = vec![String::new(); n];
    
    for (i, row) in minefield.iter().enumerate() {
        for (j, c) in row.chars().enumerate() {
            match c {
                '*' => v[i].push('*'),
                _ => {
                    let mut count = 0;
                    let m = row.len() - 1;

                    // check row above
                    if i > 0 {
                        if j > 0 && minefield[i-1].chars().nth(j-1) == Some('*') { count += 1; }
                        if minefield[i-1].chars().nth(j) == Some('*') { count += 1; }
                        if j < m && minefield[i-1].chars().nth(j+1) == Some('*') { count += 1; }
                    }
                    
                    // check current row
                    if j > 0 && minefield[i].chars().nth(j-1) == Some('*') { count += 1; }
                    if j < m && minefield[i].chars().nth(j+1) == Some('*') { count += 1; }

                    // check row below
                    if i < n-1 {
                        if j > 0 && minefield[i+1].chars().nth(j-1) == Some('*') { count += 1; }
                        if minefield[i+1].chars().nth(j) == Some('*') { count += 1; }
                        if j < m && minefield[i+1].chars().nth(j+1) == Some('*') { count += 1; }
                    }

                    if count == 0 {
                        v[i].push(' ');
                    } else {
                        v[i].push_str(&count.to_string());
                    }
                }
            }
        }
    }
    v
}
