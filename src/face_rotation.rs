use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum FaceRotation {
    R,
    R2,
    R2Prime,
    RPrime,
    F,
    F2,
    F2Prime,
    FPrime,
    D,
    D2,
    D2Prime,
    DPrime,
    L,
    L2,
    L2Prime,
    LPrime,
    B,
    B2,
    B2Prime,
    BPrime,
}

impl From<FaceRotation> for u8 {
    fn from(mv: FaceRotation) -> u8 {
        use FaceRotation::*;
        match mv {
            R => 0,
            R2 => 1,
            R2Prime => 1,
            RPrime => 2,
            F => 3,
            F2 => 4,
            F2Prime => 4,
            FPrime => 5,
            D => 6,
            D2 => 7,
            D2Prime => 7,
            DPrime => 8,
            L => 9,
            L2 => 10,
            L2Prime => 10,
            LPrime => 11,
            B => 12,
            B2 => 13,
            B2Prime => 13,
            BPrime => 14,
        }
    }
}

impl Display for FaceRotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use FaceRotation::*;
        let s = match self {
            R => "R",
            R2 => "R2",
            R2Prime => "R2'",
            RPrime => "R'",
            F => "F",
            F2 => "F2",
            F2Prime => "F2'",
            FPrime => "F'",
            D => "D",
            D2 => "D2",
            D2Prime => "D2'",
            DPrime => "D'",
            L => "L",
            L2 => "L2",
            L2Prime => "L2'",
            LPrime => "L'",
            B => "B",
            B2 => "B2",
            B2Prime => "B2'",
            BPrime => "B'",
        };
        write!(f, "{}", s)
    }
}
