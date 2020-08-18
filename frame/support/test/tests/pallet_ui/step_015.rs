#[frame_support::pallet(Example)]
mod pallet {
	use frame_support::pallet_prelude::*;

	#[pallet::trait_]
	pub trait Trait {}

	#[pallet::module]
	pub struct Module<T> {}

	#[pallet::module_interface]
	impl<T: Trait> ModuleInterface for Module<T> {}

	#[pallet::call]
	pub enum Call {}
}

fn main() {
}