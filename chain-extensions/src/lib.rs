#![cfg_attr(not(feature = "std"), no_std)]

use pallet_contracts::chain_extension::{BufInBufOutState, Environment, Ext};
use sp_runtime::DispatchError;

#[repr(u16)]
pub enum NftExtensions<T: pallet_nfts::Config<I>, I: 'static = ()> {
    CreateCollection {
        origin: T::AccountId,
    },
    CreateItem {
        collection_id: T::CollectionId,
        item_id: T::ItemId,
        who: T::AccountId,
    },
}

impl<T, I, E> TryFrom<&mut Environment<'_, '_, E, BufInBufOutState>> for NftExtensions<T, I>
where
    T: pallet_nfts::Config<I> + pallet_contracts::Config,
    I: 'static,
    E: Ext<T = T>,
{
    type Error = DispatchError;

    fn try_from(env: &mut Environment<E, BufInBufOutState>) -> Result<Self, Self::Error> {
        match env.func_id() {
            1 => {
                let origin = env.ext().caller().account_id()?.clone();
                Ok(Self::CreateCollection { origin })
            }
            2 => {
                let (collection_id, item_id, who) = env.read_as()?;
                Ok(Self::CreateItem {
                    collection_id,
                    item_id,
                    who,
                })
            }
            func_id => {
                log::error!("Called an unregistered `func_id`: {:}", func_id);
                Err(DispatchError::Other("Unimplemented func_id"))
            }
        }
    }
}
