#![cfg_attr(not(feature = "std"), no_std)]

#[repr(u16)]
pub enum NftExtensions {
    CreateCollection = 1,
    CreateItem = 2,
}

impl TryFrom<u16> for NftExtensions {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, ()> {
        match value {
            1 => Ok(Self::CreateCollection),
            2 => Ok(Self::CreateItem),
            _ => Err(()),
        }
    }
}
