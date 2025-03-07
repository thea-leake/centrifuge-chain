// Copyright 2021 Centrifuge Foundation (centrifuge.io).
//
// This file is part of the Centrifuge chain project.
// Centrifuge is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version (see http://www.gnu.org/licenses).
// Centrifuge is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

//! The Substrate runtime. This can be compiled with ``#[no_std]`, ready for Wasm.

#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit = "256"]

pub use cfg_primitives::{constants::*, types::*};
use cfg_traits::PoolUpdateGuard;
pub use cfg_types::CurrencyId;
use cfg_types::{
	CustomMetadata, FeeKey, PermissionRoles, PermissionScope, PermissionedCurrencyRole, PoolRole,
	Rate, Role, TimeProvider, TrancheToken,
};
use codec::{Decode, Encode, MaxEncodedLen};
/// Constant values used within the runtime.
use constants::currency::*;
use frame_support::{
	construct_runtime, parameter_types,
	traits::{
		AsEnsureOriginWithArg, Contains, EqualPrivilegeOnly, InstanceFilter, LockIdentifier,
		U128CurrencyToVote, UnixTime,
	},
	weights::{
		constants::{BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight},
		ConstantMultiplier, DispatchClass, Weight,
	},
	PalletId, RuntimeDebug,
};
use frame_system::{
	limits::{BlockLength, BlockWeights},
	EnsureRoot, EnsureSigned,
};
use orml_traits::parameter_type_with_key;
use pallet_anchors::AnchorData;
pub use pallet_balances::Call as BalancesCall;
use pallet_collective::{EnsureMember, EnsureProportionMoreThan};
use pallet_pools::{
	EpochSolution, PoolDetails, ScheduledUpdateDetails, TrancheIndex, TrancheLoc, TrancheSolution,
};
use pallet_restricted_tokens::{FungibleInspectPassthrough, FungiblesInspectPassthrough};
pub use pallet_timestamp::Call as TimestampCall;
pub use pallet_transaction_payment::{CurrencyAdapter, Multiplier, TargetedFeeAdjustment};
use pallet_transaction_payment_rpc_runtime_api::{FeeDetails, RuntimeDispatchInfo};
use polkadot_runtime_common::{prod_or_fast, BlockHashCount, SlowAdjustingFeeUpdate};
use runtime_common::fees::{DealWithFees, WeightToFee};
pub use runtime_common::*;
use scale_info::TypeInfo;
use sp_api::impl_runtime_apis;
use sp_core::OpaqueMetadata;
use sp_inherents::{CheckInherentsResult, InherentData};
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;
use sp_runtime::{
	create_runtime_str, generic, impl_opaque_keys,
	traits::{AccountIdConversion, BlakeTwo256, Block as BlockT, ConvertInto},
	transaction_validity::{TransactionSource, TransactionValidity},
	ApplyExtrinsicResult, Perbill, Permill,
};
use sp_std::prelude::*;
#[cfg(any(feature = "std", test))]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;
use static_assertions::const_assert;
use xcm_executor::XcmExecutor;

use crate::xcm::{XcmConfig, XcmOriginToTransactDispatchOrigin};

pub mod constants;
mod weights;
pub mod xcm;

pub use crate::xcm::*;

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

impl_opaque_keys! {
	pub struct SessionKeys {
		pub aura: Aura,
	}
}

/// Runtime version.
#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
	spec_name: create_runtime_str!("altair"),
	impl_name: create_runtime_str!("altair"),
	authoring_version: 1,
	spec_version: 1020,
	impl_version: 1,
	#[cfg(not(feature = "disable-runtime-api"))]
	apis: RUNTIME_API_VERSIONS,
	#[cfg(feature = "disable-runtime-api")]
	apis: version::create_apis_vec![[]],
	transaction_version: 1,
	state_version: 0,
};

/// Native version.
#[cfg(any(feature = "std", test))]
pub fn native_version() -> NativeVersion {
	NativeVersion {
		runtime_version: VERSION,
		can_author_with: Default::default(),
	}
}

parameter_types! {
	pub const MaximumBlockWeight: Weight = MAXIMUM_BLOCK_WEIGHT;
	pub const Version: RuntimeVersion = VERSION;
	pub RuntimeBlockLength: BlockLength =
		BlockLength::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
	pub RuntimeBlockWeights: BlockWeights = BlockWeights::builder()
		.base_block(BlockExecutionWeight::get())
		.for_class(DispatchClass::all(), |weights| {
			weights.base_extrinsic = ExtrinsicBaseWeight::get();
		})
		.for_class(DispatchClass::Normal, |weights| {
			weights.max_total = Some(NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT);
		})
		.for_class(DispatchClass::Operational, |weights| {
			weights.max_total = Some(MAXIMUM_BLOCK_WEIGHT);
			// Operational transactions have some extra reserved space, so that they
			// are included even if block reached `MAXIMUM_BLOCK_WEIGHT`.
			weights.reserved = Some(
				MAXIMUM_BLOCK_WEIGHT - NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT
			);
		})
		.avg_block_initialization(AVERAGE_ON_INITIALIZE_RATIO)
		.build_or_panic();
	pub const SS58Prefix: u8 = 136;
}

// system support impls
impl frame_system::Config for Runtime {
	/// Data to be associated with an account (other than nonce/transaction counter, which this
	/// module does regardless).
	type AccountData = pallet_balances::AccountData<Balance>;
	/// The identifier used to distinguish between accounts.
	type AccountId = AccountId;
	type BaseCallFilter = BaseCallFilter;
	/// Maximum number of block number to block hash mappings to keep (oldest pruned first).
	type BlockHashCount = BlockHashCount;
	type BlockLength = RuntimeBlockLength;
	/// The index type for blocks.
	type BlockNumber = BlockNumber;
	type BlockWeights = RuntimeBlockWeights;
	/// The aggregated dispatch type that is available for extrinsics.
	type Call = Call;
	type DbWeight = RocksDbWeight;
	/// The overarching event type.
	type Event = Event;
	/// The type for hashing blocks and tries.
	type Hash = Hash;
	/// The hashing algorithm used.
	type Hashing = BlakeTwo256;
	/// The header type.
	type Header = Header;
	/// The index type for storing how many extrinsics an account has signed.
	type Index = Index;
	/// The lookup mechanism to get account ID from whatever is passed in dispatchers.
	type Lookup = sp_runtime::traits::AccountIdLookup<AccountId, ()>;
	type MaxConsumers = frame_support::traits::ConstU32<16>;
	/// A function that is invoked when an account has been determined to be dead.
	/// All resources should be cleaned up associated with the given account.
	type OnKilledAccount = ();
	/// Handler for when a new account has just been created.
	type OnNewAccount = ();
	type OnSetCode = cumulus_pallet_parachain_system::ParachainSetCode<Self>;
	/// The ubiquitous origin type.
	type Origin = Origin;
	type PalletInfo = PalletInfo;
	type SS58Prefix = SS58Prefix;
	type SystemWeightInfo = weights::frame_system::SubstrateWeight<Runtime>;
	/// Get the chain's current version.
	type Version = Version;
}

parameter_types! {
	pub const ReservedXcmpWeight: Weight = MAXIMUM_BLOCK_WEIGHT / 4;
	pub const ReservedDmpWeight: Weight = MAXIMUM_BLOCK_WEIGHT / 4;
}

impl cumulus_pallet_parachain_system::Config for Runtime {
	type CheckAssociatedRelayNumber = cumulus_pallet_parachain_system::RelayNumberStrictlyIncreases;
	type DmpMessageHandler = DmpQueue;
	type Event = Event;
	type OnSystemEvent = ();
	type OutboundXcmpMessageSource = XcmpQueue;
	type ReservedDmpWeight = ReservedDmpWeight;
	type ReservedXcmpWeight = ReservedXcmpWeight;
	type SelfParaId = parachain_info::Pallet<Runtime>;
	type XcmpMessageHandler = XcmpQueue;
}

impl pallet_randomness_collective_flip::Config for Runtime {}

impl parachain_info::Config for Runtime {}

parameter_types! {
	pub const MinimumPeriod: Moment = SLOT_DURATION / 2;
}
impl pallet_timestamp::Config for Runtime {
	type MinimumPeriod = MinimumPeriod;
	/// A timestamp: milliseconds since the unix epoch.
	type Moment = Moment;
	type OnTimestampSet = ();
	type WeightInfo = weights::pallet_timestamp::SubstrateWeight<Self>;
}

// money stuff
parameter_types! {
	/// TransactionByteFee is set to 0.01 MicroAIR
	pub const TransactionByteFee: Balance = 1 * (MICRO_AIR / 100);
	/// This value increases the priority of `Operational` transactions by adding
	/// a "virtual tip" that's equal to the `OperationalFeeMultiplier * final_fee`.
	pub const OperationalFeeMultiplier: u8 = 5;
}

