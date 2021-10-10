pub trait CoordinateSystem<const N : usize> {
    type Point;
    fn origin(&self) -> Self::Point;

}

impl <const N : usize> CoordinateSystem<N> for CartesianCoordinates<N> {
    type Point = CartesianPoint<N>;

    fn origin(&self) -> CartesianPoint<N> {
        let components = [0.0 ; N];
        CartesianPoint { components }
    }
}

pub struct CartesianCoordinates<const N : usize> { }

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct CartesianPoint<const N : usize> {
    components: [f64 ; N]
}

impl<const N : usize> CartesianPoint<N> {
    pub fn components(&self) -> &[f64; N] {
        &self.components
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn origin_returns_all_zeros() {
        let coords = CartesianCoordinates::<2> { };
        let o = coords.origin();
        assert_eq!([0.0 as f64 ; 2], *o.components());
    }
}