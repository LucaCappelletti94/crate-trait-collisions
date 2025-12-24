pub struct First;

impl trait_crate::Associated for First {
    type Item = u32;
}

impl trait_crate::Get<u32> for First {}
impl trait_crate::Get<i32> for First {}
