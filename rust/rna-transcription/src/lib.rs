#[derive(Debug, PartialEq)]
pub struct DNA {
    strand: String
}

#[derive(Debug, PartialEq)]
pub struct RNA {
    strand: String
}

impl DNA {
    pub fn new(dna: &str) -> Result<DNA, usize> {
        let mut strand = String::new();

        for (i, c) in dna.chars().enumerate() {
            match c {
                'A' | 'C' | 'G' | 'T' => strand.push(c),
                _ => return Err(i),
            }
        }

        Ok(DNA { strand })
    }

    pub fn to_rna(self) -> RNA {
        let mut strand = String::new();

        for c in self.strand.chars() {
            match c {
                'G' => strand.push('C'),
                'C' => strand.push('G'),
                'T' => strand.push('A'),
                'A' => strand.push('U'),
                _ => strand.push(c),
            }
        }

        RNA::new(&strand).unwrap()
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
        let mut strand = String::new();

        for (i, c) in rna.chars().enumerate() {
            match c {
                'A' | 'C' | 'G' | 'U' => strand.push(c),
                _ => return Err(i),
            }
        }

        Ok(RNA { strand })
    }
}
