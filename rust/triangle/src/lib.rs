#[derive(PartialEq)]
pub struct Triangle {
    sides: Vec<u64>,
    class: TriangleClass
}

#[derive(PartialEq)]
enum TriangleClass {
    Equilateral,
    Isosceles,
    Scalene,
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        let mut sides = sides.to_vec();
        sides.sort();

        if sides.contains(&0) || sides[0] + sides[1] <= sides[2] {
            return None; 
        }

        let class = if sides[0] == sides[1] && sides[0] == sides[2] {
            TriangleClass::Equilateral
        } else if sides[0] == sides[1] || sides[0] == sides[2] || sides[1] == sides[2] {
            TriangleClass::Isosceles
        } else {
            TriangleClass::Scalene
        };

        Some(Triangle { sides, class })
    }

    pub fn is_equilateral(&self) -> bool {
        self.class == TriangleClass::Equilateral
    }

    pub fn is_scalene(&self) -> bool {
        self.class == TriangleClass::Scalene
    }

    pub fn is_isosceles(&self) -> bool {
        self.class == TriangleClass::Isosceles
    }
}