impl pallet_transaction_payment::Config for Runtime {
	type Event = Event;
	type FeeMultiplierUpdate = SlowAdjustingFeeUpdate<Self>;
	type LengthToFee = ConstantMultiplier<Balance, TransactionByteFee>;
	type OnChargeTransaction = CurrencyAdapter<Balances, DealWithFees<Runtime>>;
	type OperationalFeeMultiplier = OperationalFeeMultiplier;
	type WeightToFee = WeightToFee;
}

parameter_types! {
	// the minimum fee for an anchor is 500,000ths of a RAD.
	// This is set to a value so you can still get some return without getting your account removed.
	pub const ExistentialDeposit: Balance = 1 * MICRO_AIR;
	// For weight estimation, we assume that the most locks on an individual account will be 50.
	pub const MaxLocks: u32 = 50;
	pub const MaxReserves: u32 = 50;
}

impl pallet_balances::Config for Runtime {
	/// The means of storing the balances of an account.
	type AccountStore = System;
	/// The type for recording an account's balance.
	type Balance = Balance;
	/// Handler for the unbalanced reduction when removing a dust account.
	type DustRemoval = ();
	/// The overarching event type.
	type Event = Event;
	/// The minimum amount required to keep an account open.
	type ExistentialDeposit = ExistentialDeposit;
	type MaxLocks = MaxLocks;
	type MaxReserves = MaxReserves;
	type ReserveIdentifier = [u8; 8];
	type WeightInfo = weights::pallet_balances::SubstrateWeight<Self>;
}

parameter_types! {
	pub const UncleGenerations: BlockNumber = 5;
}

// We only use find_author to pay in anchor pallet
impl pallet_authorship::Config for Runtime {
	type EventHandler = (CollatorSelection,);
	type FilterUncle = ();
	type FindAuthor = pallet_session::FindAccountFromAuthorIndex<Self, Aura>;
	type UncleGenerations = UncleGenerations;
}

parameter_types! {
	pub const Period: u32 = 6 * HOURS;
	pub const Offset: u32 = 0;
}

impl pallet_session::Config for Runtime {
	type Event = Event;
	type Keys = SessionKeys;
	type NextSessionRotation = pallet_session::PeriodicSessions<Period, Offset>;
	// Essentially just Aura, but lets be pedantic.
	type SessionHandler = <SessionKeys as sp_runtime::traits::OpaqueKeys>::KeyTypeIdProviders;
	type SessionManager = CollatorSelection;
	type ShouldEndSession = pallet_session::PeriodicSessions<Period, Offset>;
	type ValidatorId = <Self as frame_system::Config>::AccountId;
	// we don't have stash and controller, thus we don't need the convert as well.
	type ValidatorIdOf = pallet_collator_selection::IdentityCollator;
	type WeightInfo = pallet_session::weights::SubstrateWeight<Self>;
}

parameter_types! {
	pub const MaxAuthorities: u32 = 32;
}

impl pallet_aura::Config for Runtime {
	type AuthorityId = AuraId;
	type DisabledValidators = ();
	type MaxAuthorities = MaxAuthorities;
}

impl cumulus_pallet_aura_ext::Config for Runtime {}

// substrate pallets
parameter_types! {
	// One storage item; value is size 4+4+16+32 bytes = 56 bytes.
	pub const DepositBase: Balance = 30 * CENTI_AIR;
	// Additional storage item size of 32 bytes.
	pub const DepositFactor: Balance = 5 * CENTI_AIR;
	pub const MaxSignatories: u16 = 100;
}

impl pallet_multisig::Config for Runtime {
	type Call = Call;
	type Currency = Balances;
	type DepositBase = DepositBase;
	type DepositFactor = DepositFactor;
	type Event = Event;
	type MaxSignatories = MaxSignatories;
	type WeightInfo = weights::pallet_multisig::SubstrateWeight<Self>;
}

parameter_types! {
	// One storage item; value is size 4+4+16+32 bytes = 56 bytes.
	pub const ProxyDepositBase: Balance = 30 * CENTI_AIR;
	// Additional storage item size of 32 bytes.
	pub const ProxyDepositFactor: Balance = 5 * CENTI_AIR;
	pub const MaxProxies: u16 = 32;
	pub const AnnouncementDepositBase: Balance = deposit(1, 8);
	pub const AnnouncementDepositFactor: Balance = deposit(0, 66);
	pub const MaxPending: u16 = 32;
}

/// The type used to represent the kinds of proxying allowed.
#[derive(
	Copy,
	Clone,
	Eq,
	PartialEq,
	Ord,
	PartialOrd,
	Encode,
	Decode,
	RuntimeDebug,
	MaxEncodedLen,
	TypeInfo,
)]
pub enum ProxyType {
	Any,
	NonTransfer,
	Governance,
	_Staking, // Deprecated ProxyType, that we are keeping due to the migration
	NonProxy,
}
impl Default for ProxyType {
	fn default() -> Self {
		Self::Any
	}
}

impl InstanceFilter<Call> for ProxyType {
	fn filter(&self, c: &Call) -> bool {
		match self {
			ProxyType::Any => true,
			ProxyType::NonTransfer => !matches!(c, Call::Balances(..)),
			ProxyType::Governance => matches!(
				c,
				Call::Democracy(..) | Call::Council(..) | Call::Elections(..) | Call::Utility(..)
			),
			ProxyType::_Staking => false,
			ProxyType::NonProxy => {
				matches!(c, Call::Proxy(pallet_proxy::Call::proxy { .. }))
					|| !matches!(c, Call::Proxy(..))
			}
		}
	}

	fn is_superset(&self, o: &Self) -> bool {
		match (self, o) {
			(x, y) if x == y => true,
			(ProxyType::Any, _) => true,
			(_, ProxyType::Any) => false,
			(_, ProxyType::NonProxy) => false,
			(ProxyType::NonTransfer, _) => true,
			_ => false,
		}
	}
}

impl pallet_proxy::Config for Runtime {
	type AnnouncementDepositBase = AnnouncementDepositBase;
	type AnnouncementDepositFactor = AnnouncementDepositFactor;
	type Call = Call;
	type CallHasher = BlakeTwo256;
	type Currency = Balances;
	type Event = Event;
	type MaxPending = MaxPending;
	type MaxProxies = MaxProxies;
	type ProxyDepositBase = ProxyDepositBase;
	type ProxyDepositFactor = ProxyDepositFactor;
	type ProxyType = ProxyType;
	type WeightInfo = weights::pallet_proxy::SubstrateWeight<Self>;
}

impl pallet_utility::Config for Runtime {
	type Call = Call;
	type Event = Event;
	type PalletsOrigin = OriginCaller;
	type WeightInfo = weights::pallet_utility::SubstrateWeight<Self>;
}

parameter_types! {
	pub MaximumSchedulerWeight: Weight = Perbill::from_percent(80) * MaximumBlockWeight::get();
	pub const MaxScheduledPerBlock: u32 = 50;
	// Retry a scheduled item every 10 blocks (2 minutes) until the preimage exists.
	pub const NoPreimagePostponement: Option<u32> = Some(10);
}

impl pallet_scheduler::Config for Runtime {
	type Call = Call;
	type Event = Event;
	type MaxScheduledPerBlock = MaxScheduledPerBlock;
	type MaximumWeight = MaximumSchedulerWeight;
	type NoPreimagePostponement = NoPreimagePostponement;
	type Origin = Origin;
	type OriginPrivilegeCmp = EqualPrivilegeOnly;
	type PalletsOrigin = OriginCaller;
	type PreimageProvider = Preimage;
	type ScheduleOrigin = EnsureRoot<AccountId>;
	type WeightInfo = weights::pallet_scheduler::SubstrateWeight<Self>;
}

parameter_types! {
	pub const PreimageMaxSize: u32 = 4096 * 1024;
	pub PreimageBaseDeposit: Balance = deposit(2, 64);
	pub PreimageByteDeposit: Balance = deposit(0, 1);
}

impl pallet_preimage::Config for Runtime {
	type BaseDeposit = PreimageBaseDeposit;
	type ByteDeposit = PreimageByteDeposit;
	type Currency = Balances;
	type Event = Event;
	type ManagerOrigin = EnsureRoot<AccountId>;
	type MaxSize = PreimageMaxSize;
	type WeightInfo = weights::pallet_preimage::SubstrateWeight<Self>;
}

parameter_types! {
	pub CouncilMotionDuration: BlockNumber = prod_or_fast!(5 * DAYS, 1 * MINUTES, "AIR_MOTION_DURATION");
	pub const CouncilMaxProposals: u32 = 100;
	pub const CouncilMaxMembers: u32 = 100;
}

impl pallet_collective::Config<CouncilCollective> for Runtime {
	type DefaultVote = pallet_collective::PrimeDefaultVote;
	type Event = Event;
	type MaxMembers = CouncilMaxMembers;
	type MaxProposals = CouncilMaxProposals;
	type MotionDuration = CouncilMotionDuration;
	type Origin = Origin;
	type Proposal = Call;
	type WeightInfo = weights::pallet_collective::SubstrateWeight<Self>;
}

