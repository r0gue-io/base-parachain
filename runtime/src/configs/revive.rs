use frame_support::{
	parameter_types,
	traits::{ConstBool, ConstU32, ConstU64, Nothing},
};
use frame_system::EnsureSigned;

use crate::{
	Balance, Balances, Perbill, PolkadotXcm, Runtime, RuntimeCall, RuntimeEvent, RuntimeHoldReason,
	Timestamp, TransactionPayment, UNIT, MILLIUNIT
};

// 18 decimals
const ETH: u128 = 1_000_000_000_000_000_000;

/// Deposit rate for stored data. 1/100th of the Relay Chain's deposit rate. `items` is the
/// number of keys in storage and `bytes` is the size of the value.
pub const fn deposit(items: u32, bytes: u32) -> Balance {
	(items as Balance * UNIT + (bytes as Balance) * (5 * MILLIUNIT / 100)) / 10
}

parameter_types! {
	pub CodeHashLockupDepositPercent: Perbill = Perbill::from_percent(30);
	pub const NativeToEthRatio: u32 = (ETH/UNIT) as u32;
    pub const DepositPerItem: Balance = deposit(1, 0);
	pub const DepositPerByte: Balance = deposit(0, 1);
}

impl pallet_revive::Config for Runtime {
	type AddressMapper = pallet_revive::AccountId32Mapper<Self>;
	// No runtime dispatchables are callable from contracts.
	type CallFilter = Nothing;
	type ChainExtension = ();
	// EVM chain id. 3,395 is a unique ID still.
	type ChainId = ConstU64<3_395>;
	// 30 percent of storage deposit held for using a code hash.
	type CodeHashLockupDepositPercent = CodeHashLockupDepositPercent;
	type Currency = Balances;
	type DepositPerByte = DepositPerByte;
	type DepositPerItem = DepositPerItem;
	type EthGasEncoder = ();
	type FindAuthor = <Runtime as pallet_authorship::Config>::FindAuthor;
	type InstantiateOrigin = EnsureSigned<Self::AccountId>;
	// 1 ETH : 1_000_000 UNIT
	type NativeToEthRatio = NativeToEthRatio;
	// 512 MB. Used in an integrity test that verifies the runtime has enough memory.
	type PVFMemory = ConstU32<{ 512 * 1024 * 1024 }>;
	type RuntimeCall = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type RuntimeHoldReason = RuntimeHoldReason;
	// 128 MB. Used in an integrity that verifies the runtime has enough memory.
	type RuntimeMemory = ConstU32<{ 128 * 1024 * 1024 }>;
	type Time = Timestamp;
	// Disables access to unsafe host fns such as xcm_send.
	type UnsafeUnstableInterface = ConstBool<false>;
	type UploadOrigin = EnsureSigned<Self::AccountId>;
	type WeightInfo = pallet_revive::weights::SubstrateWeight<Self>;
	type WeightPrice = TransactionPayment;
	type Xcm = PolkadotXcm;
}

impl TryFrom<RuntimeCall> for pallet_revive::Call<Runtime> {
	type Error = ();

	fn try_from(value: RuntimeCall) -> Result<Self, Self::Error> {
		match value {
			RuntimeCall::Revive(call) => Ok(call),
			_ => Err(()),
		}
	}
}