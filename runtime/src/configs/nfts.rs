use frame_support::{parameter_types, traits::AsEnsureOriginWithArg};
use frame_system::{EnsureRoot, EnsureSigned};
use pallet_nfts::PalletFeatures;
use sp_core::ConstU32;
use sp_runtime::traits::Verify;

use crate::{
    contracts_config::deposit, AccountId, Balance, Balances, BlockNumber, Runtime, RuntimeEvent,
    Signature, DAYS, UNIT,
};

parameter_types! {
    pub NftsPalletFeatures: PalletFeatures = PalletFeatures::all_enabled();
    pub const NftsMaxDeadlineDuration: BlockNumber = 12 * 30 * DAYS;
    pub const NftsCollectionDeposit: Balance = 0;
    pub const NftsItemDeposit: Balance = 0;
    pub const NftsMetadataDepositBase: Balance = 0;
    pub const NftsAttributeDepositBase: Balance = 0;
    pub const NftsDepositPerByte: Balance = 0;
}

impl pallet_nfts::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;

    type CollectionId = u32;
    type ItemId = u32;

    type Currency = Balances;
    type ForceOrigin = EnsureRoot<AccountId>;
    type CreateOrigin = AsEnsureOriginWithArg<EnsureSigned<AccountId>>;
    type Locker = ();

    type CollectionDeposit = NftsCollectionDeposit;
    type ItemDeposit = NftsItemDeposit;
    type MetadataDepositBase = NftsMetadataDepositBase;
    type AttributeDepositBase = NftsAttributeDepositBase;
    type DepositPerByte = NftsDepositPerByte;

    type StringLimit = ConstU32<256>;
    type KeyLimit = ConstU32<64>;
    type ValueLimit = ConstU32<256>;
    type ApprovalsLimit = ConstU32<20>;
    type ItemAttributesApprovalsLimit = ConstU32<30>;
    type MaxTips = ConstU32<10>;
    type MaxDeadlineDuration = NftsMaxDeadlineDuration;
    type MaxAttributesPerCall = ConstU32<10>;
    type Features = NftsPalletFeatures;

    type OffchainSignature = Signature;
    type OffchainPublic = <Signature as Verify>::Signer;
    type WeightInfo = pallet_nfts::weights::WeightInfo<Runtime>;
}