parameter_types! {
	pub const CandidacyBond: Balance = 500 * AIR;
	pub const VotingBond: Balance = 50 * CENTI_AIR;
	pub const VotingBondBase: Balance = 50 * CENTI_AIR;
	pub TermDuration: BlockNumber = prod_or_fast!(7 * DAYS, 1 * MINUTES, "AIR_TERM_DURATION");
	pub const DesiredMembers: u32 = 9;
	pub const DesiredRunnersUp: u32 = 9;
	pub const ElectionsPhragmenModuleId: LockIdentifier = *b"phrelect";
}

// Make sure that there are no more than `MAX_MEMBERS` members elected via elections-phragmen.
const_assert!(DesiredMembers::get() <= CouncilMaxMembers::get());

impl pallet_elections_phragmen::Config for Runtime {
	/// How much should be locked up in order to submit one's candidacy.
	type CandidacyBond = CandidacyBond;
	type ChangeMembers = Council;
	type Currency = Balances;
	type CurrencyToVote = U128CurrencyToVote;
	/// Number of members to elect.
	type DesiredMembers = DesiredMembers;
	/// Number of runners_up to keep.
	type DesiredRunnersUp = DesiredRunnersUp;
	type Event = Event;
	type InitializeMembers = Council;
	type KickedMember = Treasury;
	type LoserCandidate = Treasury;
	type PalletId = ElectionsPhragmenModuleId;
	/// How long each seat is kept. This defines the next block number at which an election
	/// round will happen. If set to zero, no elections are ever triggered and the module will
	/// be in passive mode.
	type TermDuration = TermDuration;
	/// Base deposit associated with voting
	type VotingBondBase = VotingBondBase;
	/// How much should be locked up in order to be able to submit votes.
	type VotingBondFactor = VotingBond;
	type WeightInfo = pallet_elections_phragmen::weights::SubstrateWeight<Self>;
}

parameter_types! {
	pub LaunchPeriod: BlockNumber = prod_or_fast!(7 * DAYS, 1 * MINUTES, "AIR_LAUNCH_PERIOD");
	pub VotingPeriod: BlockNumber = prod_or_fast!(7 * DAYS, 1 * MINUTES, "AIR_VOTING_PERIOD");
	pub FastTrackVotingPeriod: BlockNumber = prod_or_fast!(3 * HOURS, 1 * MINUTES, "AIR_FAST_TRACK_VOTING_PERIOD");
	pub const InstantAllowed: bool = false;
	pub const MinimumDeposit: Balance = 500 * AIR;
	pub EnactmentPeriod: BlockNumber = prod_or_fast!(8 * DAYS, 1 * MINUTES, "AIR_ENACTMENT_PERIOD");
	pub CooloffPeriod: BlockNumber = prod_or_fast!(7 * DAYS, 1 * MINUTES, "AIR_COOLOFF_PERIOD");
	pub const MaxProposals: u32 = 100;
	pub const MaxVotes: u32 = 100;
}

impl pallet_democracy::Config for Runtime {
	type BlacklistOrigin = EnsureRoot<AccountId>;
	// To cancel a proposal before it has been passed, must be root.
	type CancelProposalOrigin = EnsureRoot<AccountId>;
	// To cancel a proposal which has been passed, 2/3 of the council must agree to it.
	type CancellationOrigin = EnsureRootOr<TwoThirdOfCouncil>;
	/// Period in blocks where an external proposal may not be re-submitted after being vetoed.
	type CooloffPeriod = CooloffPeriod;
	type Currency = Balances;
	/// The minimum period of locking and the period between a proposal being approved and enacted.
	///
	/// It should generally be a little more than the unstake period to ensure that
	/// voting stakers have an opportunity to remove themselves from the system in the case where
	/// they are on the losing side of a vote.
	type EnactmentPeriod = EnactmentPeriod;
	type Event = Event;
	/// A unanimous council can have the next scheduled referendum be a straight default-carries
	/// (NTB) vote.
	type ExternalDefaultOrigin = AllOfCouncil;
	/// A super-majority can have the next scheduled referendum be a straight majority-carries vote.
	type ExternalMajorityOrigin = HalfOfCouncil;
	/// A straight majority of the council can decide what their next motion is.
	type ExternalOrigin = HalfOfCouncil;
	/// Half of the council can have an ExternalMajority/ExternalDefault vote
	/// be tabled immediately and with a shorter voting/enactment period.
	type FastTrackOrigin = EnsureRootOr<HalfOfCouncil>;
	type FastTrackVotingPeriod = FastTrackVotingPeriod;
	type InstantAllowed = InstantAllowed;
	type InstantOrigin = EnsureRootOr<AllOfCouncil>;
	// Same as EnactmentPeriod
	/// How often (in blocks) new public referenda are launched.
	type LaunchPeriod = LaunchPeriod;
	type MaxProposals = MaxProposals;
	type MaxVotes = MaxVotes;
	/// The minimum amount to be used as a deposit for a public referendum proposal.
	type MinimumDeposit = MinimumDeposit;
	type OperationalPreimageOrigin = EnsureMember<AccountId, CouncilCollective>;
	type PalletsOrigin = OriginCaller;
	/// The amount of balance that must be deposited per byte of preimage stored.
	type PreimageByteDeposit = PreimageByteDeposit;
	type Proposal = Call;
	type Scheduler = Scheduler;
	/// Handler for the unbalanced reduction when slashing a preimage deposit.
	type Slash = Treasury;
	// Any single council member may veto a coming council proposal, however they can
	// only do it once and it lasts only for the cooloff period.
	type VetoOrigin = EnsureMember<AccountId, CouncilCollective>;
	type VoteLockingPeriod = EnactmentPeriod;
	/// How often (in blocks) to check for new votes.
	type VotingPeriod = VotingPeriod;
	type WeightInfo = weights::pallet_democracy::SubstrateWeight<Self>;
}

parameter_types! {
	pub const MaxSubAccounts: u32 = 100;
	pub const MaxAdditionalFields: u32 = 100;
	pub const BasicDeposit: Balance = 100 * AIR;
	pub const FieldDeposit: Balance = 25 * AIR;
	pub const SubAccountDeposit: Balance = 20 * AIR;
	pub const MaxRegistrars: u32 = 20;
}

impl pallet_identity::Config for Runtime {
	type BasicDeposit = BasicDeposit;
	type Currency = Balances;
	type Event = Event;
	type FieldDeposit = FieldDeposit;
	type ForceOrigin = EnsureRootOr<EnsureProportionMoreThan<AccountId, CouncilCollective, 1, 2>>;
	type MaxAdditionalFields = MaxAdditionalFields;
	type MaxRegistrars = MaxRegistrars;
	type MaxSubAccounts = MaxSubAccounts;
	type RegistrarOrigin =
		EnsureRootOr<EnsureProportionMoreThan<AccountId, CouncilCollective, 1, 2>>;
	type Slashed = Treasury;
	type SubAccountDeposit = SubAccountDeposit;
	type WeightInfo = weights::pallet_identity::SubstrateWeight<Self>;
}

parameter_types! {
	pub const MinVestedTransfer: Balance = MIN_VESTING * AIR;
}

impl pallet_vesting::Config for Runtime {
	type BlockNumberToBalance = ConvertInto;
	type Currency = Balances;
	type Event = Event;
	type MinVestedTransfer = MinVestedTransfer;
	type WeightInfo = weights::pallet_vesting::SubstrateWeight<Self>;

	const MAX_VESTING_SCHEDULES: u32 = 28;
}

parameter_types! {
	// 5% of the proposal value need to be bonded. This will be returned
	pub const ProposalBond: Permill = Permill::from_percent(5);

	// Minimum amount to bond per proposal. This will be the least that gets bonded per proposal
	// if the above yields to lower value
	pub const ProposalBondMinimum: Balance = 100 * AIR;

	// Maximum amount to bond per proposal. This will be the most that gets bonded per proposal
	pub const ProposalBondMaximum: Balance = 500 * AIR;

	// periods between treasury spends
	pub const SpendPeriod: BlockNumber = 6 * DAYS;

	// percentage of treasury we burn per Spend period if there is a surplus
	// If the treasury is able to spend on all the approved proposals and didn't miss any
	// then we burn % amount of remaining balance
	// If the treasury couldn't spend on all the approved proposals, then we dont burn any
	pub const Burn: Permill = Permill::from_percent(0);

	// treasury pallet account id
	pub const TreasuryPalletId: PalletId = cfg_types::ids::TREASURY_PALLET_ID;

	// Maximum number of approvals that can be in the spending queue
	pub const MaxApprovals: u32 = 100;
}

