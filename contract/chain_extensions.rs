use ink::primitives::AccountId;

pub type CollectionId = u32;
pub type ItemId = u32;
pub type NftErrorCode = u32;

#[ink::chain_extension(extension = 1)]
pub trait NftChainExtension {
    type ErrorCode = NftError;

    #[ink(function = 1, handle_status = true)]
    fn create_collection() -> Result<CollectionId, NftError>;

    #[ink(function = 2, handle_status = true)]
    fn create_item(
        collection_id: CollectionId,
        item_id: ItemId,
        who: AccountId,
    ) -> Result<ItemId, NftError>;
}

#[ink::scale_derive(Encode, Decode, TypeInfo)]
pub enum NftError {
    CannotCreateCollection = 1,
    CannotCreateItem = 2,
    Other = 3,
}
impl From<scale::Error> for NftError {
    fn from(_: scale::Error) -> Self {
        panic!("encountered unexpected invalid SCALE encoding")
    }
}

impl Into<NftErrorCode> for NftError {
    fn into(self) -> NftErrorCode {
        match self {
            NftError::CannotCreateCollection => 1,
            NftError::CannotCreateItem => 2,
            NftError::Other => 3,
        }
    }
}

impl From<NftErrorCode> for NftError {
    fn from(error_code: NftErrorCode) -> Self {
        match error_code {
            1 => Self::CannotCreateCollection,
            2 => Self::CannotCreateItem,
            _ => Self::Other,
        }
    }
}

impl ink::env::chain_extension::FromStatusCode for NftError {
    fn from_status_code(status_code: u32) -> Result<(), Self> {
        if status_code == 0 {
            return Ok(());
        }
        Err(status_code.into())
    }
}
