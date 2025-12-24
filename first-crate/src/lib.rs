pub struct First;
pub struct MyMarker;

impl trait_crate::Associated for First {
    type Item = MyMarker;
}