impl pallet_treasury::Config for Runtime {
	// either democracy or 66% of council votes
	type ApproveOrigin = EnsureRootOr<TwoThirdOfCouncil>;
	type Burn = Burn;
	// we burn and dont handle the unbalance
	type BurnDestination = ();
	type Currency = Balances;
	type Event = Event;
	type MaxApprovals = MaxApprovals;
	// slashed amount goes to treasury account
	type OnSlash = Treasury;
	type PalletId = TreasuryPalletId;
	type ProposalBond = ProposalBond;
	type ProposalBondMaximum = ProposalBondMaximum;
	type ProposalBondMinimum = ProposalBondMinimum;
	// either democracy or more than 50% council votes
	type RejectOrigin = EnsureRootOr<EnsureProportionMoreThan<AccountId, CouncilCollective, 1, 2>>;
	type SpendFunds = ();
	type SpendOrigin = frame_support::traits::NeverEnsureOrigin<Balance>;
	type SpendPeriod = SpendPeriod;
	type WeightInfo = weights::pallet_treasury::SubstrateWeight<Self>;
}

parameter_types! {
	// per byte deposit is 0.01 AIR
	pub const DepositPerByte: Balance = CENTI_AIR;
	// Base deposit to add attribute is 0.1 AIR
	pub const AttributeDepositBase: Balance = 10 * CENTI_AIR;
	// Base deposit to add metadata is 0.1 AIR
	pub const MetadataDepositBase: Balance = 10 * CENTI_AIR;
	// Deposit to create a class is 1 AIR
	pub const CollectionDeposit: Balance = AIR;
	// Deposit to create a class is 0.1 AIR
	pub const ItemDeposit: Balance = 10 * CENTI_AIR;
	// Maximum limit of bytes for Metadata, Attribute key and Value
	pub const Limit: u32 = 256;
}

impl pallet_uniques::Config for Runtime {
	type AttributeDepositBase = AttributeDepositBase;
	type CollectionDeposit = CollectionDeposit;
	type CollectionId = CollectionId;
	type CreateOrigin = AsEnsureOriginWithArg<EnsureSigned<AccountId>>;
	type Currency = Balances;
	type DepositPerByte = DepositPerByte;
	type Event = Event;
	// a straight majority of council can act as force origin
	type ForceOrigin = EnsureRootOr<HalfOfCouncil>;
	#[cfg(feature = "runtime-benchmarks")]
	type Helper = ();
	type ItemDeposit = ItemDeposit;
	type ItemId = ItemId;
	type KeyLimit = Limit;
	type Locker = ();
	type MetadataDepositBase = MetadataDepositBase;
	type StringLimit = Limit;
	type ValueLimit = Limit;
	type WeightInfo = weights::pallet_uniques::SubstrateWeight<Self>;
}

// our pallets
parameter_types! {
	pub const DefaultFeeValue: Balance = DEFAULT_FEE_VALUE;
}

impl pallet_fees::Config for Runtime {
	type Currency = Balances;
	type DefaultFeeValue = DefaultFeeValue;
	type Event = Event;
	type FeeChangeOrigin = EnsureRootOr<HalfOfCouncil>;
	type FeeKey = FeeKey;
	type Treasury = pallet_treasury::Pallet<Self>;
	type WeightInfo = weights::pallet_fees::SubstrateWeight<Self>;
}

parameter_types! {
	pub const CommitAnchorFeeKey: FeeKey = FeeKey::AnchorsCommit;
	pub const PreCommitDepositFeeKey: FeeKey = FeeKey::AnchorsPreCommit;
}

impl pallet_anchors::Config for Runtime {
	type CommitAnchorFeeKey = CommitAnchorFeeKey;
	type Currency = Balances;
	type Fees = Fees;
	type PreCommitDepositFeeKey = PreCommitDepositFeeKey;
	type WeightInfo = ();
}

impl pallet_collator_allowlist::Config for Runtime {
	type Event = Event;
	type ValidatorId = AccountId;
	type ValidatorRegistration = Session;
	type WeightInfo = weights::pallet_collator_allowlist::SubstrateWeight<Self>;
}

parameter_types! {
	pub const MigrationMaxAccounts: u32 = 100;
	pub const MigrationMaxVestings: u32 = 10;
	pub const MigrationMaxProxies: u32 = 10;
}

// Implement the migration manager pallet
// The actual associated type, which executes the migration can be found in the migration folder
impl pallet_migration_manager::Config for Runtime {
	type Event = Event;
	type MigrationMaxAccounts = MigrationMaxAccounts;
	type MigrationMaxProxies = MigrationMaxProxies;
	type MigrationMaxVestings = MigrationMaxVestings;
	type WeightInfo = weights::pallet_migration_manager::SubstrateWeight<Self>;
}

// Parameterize crowdloan reward pallet configuration
parameter_types! {
	pub const CrowdloanRewardPalletId: PalletId = cfg_types::ids::CROWDLOAN_REWARD_PALLET_ID;
}

// Implement crowdloan reward pallet's configuration trait for the runtime
impl pallet_crowdloan_reward::Config for Runtime {
	type AdminOrigin = EnsureRootOr<HalfOfCouncil>;
	type Event = Event;
	type PalletId = CrowdloanRewardPalletId;
	type WeightInfo = weights::pallet_crowdloan_reward::SubstrateWeight<Self>;
}

// Parameterize crowdloan claim pallet
parameter_types! {
	pub const CrowdloanClaimPalletId: PalletId = cfg_types::ids::CROWDLOAN_CLAIM_PALLET_ID;
	pub const MaxProofLength: u32 = 30;
}

// Implement crowdloan claim pallet configuration trait for the runtime
impl pallet_crowdloan_claim::Config for Runtime {
	type AdminOrigin = EnsureRootOr<HalfOfCouncil>;
	type Event = Event;
	type MaxProofLength = MaxProofLength;
	type PalletId = CrowdloanClaimPalletId;
	type RelayChainAccountId = AccountId;
	type RewardMechanism = CrowdloanReward;
	type WeightInfo = weights::pallet_crowdloan_claim::SubstrateWeight<Self>;
}

// Parameterize collator selection pallet
parameter_types! {
	pub const PotId: PalletId = cfg_types::ids::STAKE_POT_PALLET_ID;
	pub const MaxCandidates: u32 = 1000;
	pub const MinCandidates: u32 = 5;
	pub const SessionLength: BlockNumber = 6 * HOURS;
	pub const MaxInvulnerables: u32 = 100;
}

// Implement Collator Selection pallet configuration trait for the runtime
impl pallet_collator_selection::Config for Runtime {
	type Currency = Balances;
	type Event = Event;
	// should be a multiple of session or things will get inconsistent
	type KickThreshold = Period;
	type MaxCandidates = MaxCandidates;
	type MaxInvulnerables = MaxInvulnerables;
	type MinCandidates = MinCandidates;
	type PotId = PotId;
	type UpdateOrigin = EnsureRootOr<ThreeFourthOfCouncil>;
	type ValidatorId = <Self as frame_system::Config>::AccountId;
	type ValidatorIdOf = pallet_collator_selection::IdentityCollator;
	type ValidatorRegistration = CollatorAllowlist;
	type WeightInfo = pallet_collator_selection::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
	pub const MaxTranches: u32 = 5;

	// How much time should lapse before a tranche investor can be removed
	#[derive(Debug, Eq, PartialEq, scale_info::TypeInfo, Clone)]
	pub const MinDelay: Moment = 7 * SECONDS_PER_DAY;

	#[derive(Debug, Eq, PartialEq, scale_info::TypeInfo, Clone)]
	pub const MaxRolesPerPool: u32 = 10_000;
}

impl pallet_permissions::Config for Runtime {
	type AdminOrigin = EnsureRootOr<HalfOfCouncil>;
	type Editors = Editors;
	type Event = Event;
	type MaxRolesPerScope = MaxRolesPerPool;
	type Role = Role<TrancheId, Moment>;
	type Scope = PermissionScope<PoolId, CurrencyId>;
	type Storage = PermissionRoles<TimeProvider<Timestamp>, MinDelay, TrancheId, Moment>;
	type WeightInfo = weights::pallet_permissions::SubstrateWeight<Self>;
}

pub struct Editors;
impl
	Contains<(
		AccountId,
		Option<Role<TrancheId, Moment>>,
		PermissionScope<PoolId, CurrencyId>,
		Role<TrancheId, Moment>,
	)> for Editors
{
	fn contains(
		t: &(
			AccountId,
			Option<Role<TrancheId, Moment>>,
			PermissionScope<PoolId, CurrencyId>,
			Role<TrancheId, Moment>,
		),
	) -> bool {
		let (_editor, maybe_role, _pool, role) = t;
		if let Some(with_role) = maybe_role {
			match *with_role {
				Role::PoolRole(PoolRole::PoolAdmin) => match *role {
					// PoolAdmins can manage all other admins, but not tranche investors
					Role::PoolRole(PoolRole::TrancheInvestor(_, _)) => false,
					_ => true,
				},
				Role::PoolRole(PoolRole::MemberListAdmin) => match *role {
					// MemberlistAdmins can manage tranche investors
					Role::PoolRole(PoolRole::TrancheInvestor(_, _)) => true,
					_ => false,
				},
				Role::PermissionedCurrencyRole(PermissionedCurrencyRole::Manager) => match *role {
					Role::PermissionedCurrencyRole(PermissionedCurrencyRole::Holder(_)) => true,
					_ => false,
				},
				_ => false,
			}
		} else {
			false
		}
	}
}

