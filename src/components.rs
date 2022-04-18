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