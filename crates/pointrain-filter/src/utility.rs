use pointrain_core::types::Position;

pub fn is_finite(p: &Position) -> bool {
    p.x.is_finite() && p.y.is_finite() && p.z.is_finite()
}