parameter_types! {
	pub const NativeToken: CurrencyId = CurrencyId::Native;
}

impl pallet_restricted_tokens::Config for Runtime {
	type Balance = Balance;
	type CurrencyId = CurrencyId;
	type Event = Event;
	type Fungibles = OrmlTokens;
	type NativeFungible = Balances;
	type NativeToken = NativeToken;
	type PreCurrency = cfg_traits::Always;
	type PreExtrTransfer = cfg_traits::Always;
	type PreFungibleInspect = FungibleInspectPassthrough;
	type PreFungibleInspectHold = cfg_traits::Always;
	type PreFungibleMutate = cfg_traits::Always;
	type PreFungibleMutateHold = cfg_traits::Always;
	type PreFungibleTransfer = cfg_traits::Always;
	type PreFungiblesInspect = FungiblesInspectPassthrough;
	type PreFungiblesInspectHold = cfg_traits::Always;
	type PreFungiblesMutate = cfg_traits::Always;
	type PreFungiblesMutateHold = cfg_traits::Always;
	type PreFungiblesTransfer = cfg_traits::Always;
	type PreReservableCurrency = cfg_traits::Always;
	type WeightInfo = weights::pallet_restricted_tokens::SubstrateWeight<Self>;
}

parameter_type_with_key! {
	pub ExistentialDeposits: |currency_id: CurrencyId| -> Balance {
		match currency_id {
			CurrencyId::Native => ExistentialDeposit::get(),
			_ => 0,
		}
	};
}

parameter_types! {
	pub TreasuryAccount: AccountId = TreasuryPalletId::get().into_account_truncating();
}

impl orml_tokens::Config for Runtime {
	type Amount = IBalance;
	type Balance = Balance;
	type CurrencyId = CurrencyId;
	type DustRemovalWhitelist = frame_support::traits::Nothing;
	type Event = Event;
	type ExistentialDeposits = ExistentialDeposits;
	type MaxLocks = MaxLocks;
	type MaxReserves = MaxReserves;
	type OnDust = orml_tokens::TransferDust<Runtime, TreasuryAccount>;
	type OnKilledTokenAccount = ();
	type OnNewTokenAccount = ();
	type ReserveIdentifier = [u8; 8];
	type WeightInfo = ();
}

impl orml_asset_registry::Config for Runtime {
	type AssetId = CurrencyId;
	type AssetProcessor = asset_registry::CustomAssetProcessor;
	type AuthorityOrigin = asset_registry::AuthorityOrigin<Origin, EnsureRootOr<HalfOfCouncil>>;
	type Balance = Balance;
	type CustomMetadata = CustomMetadata;
	type Event = Event;
	type WeightInfo = ();
}

parameter_types! {
	pub const NftSalesPalletId: PalletId = cfg_types::ids::NFT_SALES_PALLET_ID;
}

impl pallet_nft_sales::Config for Runtime {
	type CollectionId = CollectionId;
	type Event = Event;
	type Fungibles = Tokens;
	type ItemId = ItemId;
	type NonFungibles = Uniques;
	type PalletId = NftSalesPalletId;
	type WeightInfo = weights::pallet_nft_sales::SubstrateWeight<Self>;
}

// XCM

/// XCMP Queue is responsible to handle XCM messages coming directly from sibling parachains.
impl cumulus_pallet_xcmp_queue::Config for Runtime {
	type ChannelInfo = ParachainSystem;
	type ControllerOrigin = EnsureRoot<AccountId>;
	type ControllerOriginConverter = XcmOriginToTransactDispatchOrigin;
	type Event = Event;
	type ExecuteOverweightOrigin = EnsureRoot<AccountId>;
	type VersionWrapper = PolkadotXcm;
	type WeightInfo = cumulus_pallet_xcmp_queue::weights::SubstrateWeight<Self>;
	type XcmExecutor = XcmExecutor<XcmConfig>;
}

/// The config for the Downward Message Passing Queue, i.e., how messages coming from the
/// relay-chain are handled.
impl cumulus_pallet_dmp_queue::Config for Runtime {
	type Event = Event;
	type ExecuteOverweightOrigin = EnsureRoot<AccountId>;
	type XcmExecutor = XcmExecutor<XcmConfig>;
}

// Pools & Loans

parameter_types! {
	pub const LoansPalletId: PalletId = cfg_types::ids::LOANS_PALLET_ID;
	pub const MaxActiveLoansPerPool: u32 = 300;
	pub const MaxWriteOffGroups: u32 = 100;
}

impl pallet_loans::Config for Runtime {
	type Balance = Balance;
	type BlockNumberProvider = System;
	type ClassId = CollectionId;
	type CurrencyId = CurrencyId;
	type Event = Event;
	type InterestAccrual = InterestAccrual;
	type LoanId = ItemId;
	type LoansPalletId = LoansPalletId;
	type MaxActiveLoansPerPool = MaxActiveLoansPerPool;
	type MaxWriteOffGroups = MaxWriteOffGroups;
	type NonFungible = Uniques;
	type Permission = Permissions;
	type Pool = Pools;
	type Rate = Rate;
	type Time = Timestamp;
	type WeightInfo = weights::pallet_loans::SubstrateWeight<Self>;
}

parameter_types! {
	pub const PoolPalletId: frame_support::PalletId = cfg_types::ids::POOLS_PALLET_ID;

	pub const MinUpdateDelay: u64 = if cfg!(feature = "runtime-benchmarks") {
		0
	} else {
		2 * SECONDS_PER_DAY
	};

	pub const ChallengeTime: BlockNumber = if cfg!(feature = "runtime-benchmarks") {
		// Disable challenge time in benchmarks
		0
	} else {
		30 * MINUTES
	};

	// Defaults for pool parameters
	pub const DefaultMinEpochTime: u64 = 23 * SECONDS_PER_HOUR + 50 * SECONDS_PER_MINUTE; // Just under a day
	pub const DefaultMaxNAVAge: u64 = 0;

	// Runtime-defined constraints for pool parameters
	pub const MinEpochTimeLowerBound: u64 = if cfg!(feature = "runtime-benchmarks") {
		// Allow short epoch time in benchmarks
		1
	} else {
		1 * SECONDS_PER_HOUR // 1 hour
	};
	pub const MinEpochTimeUpperBound: u64 = 30 * SECONDS_PER_DAY; // 1 month
	pub const MaxNAVAgeUpperBound: u64 = if cfg!(feature = "runtime-benchmarks") {
		// Allow an aged NAV in benchmarks
		SECONDS_PER_HOUR
	} else {
		0
	};

	// Pool metadata limit
	#[derive(scale_info::TypeInfo, Eq, PartialEq, Debug, Clone, Copy )]
	pub const MaxSizeMetadata: u32 = 46; // length of IPFS hash

	#[derive(scale_info::TypeInfo, Eq, PartialEq, Debug, Clone, Copy )]
	pub const MaxTokenNameLength: u32 = 128;

	#[derive(scale_info::TypeInfo, Eq, PartialEq, Debug, Clone, Copy )]
	pub const MaxTokenSymbolLength: u32 = 128;

	// Deposit to create a pool. This covers pool data, loan data, and permissions data.
	pub const PoolDeposit: Balance = 0;
}

// The pool benchmarks can't handle a required root origin (yet).
// TODO: Fix those benchmarks and remove this
#[cfg(not(feature = "runtime-benchmarks"))]
type PoolCreateOrigin = EnsureRoot<AccountId>;
#[cfg(feature = "runtime-benchmarks")]
type PoolCreateOrigin = EnsureSigned<AccountId>;

