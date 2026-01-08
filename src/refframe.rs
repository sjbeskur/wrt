use std::{
    marker::PhantomData,
    ops::{Add, Mul, Not},
};

pub trait FrameOfReference {
    fn name() -> &'static str;
}

pub struct Motion<TFromPose, TToPose>
where
    TFromPose: FrameOfReference,
    TToPose: FrameOfReference,
{
    _marker: PhantomData<(TFromPose, TToPose)>,
}

impl<TFromPose, TToPose> Motion<TFromPose, TToPose>
where
    TFromPose: FrameOfReference,
    TToPose: FrameOfReference,
{
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
    pub fn is_null(&self) -> bool {
        TFromPose::name() == TToPose::name()
    }
}

impl<TpFrom, TpTo> Not for &Motion<TpFrom, TpTo>
where
    TpFrom: FrameOfReference,
    TpTo: FrameOfReference,
{
    type Output = Motion<TpTo, TpFrom>;

    fn not(self) -> Self::Output {
        println!(
            "Inverting motion from {} to {}",
            TpFrom::name(),
            TpTo::name()
        );
        Motion::new()
    }
}

impl<TP1, TP2, TP3> Add<&Motion<TP2, TP3>> for &Motion<TP1, TP2>
where
    TP1: FrameOfReference,
    TP2: FrameOfReference,
    TP3: FrameOfReference,
{
    type Output = Motion<TP1, TP3>;

    fn add(self, _rhs: &Motion<TP2, TP3>) -> Self::Output {
        println!(
            "Moving motion from {} to {} via {}",
            TP1::name(),
            TP3::name(),
            TP2::name()
        );
        Motion::new()
    }
}

pub struct Position<Tref>
where
    Tref: FrameOfReference,
{
    p: [f32; 2],
    _marker: PhantomData<Tref>,
}

impl<Tref> Position<Tref>
where
    Tref: FrameOfReference,
{
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            p: [x, y],
            _marker: PhantomData,
        }
    }
}

impl<TrefFrom, TrefTo> Mul<&Position<TrefTo>> for &Motion<TrefFrom, TrefTo>
where
    TrefFrom: FrameOfReference,
    TrefTo: FrameOfReference,
{
    type Output = Position<TrefFrom>;

    fn mul(self, rhs: &Position<TrefTo>) -> Self::Output {
        println!(
            "Transforming position from {} to {}",
            TrefTo::name(),
            TrefFrom::name()
        );
        Position::new(rhs.p[0], rhs.p[1])
    }
}

mod tests {
    #[cfg(test)]
    use super::*;

    #[cfg(test)]
    struct FrameA;
    #[cfg(test)]
    impl FrameOfReference for FrameA {
        fn name() -> &'static str {
            "FrameA"
        }
    }

    #[cfg(test)]
    struct FrameB;
    #[cfg(test)]
    impl FrameOfReference for FrameB {
        fn name() -> &'static str {
            "FrameB"
        }
    }

    #[test]
    fn test_motion_compile_checks() {
        let motion_ab: Motion<FrameA, FrameB> = Motion::new();
        let motion_ba = !&motion_ab;
        let motion_aa = &motion_ab + &motion_ba;

        assert!(motion_aa.is_null());

        let pos_b = Position::<FrameB>::new(1.0, 2.0);
        let _pos_a = &motion_ab * &pos_b;
    }
}