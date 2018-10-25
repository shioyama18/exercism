use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    match nucleotide {
        'A' | 'C' | 'G' | 'T' => { 
            if let Some(&n) = nucleotide_counts(dna)?.get(&nucleotide) {
                Ok(n)
            } else {
                Ok(0)
            }
        }
        x => Err(x)
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut map = "ACGT".chars().zip(std::iter::repeat(0)).collect::<HashMap<_, _>>();

    for c in dna.chars() {
        match c {
            'A' | 'C' | 'G' | 'T' => *map.entry(c).or_insert(0) += 1,
            x => return Err(x),
        }
    }
    
    Ok(map)
}