impl pallet_pools::Config for Runtime {
	type AssetRegistry = OrmlAssetRegistry;
	type Balance = Balance;
	type BalanceRatio = Rate;
	type ChallengeTime = ChallengeTime;
	type Currency = Balances;
	type CurrencyId = CurrencyId;
	type DefaultMaxNAVAge = DefaultMaxNAVAge;
	type DefaultMinEpochTime = DefaultMinEpochTime;
	type EpochId = u32;
	type Event = Event;
	type InterestRate = Rate;
	type MaxNAVAgeUpperBound = MaxNAVAgeUpperBound;
	type MaxSizeMetadata = MaxSizeMetadata;
	type MaxTokenNameLength = MaxTokenNameLength;
	type MaxTokenSymbolLength = MaxTokenSymbolLength;
	type MaxTranches = MaxTranches;
	type MinEpochTimeLowerBound = MinEpochTimeLowerBound;
	type MinEpochTimeUpperBound = MinEpochTimeUpperBound;
	type MinUpdateDelay = MinUpdateDelay;
	type NAV = Loans;
	type PalletId = PoolPalletId;
	type ParachainId = ParachainInfo;
	type Permission = Permissions;
	type PoolCreateOrigin = PoolCreateOrigin;
	type PoolCurrency = PoolCurrency;
	type PoolDeposit = PoolDeposit;
	type PoolId = PoolId;
	type Time = Timestamp;
	type Tokens = Tokens;
	type TrancheId = TrancheId;
	type TrancheToken = TrancheToken;
	type TrancheWeight = TrancheWeight;
	type UpdateGuard = UpdateGuard;
	type WeightInfo = weights::pallet_pools::SubstrateWeight<Runtime>;
}

pub struct PoolCurrency;
impl Contains<CurrencyId> for PoolCurrency {
	fn contains(id: &CurrencyId) -> bool {
		match id {
			CurrencyId::Tranche(_, _)
			| CurrencyId::Native
			| CurrencyId::KSM
			| CurrencyId::ForeignAsset(_) => false,
			CurrencyId::AUSD => true,
		}
	}
}

pub struct UpdateGuard;
impl PoolUpdateGuard for UpdateGuard {
	type Moment = Moment;
	type PoolDetails = PoolDetails<
		CurrencyId,
		u32,
		Balance,
		Rate,
		MaxSizeMetadata,
		TrancheWeight,
		TrancheId,
		PoolId,
	>;
	type ScheduledUpdateDetails =
		ScheduledUpdateDetails<Rate, MaxTokenNameLength, MaxTokenSymbolLength>;

	fn released(
		pool: &Self::PoolDetails,
		update: &Self::ScheduledUpdateDetails,
		now: Self::Moment,
	) -> bool {
		if now < update.scheduled_time {
			return false;
		}

		// The epoch in which the redemptions were fulfilled,
		// should have closed after the scheduled time already,
		// to ensure that investors had the `MinUpdateDelay`
		// to submit their redemption orders.
		if now < pool.epoch.last_closed {
			return false;
		}

		// There should be no outstanding redemption orders.
		let acc_outstanding_redemptions = pool
			.tranches
			.acc_outstanding_redemptions()
			.unwrap_or(Balance::MAX);
		if acc_outstanding_redemptions != 0u128 {
			return false;
		}

		return true;
	}
}

impl pallet_interest_accrual::Config for Runtime {
	type Balance = Balance;
	type Event = Event;
	type InterestRate = Rate;
	type Time = Timestamp;
	type Weights = ();
}

/// Base Call Filter
/// We block any call that could lead for tranche tokens to be transferred through XCM.
pub struct BaseCallFilter;
impl Contains<Call> for BaseCallFilter {
	fn contains(c: &Call) -> bool {
		match c {
			Call::PolkadotXcm(method) => match method {
				// We disable all PolkadotXcm extrinsics that allow users to build XCM messages
				// from scratch, which could have them transferring Tranche tokens.
				// To transfer tokens, use XTokens, for which we have specific filters
				// blocking tranche transfers.
				// To send a raw XCM message, use orml_xcm, which ensures the origin of
				// such call to be root or majority of the collective.
				pallet_xcm::Call::send { .. }
				| pallet_xcm::Call::execute { .. }
				| pallet_xcm::Call::teleport_assets { .. }
				| pallet_xcm::Call::reserve_transfer_assets { .. }
				| pallet_xcm::Call::limited_reserve_transfer_assets { .. }
				| pallet_xcm::Call::limited_teleport_assets { .. } => {
					return false;
				}
				pallet_xcm::Call::force_xcm_version { .. }
				| pallet_xcm::Call::force_default_xcm_version { .. }
				| pallet_xcm::Call::force_subscribe_version_notify { .. }
				| pallet_xcm::Call::force_unsubscribe_version_notify { .. } => {
					return true;
				}
				pallet_xcm::Call::__Ignore { .. } => {
					unimplemented!()
				}
			},
			Call::XTokens(method) => match method {
				orml_xtokens::Call::transfer {
					currency_id: CurrencyId::Tranche(_, _),
					..
				}
				| orml_xtokens::Call::transfer_with_fee {
					currency_id: CurrencyId::Tranche(_, _),
					..
				}
				// We preemptively disable this as we haven't encountered a use case for it.
				// Shall some user or use case require it, we will make it more fine-grained.
				| orml_xtokens::Call::transfer_multiasset { .. }
				| orml_xtokens::Call::transfer_multiasset_with_fee { .. }
				| orml_xtokens::Call::transfer_multiassets { .. }
				| orml_xtokens::Call::transfer_multicurrencies { .. } => false,
				// Any other XTokens call is good to go
				_ => true,
			},
			_ => true,
		}
	}
}

// Frame Order in this block dictates the index of each one in the metadata
// Any addition should be done at the bottom
// Any deletion affects the following frames during runtime upgrades
construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = node_primitives::Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
		// basic system stuff
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>} = 0,
		ParachainSystem: cumulus_pallet_parachain_system::{Pallet, Call, Config, Storage, Inherent, Event<T>} = 1,
		RandomnessCollectiveFlip: pallet_randomness_collective_flip::{Pallet, Storage} = 2,
		Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent} = 3,
		ParachainInfo: parachain_info::{Pallet, Storage, Config} = 4,

		// money stuff
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>} = 20,
		TransactionPayment: pallet_transaction_payment::{Event<T>, Pallet, Storage} = 21,

		// authoring stuff
		// collator_selection must go here in order for the storage to be available to pallet_session
		CollatorSelection: pallet_collator_selection::{Pallet, Call, Storage, Event<T>, Config<T>} = 71,
		Authorship: pallet_authorship::{Pallet, Call, Storage} = 30,
		Session: pallet_session::{Pallet, Call, Storage, Event, Config<T>} = 31,
		Aura: pallet_aura::{Pallet, Storage, Config<T>} = 32,
		AuraExt: cumulus_pallet_aura_ext::{Pallet, Storage, Config} = 33,

		// substrate pallets
		Multisig: pallet_multisig::{Pallet, Call, Storage, Event<T>} = 60,
		Proxy: pallet_proxy::{Pallet, Call, Storage, Event<T>} = 61,
		Utility: pallet_utility::{Pallet, Call, Event} = 62,
		Scheduler: pallet_scheduler::{Pallet, Call, Storage, Event<T>} = 63,
		Council: pallet_collective::<Instance1>::{Pallet, Call, Storage, Origin<T>, Event<T>, Config<T>} = 64,
		Elections: pallet_elections_phragmen::{Pallet, Call, Storage, Event<T>, Config<T>} = 65,
		Democracy: pallet_democracy::{Pallet, Call, Storage, Config<T>, Event<T>} = 66,
		Identity: pallet_identity::{Pallet, Call, Storage, Event<T>} = 67,
		Vesting: pallet_vesting::{Pallet, Call, Storage, Event<T>, Config<T>} = 68,
		Treasury: pallet_treasury::{Pallet, Call, Storage, Config, Event<T>} = 69,
		Preimage: pallet_preimage::{Pallet, Call, Storage, Event<T>} = 70,
		Uniques: pallet_uniques::{Pallet, Call, Storage, Event<T>} = 72,

		// our pallets
		Fees: pallet_fees::{Pallet, Call, Storage, Config<T>, Event<T>} = 90,
		Anchor: pallet_anchors::{Pallet, Call, Storage} = 91,
		CrowdloanClaim: pallet_crowdloan_claim::{Pallet, Call, Storage, Event<T>} = 93,
		CrowdloanReward: pallet_crowdloan_reward::{Pallet, Call, Storage, Event<T>} = 94,
		CollatorAllowlist: pallet_collator_allowlist::{Pallet, Call, Storage, Config<T>, Event<T>} = 95,
		Permissions: pallet_permissions::{Pallet, Call, Storage, Event<T>} = 96,
		Tokens: pallet_restricted_tokens::{Pallet, Call, Event<T>} = 97,
		NftSales: pallet_nft_sales::{Pallet, Call, Storage, Event<T>} = 98,
		Pools: pallet_pools::{Pallet, Call, Storage, Event<T>} = 99,
		Loans: pallet_loans::{Pallet, Call, Storage, Event<T>} = 100,
		InterestAccrual: pallet_interest_accrual::{Pallet, Storage, Event<T>, Config<T>} = 101,

		// XCM
		XcmpQueue: cumulus_pallet_xcmp_queue::{Pallet, Call, Storage, Event<T>} = 120,
		PolkadotXcm: pallet_xcm::{Pallet, Call, Event<T>, Origin} = 121,
		CumulusXcm: cumulus_pallet_xcm::{Pallet, Event<T>, Origin} = 122,
		DmpQueue: cumulus_pallet_dmp_queue::{Pallet, Call, Storage, Event<T>} = 123,
		XTokens: orml_xtokens::{Pallet, Storage, Call, Event<T>} = 124,

		// 3rd party pallets
		OrmlTokens: orml_tokens::{Pallet, Storage, Event<T>, Config<T>} = 150,
		OrmlAssetRegistry: orml_asset_registry::{Pallet, Storage, Call, Event<T>, Config<T>} = 151,
		OrmlXcm: orml_xcm::{Pallet, Storage, Call, Event<T>} = 152,

		// migration pallet
		Migration: pallet_migration_manager::{Pallet, Call, Storage, Event<T>} = 199,

	}
);

