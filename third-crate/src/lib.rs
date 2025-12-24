use trait_crate::{Associated, Get};

pub struct Third;
pub struct ThirdMarker;
pub struct Forth;
pub struct ForthMarker;

impl Associated for Third {
    type Item = ThirdMarker;
}

impl Get<ForthMarker> for Forth {}
impl Get<ThirdMarker> for Third {}
impl Get<<Third as Associated>::Item> for Forth {}