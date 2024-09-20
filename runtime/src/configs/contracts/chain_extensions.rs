use codec::Encode;
use core::marker::PhantomData;
use frame_support::traits::tokens::nonfungibles_v2::{Create, Mutate};
use my_chain_extensions::NftExtensions;
use pallet_contracts::chain_extension::*;
use sp_runtime::DispatchError;

pub struct ChainExtensions<T>(PhantomData<T>);

impl<T> Default for ChainExtensions<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
type CollectionIdOf<T> = <T as pallet_nfts::Config>::CollectionId;
type ItemIdOf<T> = <T as pallet_nfts::Config>::ItemId;

impl<T: pallet_contracts::Config + pallet_nfts::Config> ChainExtension<T> for ChainExtensions<T> {
    fn call<E: Ext<T = T>>(&mut self, env: Environment<E, InitState>) -> Result<RetVal> {
        let mut env = env.buf_in_buf_out();
        let origin = env.ext().caller().account_id()?.clone();

        match env
            .func_id()
            .try_into()
            .map_err(|_| DispatchError::Other("Cannot convert func_id"))?
        {
            NftExtensions::CreateCollection => {
                let collection_id = pallet_nfts::Pallet::<T>::create_collection(
                    &origin,
                    &origin,
                    &pallet_nfts::CollectionConfig {
                        settings: Default::default(),
                        max_supply: None,
                        mint_settings: pallet_nfts::MintSettings {
                            mint_type: pallet_nfts::MintType::<CollectionIdOf<T>>::Issuer,
                            ..Default::default()
                        },
                    },
                )?;
                env.write(&collection_id.encode(), false, None)?;
            }
            NftExtensions::CreateItem => {
                let (collection_id, item_id, who): (
                    CollectionIdOf<T>,
                    ItemIdOf<T>,
                    AccountIdOf<T>,
                ) = env.read_as()?;
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