/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
/// A Block signed with a Justification
pub type SignedBlock = generic::SignedBlock<Block>;
/// BlockId type as expected by this runtime.
pub type BlockId = generic::BlockId<Block>;
/// The SignedExtension to the basic transaction logic.
pub type SignedExtra = (
	frame_system::CheckNonZeroSender<Runtime>,
	frame_system::CheckSpecVersion<Runtime>,
	frame_system::CheckTxVersion<Runtime>,
	frame_system::CheckGenesis<Runtime>,
	frame_system::CheckEra<Runtime>,
	frame_system::CheckNonce<Runtime>,
	frame_system::CheckWeight<Runtime>,
	pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
);
/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic = generic::UncheckedExtrinsic<Address, Call, Signature, SignedExtra>;
/// Extrinsic type that has already been checked.
pub type CheckedExtrinsic = generic::CheckedExtrinsic<AccountId, Call, SignedExtra>;

/// Executive: handles dispatch to the various modules.
pub type Executive = frame_executive::Executive<
	Runtime,
	Block,
	frame_system::ChainContext<Runtime>,
	Runtime,
	AllPalletsWithSystem,
	upgrade::Upgrade,
>;

/// Runtime upgrade logic
mod upgrade {
	use frame_support::{traits::OnRuntimeUpgrade, weights::Weight};

	use super::*;

	pub struct Upgrade;
	impl OnRuntimeUpgrade for Upgrade {
		fn on_runtime_upgrade() -> Weight {
			let mut weight = 0;
			weight += InterestAccrual::upgrade_to_v1();
			weight += Loans::reference_active_rates();
			weight += InterestAccrual::remove_unused_rates();
			weight += pallet_anchors::migration::fix_evict_date::migrate::<Runtime>();
			weight
		}

		#[cfg(feature = "try-runtime")]
		fn pre_upgrade() -> Result<(), &'static str> {
			pallet_anchors::migration::fix_evict_date::pre_migrate::<Runtime>()
		}

		#[cfg(feature = "try-runtime")]
		fn post_upgrade() -> Result<(), &'static str> {
			pallet_anchors::migration::fix_evict_date::post_migrate::<Runtime>()
		}
	}
}

