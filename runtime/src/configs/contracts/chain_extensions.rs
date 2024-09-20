use codec::Encode;
use core::marker::PhantomData;
use frame_support::traits::tokens::nonfungibles_v2::{Create, Mutate};
use log::trace;
use my_chain_extensions::NftExtensions;
use pallet_contracts::chain_extension::*;

pub struct ChainExtensions<T>(PhantomData<T>);

impl<T> Default for ChainExtensions<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T: pallet_contracts::Config + pallet_nfts::Config> ChainExtension<T> for ChainExtensions<T> {
    fn call<E: Ext<T = T>>(&mut self, env: Environment<E, InitState>) -> Result<RetVal> {
        let mut env = env.buf_in_buf_out();

        match NftExtensions::try_from(&mut env)? {
            NftExtensions::CreateCollection { origin } => {
                let collection_id = pallet_nfts::Pallet::<T>::create_collection(
                    &origin,
                    &origin,
                    &pallet_nfts::CollectionConfig {
                        settings: Default::default(),
                        max_supply: None,
                        mint_settings: pallet_nfts::MintSettings {
                            mint_type: pallet_nfts::MintType::<T::CollectionId>::Issuer,
                            ..Default::default()
                        },
                    },
                )?;
                env.write(&collection_id.encode(), false, None)?;
            }
            NftExtensions::CreateItem {
                collection_id,
                item_id,
                who,
            } => {
                trace!("Minting item ({collection_id:?}, {item_id:?}) for {who:?}");
                let item_id = pallet_nfts::Pallet::<T>::mint_into(
                    &collection_id,
                    &item_id,
                    &who,
                    &Default::default(),
                    true,
                )?;
                env.write(&item_id.encode(), false, None)?;
            }
        }

        Ok(RetVal::Converging(0))
    }
}
