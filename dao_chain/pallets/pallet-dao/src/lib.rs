#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

pub use frame_support::{
	sp_runtime::traits::{One, Saturating},
	storage::bounded_vec::BoundedVec,
	traits::{
		tokens::fungibles::{metadata::Mutate as MetadataMutate, Mutate},
		Currency,
	},
	weights::Weight,
};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;
mod types;
pub use types::*;

pub use crate::types::{
	AccountIdOf, AssetIdOf, CurrencyOf, DaoInfo, DaoNameOf, DepositBalanceOf, MetadataOf,
};

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{dispatch::Vec, pallet_prelude::*, traits::ReservableCurrency};
	use frame_system::pallet_prelude::*;
	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Type representing the weight of this pallet
		type WeightInfo: WeightInfo;
		type Currency: ReservableCurrency<Self::AccountId>;
		type AssetId: Member
			+ Parameter
			+ Copy
			+ Default
			+ MaxEncodedLen
			+ One
			+ Saturating
			+ MaybeSerializeDeserialize
			+ MaxEncodedLen;

		#[pallet::constant]
		type MinLength: Get<u32>;

		#[pallet::constant]
		type MaxDaoNameLength: Get<u32>;

		#[pallet::constant]
		type MaxLengthMetadata: Get<u32>;

		#[pallet::constant]
		type DaoDeposit: Get<DepositBalanceOf<Self>>;
	}

	#[pallet::storage]
	#[pallet::getter(fn get_dao)]
	pub type Daos<T: Config> = StorageMap<_, Blake2_128Concat, u128, DaoInfo<T>>;

	#[pallet::storage]
	#[pallet::getter(fn get_current_dao_id)]
	pub type CurrentDaoId<T: Config> = StorageValue<_, u128, ValueQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// New Dao created.
		DaoCreated {
			dao_id: u128,
			owner: AccountIdOf<T>,
		},
		// Owners of Dao changed.
		DaoOwnerChanged {
			dao_id: u128,
			new_owner: AccountIdOf<T>,
		},
		/// Existing Dao Burned.
		DaoBurned {
			dao_id: u128,
		},
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		DaoNameLengthTooShort,
		DaoNameLengthTooLong,
		DaoDoesNotExist,
		DaoSignerNotOwner,
		DaoTokenAlreadyIssued,
		MetadataLengthTooLong,
		HashLengthIsInvalid,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create New Dao for caller.
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::create_dao())]
		pub fn create_dao(
			origin: OriginFor<T>,
			dao_name: Vec<u8>,
			meta_info: Vec<u8>,
			hash: Vec<u8>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let dao_name: BoundedVec<_, _> =
				dao_name.try_into().map_err(|_| Error::<T>::DaoNameLengthTooLong)?;

			ensure!(
				dao_name.len() >= T::MinLength::get() as usize,
				Error::<T>::DaoNameLengthTooShort
			);

			let meta_info: BoundedVec<_, _> =
				meta_info.try_into().map_err(|_| Error::<T>::MetadataLengthTooLong)?;

			let hash: BoundedVec<_, _> =
				hash.try_into().map_err(|_| Error::<T>::HashLengthIsInvalid)?;
			<T as Config>::Currency::reserve(&sender, <T as Config>::DaoDeposit::get())?;

			CurrentDaoId::<T>::mutate(|id| {
				*id += 1;
			});
			let dao_id = CurrentDaoId::<T>::get();
			let dao = Dao {
				id: dao_id.clone(),
				name: dao_name,
				owner: sender.clone(),
				asset_id: None,
				meta: meta_info,
				meta_hash: hash,
			};
			<Daos<T>>::insert(dao_id.clone(), dao);
			Self::deposit_event(Event::DaoCreated { dao_id, owner: sender.clone() });
			Ok(())
		}

		/// Delete existing dao if caller is owner.
		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::delete_dao())]
		pub fn delete_dao(origin: OriginFor<T>, dao_id: u128) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			let dao = Daos::<T>::get(dao_id).unwrap();
			ensure!(dao.owner == sender, Error::<T>::DaoSignerNotOwner);
			<T as Config>::Currency::unreserve(&sender, <T as Config>::DaoDeposit::get());
			Self::deposit_event(Event::DaoBurned { dao_id: dao.id.clone() });
			<Daos<T>>::remove(&dao.id);
			Ok(())
		}

		/// Delete existing dao if caller is owner.
		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::transfer_dao_ownership())]
		pub fn transfer_dao_ownership(
			origin: OriginFor<T>,
			dao_id: u128,
			to_account: AccountIdOf<T>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			let dao = Daos::<T>::get(dao_id).unwrap();
			ensure!(dao.owner == sender, Error::<T>::DaoSignerNotOwner);
			<T as Config>::Currency::unreserve(&sender, <T as Config>::DaoDeposit::get());
			<T as Config>::Currency::unreserve(&to_account, <T as Config>::DaoDeposit::get());
			Daos::<T>::mutate(dao_id, |dao_info| {
				let dao_info = dao_info.as_mut().unwrap();
				dao_info.owner = to_account.clone();
			});
			Self::deposit_event(Event::DaoOwnerChanged {
				dao_id: dao.id.clone(),
				new_owner: to_account,
			});
			Ok(())
		}
	}
}
