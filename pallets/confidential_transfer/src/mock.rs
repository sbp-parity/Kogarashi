use crate::{self as confidential_transfer, pallet::Config};

use zero_circuits::{ConfidentialTransferCircuit, ConfidentialTransferTransaction};

use rand_core::RngCore;
use zero_crypto::behave::Group;
use zero_elgamal::EncryptedNumber;
use zero_jubjub::{Fp as JubJubScalar, JubJubAffine, GENERATOR_EXTENDED};

use pallet_encrypted_balance::{Account, AccountData};

use frame_support::traits::StorageMapShim;
use frame_support::{construct_runtime, parameter_types};
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<TestRuntime>;
type Block = frame_system::mocking::MockBlock<TestRuntime>;

construct_runtime!(
    pub enum TestRuntime where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Module, Call, Config, Storage, Event<T>},
        Plonk: pallet_plonk::{Module, Call, Storage, Event<T>},
        EncryptedBalance: pallet_encrypted_balance::{Module, Call, Storage, Config<T>, Event<T>},
        ConfidentialTransfer: confidential_transfer::{Module, Call, Storage, Event<T>},
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub BlockWeights: frame_system::limits::BlockWeights =
        frame_system::limits::BlockWeights::simple_max(1024);
}

impl frame_system::Config for TestRuntime {
    type BaseCallFilter = ();
    type BlockWeights = ();
    type BlockLength = ();
    type Origin = Origin;
    type Index = u64;
    type Call = Call;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = Event;
    type BlockHashCount = BlockHashCount;
    type DbWeight = ();
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
}

impl pallet_plonk::Config for TestRuntime {
    type CustomCircuit = ConfidentialTransferCircuit;
    type Event = Event;
}

impl pallet_encrypted_balance::Config for TestRuntime {
    type EncryptedBalance = EncryptedNumber;
    type Event = Event;
    type AccountStore = StorageMapShim<
        Account<TestRuntime>,
        frame_system::Provider<TestRuntime>,
        u64,
        AccountData<Self::EncryptedBalance>,
    >;
    type WeightInfo = ();
}

impl Config for TestRuntime {
    type Event = Event;
}

// This function basically just builds a genesis storage key/value store according to
// our desired mockup.
pub(crate) fn new_test_ext() -> sp_io::TestExternalities {
    frame_system::GenesisConfig::default()
        .build_storage::<TestRuntime>()
        .unwrap()
        .into()
}

pub(crate) fn generate_confidential_transfer_params(
    mut rng: impl RngCore,
) -> (
    ConfidentialTransferCircuit,
    ConfidentialTransferTransaction<EncryptedNumber>,
) {
    let generator = GENERATOR_EXTENDED;
    let alice_private_key = JubJubScalar::random(&mut rng);
    let bob_private_key = JubJubScalar::random(&mut rng);
    let alice_public_key = generator * alice_private_key;
    let bob_public_key = generator * bob_private_key;

    let alice_balance = JubJubScalar::from(1500 as u64);
    let transfer_amount = JubJubScalar::from(800 as u64);
    let alice_after_balance = JubJubScalar::from(700 as u64);
    let alice_original_randomness = JubJubScalar::from(789 as u64);
    let randomness = JubJubScalar::from(123 as u64);

    let alice_t_encrypted_balance =
        (generator * alice_balance) + (alice_public_key * alice_original_randomness);
    let alice_s_encrypted_balance = generator * alice_original_randomness;
    let alice_t_encrypted_transfer_amount =
        (generator * transfer_amount) + (alice_public_key * randomness);
    let alice_s_encrypted_transfer_amount = generator * randomness;
    let bob_encrypted_transfer_amount =
        (generator * transfer_amount) + (bob_public_key * randomness);
    let alice_public_key = JubJubAffine::from(alice_public_key);
    let bob_public_key = JubJubAffine::from(bob_public_key);
    let alice_t_encrypted_balance = JubJubAffine::from(alice_t_encrypted_balance);
    let alice_s_encrypted_balance = JubJubAffine::from(alice_s_encrypted_balance);
    let alice_t_encrypted_transfer_amount = JubJubAffine::from(alice_t_encrypted_transfer_amount);
    let alice_s_encrypted_transfer_amount = JubJubAffine::from(alice_s_encrypted_transfer_amount);
    let alice_encrypted_transfer_amount = EncryptedNumber::new(
        alice_t_encrypted_transfer_amount,
        alice_s_encrypted_transfer_amount,
    );
    let bob_encrypted_transfer_amount = JubJubAffine::from(bob_encrypted_transfer_amount);

    (
        ConfidentialTransferCircuit::new(
            alice_public_key,
            bob_public_key,
            EncryptedNumber::new(alice_t_encrypted_balance, alice_s_encrypted_balance),
            alice_encrypted_transfer_amount,
            bob_encrypted_transfer_amount,
            alice_private_key,
            transfer_amount,
            alice_after_balance,
            randomness,
        ),
        ConfidentialTransferTransaction::new(
            alice_public_key,
            bob_public_key,
            alice_encrypted_transfer_amount,
            bob_encrypted_transfer_amount,
        ),
    )
}
