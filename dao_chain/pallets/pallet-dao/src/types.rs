use crate::Config;
use codec::MaxEncodedLen;
use frame_support::{
	codec::{Decode, Encode},
	traits::{ConstU32, Currency},
	BoundedVec, RuntimeDebug,
};
use scale_info::TypeInfo;

pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
pub type CurrencyOf<T> = <T as Config>::Currency;
pub type DepositBalanceOf<T> = <CurrencyOf<T> as Currency<AccountIdOf<T>>>::Balance;
pub type AssetIdOf<T> = <T as Config>::AssetId;
pub type DaoNameOf<T> = BoundedVec<u8, <T as Config>::MaxDaoNameLength>;
pub type MetadataOf<T> = BoundedVec<u8, <T as Config>::MaxLengthMetadata>;
pub type DaoInfo<T> = Dao<AccountIdOf<T>, DaoNameOf<T>, AssetIdOf<T>, MetadataOf<T>>;
pub type BalanceOf<T> = <CurrencyOf<T> as Currency<AccountIdOf<T>>>::Balance;

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
pub struct Dao<AccountId, DaoName, AssetId, Metadata> {
	pub id: u128,
	pub owner: AccountId,
	pub name: DaoName,
	pub asset_id: Option<AssetId>,
	pub meta: Metadata,
	pub meta_hash: BoundedVec<u8, ConstU32<64>>,
}
