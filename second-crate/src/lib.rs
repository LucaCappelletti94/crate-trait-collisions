use trait_crate::{Associated, Get};

pub struct Second;

impl Associated for Second {
    type Item = u32;
}

impl Get<i32> for Second {}
impl Get<<first_crate::First as Associated>::Item> for Second {}
