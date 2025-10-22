use dioxus::prelude::*;
use strum_macros::EnumIter;


#[derive(Clone, Copy, PartialEq, Eq, Debug, EnumIter)]
pub enum Mark {
    Correct, Wrong, Repeat
}

#[extension(pub trait OptionMarkExt)]
impl Option<Mark> {
    fn image_asset(&self) -> Asset {
        match self {
            Some(Mark::Correct) => {
                asset!("/assets/images/fa-check.svg")
            }
            Some(Mark::Wrong) => {
                asset!("/assets/images/fa-xmark.svg")
            }
            Some(Mark::Repeat) => {
                asset!("/assets/images/fa-repeat.svg")
            }
            None => {
                asset!("/assets/images/blank.svg")
            }
        }
    }
}