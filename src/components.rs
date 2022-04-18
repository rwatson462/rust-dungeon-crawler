pub use crate::prelude::*;

#[derive(Clone,Copy,Debug,PartialEq)]
pub struct Render {
    pub colour: ColorPair,
    pub glyph: FontCharType
}

#[derive(Clone,Copy,Debug,PartialEq)]
pub struct Player;

#[derive(Clone,Copy,Debug,PartialEq)]
pub struct Monster;

#[derive(Clone,Copy,Debug,PartialEq)]
pub struct MovingRandomly;

#[derive(Clone,Copy,Debug,PartialEq)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Point
}
