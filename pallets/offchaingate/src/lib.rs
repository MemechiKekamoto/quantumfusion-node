#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::{
		offchain::{
			AppCrypto, CreateSignedTransaction, SendSignedTransaction, SendUnsignedTransaction,
			SignedPayload, Signer, SigningTypes,
		},
		pallet_prelude::*,
	};
	use sp_core::crypto::KeyTypeId;
	use sp_runtime::{
		transaction_validity::{InvalidTransaction, TransactionValidity, ValidTransaction},
		RuntimeDebug,
	};

	pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"gate");
	const UNSIGNED_TXS_PRIORITY: u64 = 100;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	pub mod weights {
		pub struct SubstrateWeight<T>(core::marker::PhantomData<T>);
	}

	pub mod crypto {
		use super::KEY_TYPE;
		use sp_runtime::{
			app_crypto::{app_crypto, sr25519},
			MultiSignature, MultiSigner
		};
		app_crypto!(sr25519, KEY_TYPE);
	
		pub struct TestAuthId;
	
		// implemented for runtime
		impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for TestAuthId {
			type RuntimeAppPublic = Public;
			type GenericSignature = sp_core::sr25519::Signature;
			type GenericPublic = sp_core::sr25519::Public;
		}
	}

	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, scale_info::TypeInfo)]
	pub struct Payload<Public> {
		calldata: u64,
		public: Public,
	}

	impl<T: SigningTypes> SignedPayload<T> for Payload<T::Public> {
		fn public(&self) -> T::Public {
		self.public.clone()
	}
}

	#[pallet::config]
	pub trait Config: CreateSignedTransaction<Call<Self>> + frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type WeightInfo;
		type AuthorityId: AppCrypto<Self::Public, Self::Signature>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		CalldataApproved { who: T::AccountId, calldata: T::Hash },
		CalldataDisproved { who: T::AccountId, calldata: T::Hash },
	}

	#[pallet::error]
	pub enum Error<T> {
		ApproveFailed,
		InvalidCalldata,
		InvalidPayload,
		NotOwner,
	}

	#[pallet::storage]
	pub(super) type Calldata<T: Config> = StorageMap<_, Blake2_128Concat, T::Hash, (T::AccountId, BlockNumberFor<T>)>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(Weight::default())]
		pub fn verify_calldata(origin: OriginFor<T>, calldata: T::Hash) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			ensure!(!Calldata::<T>::contains_key(&calldata), Error::<T>::InvalidCalldata);
			
			let current_block = <frame_system::Pallet<T>>::block_number();
			
			Calldata::<T>::insert(&calldata, (&sender, current_block));

			Self::deposit_event(Event::CalldataApproved { who: sender, calldata });

			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(Weight::default())]
		pub fn cause_error(origin: OriginFor<T>, calldata: T::Hash) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let (owner, _) = Calldata::<T>::get(&calldata).ok_or(Error::<T>::InvalidCalldata)?;
			
			ensure!(sender == owner, Error::<T>::NotOwner);

			Calldata::<T>::remove(&calldata);

			Self::deposit_event(Event::CalldataDisproved { who: sender, calldata });
   			
			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(Weight::default())]
		pub fn submit_calldata_signed(
			origin: OriginFor<T>,
			calldata: u64
		) -> DispatchResult {
			let _ = ensure_signed(origin)?;
			log::info!("submit_calldata_signed: {}", calldata);
			Ok(())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(Weight::default())]
		pub fn submit_calldata_unsigned(
			origin: OriginFor<T>,
			calldata: u64
		) -> DispatchResult {
			let _ = ensure_none(origin)?;
			log::info!("submit_calldata_unsigned: {}", calldata);
			Ok(())
		}

		#[pallet::call_index(4)]
		#[pallet::weight(Weight::default())]
		pub fn submit_calldata_unsigned_with_signed_payload(
			origin: OriginFor<T>,
			payload: Payload<T::Public>,
			signature: T::Signature,
		) -> DispatchResult {
			let _ = ensure_none(origin)?;
			// Verify the payload
			if !SignedPayload::<T>::verify::<T::AuthorityId>(&payload, signature) {
				return Err(Error::<T>::InvalidPayload.into());
			}
			let calldata = payload.calldata;
			log::info!("submit_calldata_unsigned_with_signed_payload: {}", calldata);
			Ok(())
		}
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn offchain_worker(_block_number: BlockNumberFor<T>) {
			let calldata: u64 = 10; // Define the variable `calldata`
	
			let signer = Signer::<T, T::AuthorityId>::any_account();
	
			// if let Some((_, res)) = signer.send_signed_transaction(
			// 	|_acct| {
			// 		Call::submit_calldata_signed {
			// 			calldata,
			// 		}
			// 	},
			// ) {
			// 	match res {
			// 		Ok(()) => log::info!("unsigned tx with signed payload successfully sent."),
			// 		Err(()) => log::error!("sending unsigned tx with signed payload failed."),
			// 	};
			// } else {
			// 	log::error!("No local account available");
			// }

			if let Some((_, res)) = signer.send_unsigned_transaction(
				|acct| Payload { calldata, public: acct.public.clone() },
				|payload: Payload<<T as SigningTypes>::Public>, signature| {
					Call::submit_calldata_unsigned_with_signed_payload {
						payload,
						signature,
					}
				},
			) {
				match res {
					Ok(()) => log::info!("unsigned tx with signed payload successfully sent."),
					Err(()) => log::error!("sending unsigned tx with signed payload failed."),
				};
			} else {
				log::error!("No local account available");
			}
		}
	}
	
	#[pallet::validate_unsigned]
	impl<T: Config> ValidateUnsigned for Pallet<T> {
		type Call = Call<T>;

		fn validate_unsigned(_source: TransactionSource, call: &Self::Call) -> TransactionValidity {
			let valid_tx = |provide| ValidTransaction::with_tag_prefix("pallet-offchaingate")
				.priority(UNSIGNED_TXS_PRIORITY)
				.and_provides([&provide])
				.longevity(3)
				.propagate(true)
				.build();

			match call {
				Call::submit_calldata_unsigned { calldata: _calldata } => valid_tx(b"submit_calldata_unsigned".to_vec()),
				Call::submit_calldata_unsigned_with_signed_payload {
					ref payload,
					ref signature
				} => {
					if !SignedPayload::<T>::verify::<T::AuthorityId>(payload, signature.clone()) {
						return InvalidTransaction::BadProof.into();
					}
					valid_tx(b"submit_calldata_unsigned_with_signed_payload".to_vec())
				},
				_ => InvalidTransaction::Call.into(),
			}
		}
	}
}
