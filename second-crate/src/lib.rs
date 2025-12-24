use trait_crate::{Associated, Get};

pub struct Second;
pub struct AnotherMarker;

impl Get<AnotherMarker> for Second {}
impl Get<<first_crate::First as Associated>::Item> for Second {}
