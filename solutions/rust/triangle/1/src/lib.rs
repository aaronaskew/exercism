#[derive(PartialEq)]
enum TriangleType {
    Equilateral,
    Scalene,
    Isosceles,
}

pub struct Triangle(TriangleType);

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        if sides[0] == 0
            || sides[1] == 0
            || sides[2] == 0
            || sides[0] + sides[1] < sides[2]
            || sides[0] + sides[2] < sides[1]
            || sides[1] + sides[2] < sides[0]
        {
            None
        } else {
            match (sides[0], sides[1], sides[2]) {
                (a, b, c) if a == b && b == c => Some(Triangle(TriangleType::Equilateral)),
                (a, b, c) if a == b || b == c || a == c => Some(Triangle(TriangleType::Isosceles)),
                _ => Some(Triangle(TriangleType::Scalene)),
            }
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.0 == TriangleType::Equilateral
    }

    pub fn is_scalene(&self) -> bool {
        self.0 == TriangleType::Scalene
    }

    pub fn is_isosceles(&self) -> bool {
        matches!(self.0, TriangleType::Isosceles | TriangleType::Equilateral)
    }
}
