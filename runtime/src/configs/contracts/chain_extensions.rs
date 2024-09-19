use core::marker::PhantomData;
use pallet_contracts::chain_extension::*;

pub struct ChainExtensions<T>(PhantomData<T>);

impl<T> Default for ChainExtensions<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T: pallet_contracts::Config> ChainExtension<T> for ChainExtensions<T> {
    fn call<E: Ext<T = T>>(&mut self, _env: Environment<E, InitState>) -> Result<RetVal> {
        todo!()
    }
}