#[cfg(not(feature = "disable-runtime-api"))]
impl_runtime_apis! {
	impl sp_api::Core<Block> for Runtime {
		fn version() -> RuntimeVersion {
			VERSION
		}

		fn execute_block(block: Block) {
			Executive::execute_block(block)
		}

		fn initialize_block(header: &<Block as BlockT>::Header) {
			Executive::initialize_block(header)
		}
	}

	impl sp_api::Metadata<Block> for Runtime {
		fn metadata() -> OpaqueMetadata {
			OpaqueMetadata::new(Runtime::metadata().into())
		}
	}

	impl sp_block_builder::BlockBuilder<Block> for Runtime {
		fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
			Executive::apply_extrinsic(extrinsic)
		}

		fn finalize_block() -> <Block as BlockT>::Header {
			Executive::finalize_block()
		}

		fn inherent_extrinsics(data: InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
			data.create_extrinsics()
		}

		fn check_inherents(block: Block, data: InherentData) -> CheckInherentsResult {
			data.check_extrinsics(&block)
		}
	}

	impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
		fn validate_transaction(
			source: TransactionSource,
			tx: <Block as BlockT>::Extrinsic,
			block_hash: <Block as BlockT>::Hash,
		) -> TransactionValidity {
			Executive::validate_transaction(source, tx, block_hash)
		}
	}

	impl sp_offchain::OffchainWorkerApi<Block> for Runtime {
		fn offchain_worker(header: &<Block as BlockT>::Header) {
			Executive::offchain_worker(header)
		}
	}

	impl sp_session::SessionKeys<Block> for Runtime {
		fn decode_session_keys(
			encoded: Vec<u8>,
		) -> Option<Vec<(Vec<u8>, sp_core::crypto::KeyTypeId)>> {
			SessionKeys::decode_into_raw_public_keys(&encoded)
		}

		fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
			SessionKeys::generate(seed)
		}
	}

	impl sp_consensus_aura::AuraApi<Block, AuraId> for Runtime {
		fn slot_duration() -> sp_consensus_aura::SlotDuration {
			sp_consensus_aura::SlotDuration::from_millis(Aura::slot_duration())
		}

		fn authorities() -> Vec<AuraId> {
			Aura::authorities().into_inner()
		}
	}

	impl frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Index> for Runtime {
		fn account_nonce(account: AccountId) -> Index {
			System::account_nonce(account)
		}
	}

	impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<
		Block,
		Balance,
	> for Runtime {
		fn query_info(uxt: <Block as BlockT>::Extrinsic, len: u32) -> RuntimeDispatchInfo<Balance> {
			TransactionPayment::query_info(uxt, len)
		}
		fn query_fee_details(uxt: <Block as BlockT>::Extrinsic, len: u32) -> FeeDetails<Balance> {
			TransactionPayment::query_fee_details(uxt, len)
		}
	}

	impl cumulus_primitives_core::CollectCollationInfo<Block> for Runtime {
		fn collect_collation_info(header: &<Block as BlockT>::Header) -> cumulus_primitives_core::CollationInfo {
			ParachainSystem::collect_collation_info(header)
		}
	}

	impl runtime_common::apis::AnchorApi<Block, Hash, BlockNumber> for Runtime {
		fn get_anchor_by_id(id: Hash) -> Option<AnchorData<Hash, BlockNumber>> {
			Anchor::get_anchor_by_id(id)
		}
	}

	impl runtime_common::apis::PoolsApi<Block, PoolId, TrancheId, Balance, CurrencyId, Rate> for Runtime {
		fn currency(pool_id: PoolId) -> Option<CurrencyId>{
			pallet_pools::Pool::<Runtime>::get(pool_id).map(|details| details.currency)
		}

		fn inspect_epoch_solution(pool_id: PoolId, solution: Vec<TrancheSolution>) -> Option<EpochSolution<Balance>>{
			let pool = pallet_pools::Pool::<Runtime>::get(pool_id)?;
			let epoch_execution_info = pallet_pools::EpochExecution::<Runtime>::get(pool_id)?;
			pallet_pools::Pallet::<Runtime>::score_solution(
				&pool,
				&epoch_execution_info,
				&solution
			).ok()
		}

		fn tranche_token_price(pool_id: PoolId, tranche: TrancheLoc<TrancheId>) -> Option<Rate>{
			let now = <pallet_timestamp::Pallet::<Runtime> as UnixTime>::now().as_secs();
			let mut pool = pallet_pools::Pool::<Runtime>::get(pool_id)?;
			let nav: Balance = pallet_loans::Pallet::<Runtime>::update_nav_of_pool(pool_id)
				.ok()
				.map(|(latest, _)| latest.into())?;
			let total_assets = pool.reserve.total.saturating_add(nav);
			let index: usize = pool.tranches.tranche_index(&tranche)?.try_into().ok()?;
			let prices = pool
				.tranches
				.calculate_prices::<_, OrmlTokens, _>(total_assets, now)
				.ok()?;
			prices.get(index).map(|rate: &Rate| rate.clone())
		}

		fn tranche_token_prices(pool_id: PoolId) -> Option<Vec<Rate>>{
			let now = <pallet_timestamp::Pallet::<Runtime> as UnixTime>::now().as_secs();
			let mut pool = pallet_pools::Pool::<Runtime>::get(pool_id)?;
			let nav: Balance = pallet_loans::Pallet::<Runtime>::update_nav_of_pool(pool_id)
				.ok()
				.map(|(latest, _)| latest.into())?;
			let total_assets = pool.reserve.total.saturating_add(nav);
			pool
				.tranches
				.calculate_prices::<Rate, OrmlTokens, AccountId>(total_assets, now)
				.ok()
		}

		fn tranche_ids(pool_id: PoolId) -> Option<Vec<TrancheId>>{
			let pool = pallet_pools::Pool::<Runtime>::get(pool_id)?;
			Some(pool.tranches.ids_residual_top())
		}

		fn tranche_id(pool_id: PoolId, tranche_index: TrancheIndex) -> Option<TrancheId>{
			let pool = pallet_pools::Pool::<Runtime>::get(pool_id)?;
			let index: usize = tranche_index.try_into().ok()?;
			pool.tranches.ids_residual_top().get(index).map(|id| id.clone())
		}

		fn tranche_currency(pool_id: PoolId, tranche_loc: TrancheLoc<TrancheId>) -> Option<CurrencyId>{
			let pool = pallet_pools::Pool::<Runtime>::get(pool_id)?;
			pool.tranches.tranche_currency(tranche_loc)
		}
	}

	#[cfg(feature = "runtime-benchmarks")]
	impl frame_benchmarking::Benchmark<Block> for Runtime {

		fn benchmark_metadata(extra: bool) -> (
			Vec<frame_benchmarking::BenchmarkList>,
			Vec<frame_support::traits::StorageInfo>,
		) {
			use frame_benchmarking::{list_benchmark, Benchmarking, BenchmarkList};
			use frame_support::traits::StorageInfoTrait;
			use frame_system_benchmarking::Pallet as SystemBench;
			use pallet_loans::benchmarking::Pallet as LoansPallet;

			let mut list = Vec::<BenchmarkList>::new();

			list_benchmark!(list, extra, frame_system, SystemBench::<Runtime>);
			list_benchmark!(list, extra, pallet_timestamp, Timestamp);
			list_benchmark!(list, extra, pallet_balances, Balances);
			// TODO: Not working as benches expect everbody to be whitelisted to register
			//       as collator. But our runtimes restrict this. A PR to the cumulus
			//       benches is needed or benchmarks allow some kind of pre-setup logic
			// list_benchmark!(list, extra, pallet_collator_selection, CollatorSelection);
			// TODO: Not working as benches depend on pallet-staking which we don't use
			//       Not sure how to fix TBH.
			// use pallet_session_benchmarking::Pallet as SessionBench;
			// list_benchmark!(list, extra, pallet_session, SessionBench::<Runtime>);
			list_benchmark!(list, extra, pallet_multisig, Multisig);
			list_benchmark!(list, extra, pallet_proxy, Proxy);
			list_benchmark!(list, extra, pallet_utility, Utility);
			list_benchmark!(list, extra, pallet_scheduler, Scheduler);
			list_benchmark!(list, extra, pallet_collective, Council);
			// TODO: Fails for reason: Error: Input("failed to submit candidacy")
			// list_benchmark!(list, extra, pallet_elections_phragmen, Elections);
			list_benchmark!(list, extra, pallet_democracy, Democracy);
			list_benchmark!(list, extra, pallet_identity, Identity);
			list_benchmark!(list, extra, pallet_vesting, Vesting);
			list_benchmark!(list, extra, pallet_treasury, Treasury);
			list_benchmark!(list, extra, pallet_preimage, Preimage);
			list_benchmark!(list, extra, pallet_uniques, Uniques);
			list_benchmark!(list, extra, pallet_fees, Fees);
			list_benchmark!(list, extra, pallet_anchors, Anchor);
			list_benchmark!(list, extra, pallet_crowdloan_claim, CrowdloanClaim);
			list_benchmark!(list, extra, pallet_crowdloan_reward, CrowdloanReward);
			list_benchmark!(list, extra, pallet_collator_allowlist, CollatorAllowlist);
			list_benchmark!(list, extra, pallet_migration_manager, Migration);
			list_benchmark!(list, extra, pallet_permissions, Permissions);
			// list_benchmark!(list, extra, pallet_restricted_tokens, Tokens);
			list_benchmark!(list, extra, pallet_nft_sales, NftSales);
			list_benchmark!(list, extra, pallet_pools, Pools);
			list_benchmark!(list, extra, pallet_loans, LoansPallet::<Runtime>);
			list_benchmark!(list, extra, pallet_interest_accrual, InterestAccrual);

			let storage_info = AllPalletsWithSystem::storage_info();

			return (list, storage_info)
		}

		fn dispatch_benchmark(
				config: frame_benchmarking::BenchmarkConfig
		) -> Result<Vec<frame_benchmarking::BenchmarkBatch>, sp_runtime::RuntimeString>{
			use frame_benchmarking::{Benchmarking, BenchmarkBatch, TrackedStorageKey, add_benchmark};
			use frame_system_benchmarking::Pallet as SystemBench;

			impl frame_system_benchmarking::Config for Runtime {}

			// you can whitelist any storage keys you do not want to track here
			let whitelist: Vec<TrackedStorageKey> = vec![
				// Block Number
				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef702a5c1b19ab7a04f536c519aca4983ac").to_vec().into(),
				// Total Issuance
				hex_literal::hex!("c2261276cc9d1f8598ea4b6a74b15c2f57c875e4cff74148e4628f264b974c80").to_vec().into(),
				// Execution Phase
				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef7ff553b5a9862a516939d82b3d3d8661a").to_vec().into(),
				// Event Count
				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef70a98fdbe9ce6c55837576c60c7af3850").to_vec().into(),
				// System Events
				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef780d41e5e16056765bc8461851072c9d7").to_vec().into(),
			];

			let mut batches = Vec::<BenchmarkBatch>::new();
			let params = (&config, &whitelist);

			use pallet_loans::benchmarking::Pallet as LoansPallet;
			impl pallet_loans::benchmarking::Config for Runtime {}

			// It should be called Anchors to make the runtime_benchmarks.sh script works
			type Anchors = Anchor;

			// Note: Only add working benches here. Commenting out will still
			//       result in the runtime_benchmarks.sh script trying to
			//       the benches for the given pallet.
			add_benchmark!(params, batches, frame_system, SystemBench::<Runtime>);
			add_benchmark!(params, batches, pallet_timestamp, Timestamp);
			add_benchmark!(params, batches, pallet_balances, Balances);
			add_benchmark!(params, batches, pallet_multisig, Multisig);
			add_benchmark!(params, batches, pallet_proxy, Proxy);
			add_benchmark!(params, batches, pallet_utility, Utility);
			add_benchmark!(params, batches, pallet_scheduler, Scheduler);
			add_benchmark!(params, batches, pallet_collective, Council);
			add_benchmark!(params, batches, pallet_democracy, Democracy);
			add_benchmark!(params, batches, pallet_identity, Identity);
			add_benchmark!(params, batches, pallet_vesting, Vesting);
			add_benchmark!(params, batches, pallet_treasury, Treasury);
			add_benchmark!(params, batches, pallet_preimage, Preimage);
			add_benchmark!(params, batches, pallet_uniques, Uniques);
			add_benchmark!(params, batches, pallet_fees, Fees);
			add_benchmark!(params, batches, pallet_anchors, Anchors);
			add_benchmark!(params, batches, pallet_crowdloan_claim, CrowdloanClaim);
			add_benchmark!(params, batches, pallet_crowdloan_reward, CrowdloanReward);
			add_benchmark!(params, batches, pallet_collator_allowlist, CollatorAllowlist);
			add_benchmark!(params, batches, pallet_migration_manager, Migration);
			add_benchmark!(params, batches, pallet_permissions, Permissions);
			add_benchmark!(params, batches, pallet_nft_sales, NftSales);
			add_benchmark!(params, batches, pallet_pools, Pools);
			add_benchmark!(params, batches, pallet_loans, LoansPallet::<Runtime>);
			add_benchmark!(params, batches, pallet_interest_accrual, InterestAccrual);

			if batches.is_empty() { return Err("Benchmark not found for this pallet.".into()) }
			Ok(batches)
		}
	}

	#[cfg(feature = "try-runtime")]
	impl frame_try_runtime::TryRuntime<Block> for Runtime {
		fn on_runtime_upgrade() -> (Weight, Weight) {
			let weight = Executive::try_runtime_upgrade().unwrap();
			(weight, RuntimeBlockWeights::get().max_block)
		}
		fn execute_block_no_check(block: Block) -> Weight {
			Executive::execute_block_no_check(block)
		}
	}
}

struct CheckInherents;

impl cumulus_pallet_parachain_system::CheckInherents<Block> for CheckInherents {
	fn check_inherents(
		block: &Block,
		relay_state_proof: &cumulus_pallet_parachain_system::RelayChainStateProof,
	) -> sp_inherents::CheckInherentsResult {
		let relay_chain_slot = relay_state_proof
			.read_slot()
			.expect("Could not read the relay chain slot from the proof");

		let inherent_data =
			cumulus_primitives_timestamp::InherentDataProvider::from_relay_chain_slot_and_duration(
				relay_chain_slot,
				sp_std::time::Duration::from_secs(6),
			)
			.create_inherent_data()
			.expect("Could not create the timestamp inherent data");

		inherent_data.check_extrinsics(&block)
	}
}

cumulus_pallet_parachain_system::register_validate_block! {
	Runtime = Runtime,
	BlockExecutor = cumulus_pallet_aura_ext::BlockExecutor::<Runtime, Executive>,
	CheckInherents = CheckInherents,
}
