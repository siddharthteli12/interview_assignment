#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{
		dispatch::Vec,
		pallet_prelude::{OptionQuery, *},
	};
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
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(_block_number: BlockNumberFor<T>) -> Weight {
			let hash = <frame_system::Pallet<T>>::parent_hash();

			<Random<T>>::insert(hash, ());

			T::DbWeight::get().reads_writes(1, 1)
		}
	}

	#[pallet::storage]
	#[pallet::getter(fn random_material)]
	pub(super) type Random<T: Config> = StorageMap<_, Blake2_128Concat, T::Hash, (), OptionQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Random no. generated
		RandomNumberGenerated { hash: Vec<T::Hash> },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Generate random number.
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::generate_random())]
		pub fn generate_random(origin: OriginFor<T>) -> DispatchResult {
			let _ = ensure_signed(origin)?;
			let random_num: Vec<T::Hash> = Random::<T>::iter_keys().collect();
			Self::deposit_event(Event::RandomNumberGenerated { hash: random_num });
			Ok(())
		}
	}
}
