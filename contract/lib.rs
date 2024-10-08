#![cfg_attr(not(feature = "std"), no_std, no_main)]

mod chain_extensions;
mod environment;

#[ink::contract(env = ChainEnvironment)]
mod contract {
    use crate::chain_extensions::{CollectionId, ItemId};
    use crate::environment::ChainEnvironment;

    #[ink(event)]
    pub struct NftAirdropped {
        collection_id: CollectionId,
        item_id: ItemId,
        who: AccountId,
    }

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[derive(Default)]
    #[ink(storage)]
    pub struct Contract {
        /// Stores a single `bool` value on the storage.
        collection_id: Option<CollectionId>,
        item_id: ItemId,
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Default::default()
        }

        #[ink(message)]
        pub fn initialize_collection(&mut self) -> Result<(), u32> {
            if let Some(_) = self.collection_id {
                return Ok(());
            }

            let collection_id = self
                .env()
                .extension()
                .create_collection()
                .map_err(|_| 1u32)?;
            self.collection_id = Some(collection_id);
            Ok(())
        }

        #[ink(message)]
        pub fn airdrop_nft(&mut self) -> Result<(), u32> {
            let who = self.env().caller();
            let collection_id = self.collection_id.ok_or(1u32)?;
            self.item_id = self.item_id + 1;

            self.env()
                .extension()
                .create_item(collection_id, self.item_id, who)
                .map_err(|_| 2u32)?;

            self.env().emit_event(NftAirdropped {
                collection_id,
                item_id: self.item_id,
                who,
            });

            Ok(())
        }
    }

    // /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    // /// module and test functions are marked with a `#[test]` attribute.
    // /// The below code is technically just normal Rust code.
    // #[cfg(test)]
    // mod tests {
    //     /// Imports all the definitions from the outer scope so we can use them here.
    //     use super::*;

    //     /// We test if the default constructor does its job.
    //     #[ink::test]
    //     fn default_works() {
    //         let contract = Contract::default();
    //         assert_eq!(contract.get(), false);
    //     }

    //     /// We test a simple use case of our contract.
    //     #[ink::test]
    //     fn it_works() {
    //         let mut contract = Contract::new(false);
    //         assert_eq!(contract.get(), false);
    //         contract.flip();
    //         assert_eq!(contract.get(), true);
    //     }
    // }

    /// This is how you'd write end-to-end (E2E) or integration tests for ink! contracts.
    ///
    /// When running these you need to make sure that you:
    /// - Compile the tests with the `e2e-tests` feature flag enabled (`--features e2e-tests`)
    /// - Are running a Substrate node which contains `pallet-contracts` in the background
    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// A helper function used for calling contract messages.
        use ink_e2e::build_message;

        /// The End-to-End test `Result` type.
        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        /// We test that we can upload and instantiate the contract using its default constructor.
        #[ink_e2e::test]
        async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = ContractRef::default();

            // When
            let contract_account_id = client
                .instantiate("contract", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // Then
            let get = build_message::<ContractRef>(contract_account_id.clone())
                .call(|contract| contract.get());
            let get_result = client.call_dry_run(&ink_e2e::alice(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), false));

            Ok(())
        }

        /// We test that we can read and write a value from the on-chain contract contract.
        #[ink_e2e::test]
        async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = ContractRef::new(false);
            let contract_account_id = client
                .instantiate("contract", &ink_e2e::bob(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let get = build_message::<ContractRef>(contract_account_id.clone())
                .call(|contract| contract.get());
            let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), false));

            // When
            let flip = build_message::<ContractRef>(contract_account_id.clone())
                .call(|contract| contract.flip());
            let _flip_result = client
                .call(&ink_e2e::bob(), flip, 0, None)
                .await
                .expect("flip failed");

            // Then
            let get = build_message::<ContractRef>(contract_account_id.clone())
                .call(|contract| contract.get());
            let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), true));

            Ok(())
        }
    }
}
