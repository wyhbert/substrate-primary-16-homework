//! # Template Pallet
//!
//! A pallet with minimal functionality to help developers understand the essential components of
//! writing a FRAME pallet. It is typically used in beginner tutorials or in Substrate template
//! nodes as a starting point for creating a new pallet and **not meant to be used in production**.
//!
//! ## Overview
//!
//! This template pallet contains basic examples of:
//! - declaring a storage item that stores a single `u32` value
//! - declaring and using events
//! - declaring and using errors
//! - a dispatchable function that allows a user to set a new value to storage and emits an event
//!   upon success
//! - another dispatchable function that causes a custom error to be thrown
//!
//! Each pallet section is annotated with an attribute using the `#[pallet::...]` procedural macro.
//! This macro generates the necessary code for a pallet to be aggregated into a FRAME runtime.
//!
//! Learn more about FRAME macros [here](https://docs.substrate.io/reference/frame-macros/).
//!
//! ### Pallet Sections
//!
//! The pallet sections in this template are:
//!
//! - A **configuration trait** that defines the types and parameters which the pallet depends on
//!   (denoted by the `#[pallet::config]` attribute). See: [`Config`].
//! - A **means to store pallet-specific data** (denoted by the `#[pallet::storage]` attribute).
//!   See: [`storage_types`].
//! - A **declaration of the events** this pallet emits (denoted by the `#[pallet::event]`
//!   attribute). See: [`Event`].
//! - A **declaration of the errors** that this pallet can throw (denoted by the `#[pallet::error]`
//!   attribute). See: [`Error`].
//! - A **set of dispatchable functions** that define the pallet's functionality (denoted by the
//!   `#[pallet::call]` attribute). See: [`dispatchables`].
//!
//! Run `cargo doc --package pallet-template --open` to view this pallet's documentation.

// We make sure this pallet uses `no_std` for compiling to Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

// FRAME pallets require their own "mock runtimes" to be able to run unit tests. This module
// contains a mock runtime specific for testing this pallet's functionality.
#[cfg(test)]
mod mock;

// This module contains the unit tests for this pallet.
// Learn about pallet unit testing here: https://docs.substrate.io/test/unit-testing/
#[cfg(test)]
mod tests;

// Every callable function or "dispatchable" a pallet exposes must have weight values that correctly
// estimate a dispatchable's execution time. The benchmarking module is used to calculate weights
// for each dispatchable and generates this pallet's weight.rs file. Learn more about benchmarking here: https://docs.substrate.io/test/benchmark/
/* 
#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;
*/

// All pallet logic is defined in its own module and must be annotated by the `pallet` attribute.
#[frame_support::pallet]
pub mod pallet {
    // Import various useful types required by all FRAME pallets.
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    // The `Pallet` struct serves as a placeholder to implement traits, methods and dispatchables
    // (`Call`s) in this pallet.
    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// The pallet's configuration trait.
    ///
    /// All our types and constants a pallet depends on must be declared here.
    /// These types are defined generically and made concrete when the pallet is declared in the
    /// `runtime/src/lib.rs` file of your chain.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        #[pallet::constant]
        type MaxClaimLength: Get<u32>;
        /// The overarching runtime event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    
    }
    #[pallet::storage]
    pub type Proofs<T:Config> = 
        StorageMap<_, Blake2_128Concat, BoundedVec<u8,T::MaxClaimLength>, 
        (T::AccountId, BlockNumberFor<T>,
        bool)>;

    /// Events that functions in this pallet can emit.
    ///
    /// Events are a simple means of indicating to the outside world (such as dApps, chain explorers
    /// or other users) that some notable update in the runtime has occurred. In a FRAME pallet, the
    /// documentation for each event field and its parameters is added to a node's metadata so it
    /// can be used by external interfaces or tools.
    ///
    ///	The `generate_deposit` macro generates a function on `Pallet` called `deposit_event` which
    /// will convert the event type of your pallet into `RuntimeEvent` (declared in the pallet's
    /// [`Config`] trait) and deposit it using [`frame_system::Pallet::deposit_event`].
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A user has successfully set a new value.
        ClaimCreated {
            /// The new value set.
            //something: u32,
            /// The account who set the new value.
            //who: T::AccountId,
            owner:T::AccountId, 
            claim:BoundedVec<u8, T::MaxClaimLength>,
        },
        ClaimRevoked{ 
            owner: T::AccountId,
            claim: BoundedVec<u8, T::MaxClaimLength> 
        }, 
        ClaimTransferred { 
            old_owner: T::AccountId, 
            new_owner: T::AccountId, 
            claim: BoundedVec<u8, T::MaxClaimLength> 
        },


    }

    /// Errors that can be returned by this pallet.
    ///
    /// Errors tell users that something went wrong so it's important that their naming is
    /// informative. Similar to events, error documentation is added to a node's metadata so it's
    /// equally important that they have helpful documentation associated with them.
    ///
    /// This type of runtime error can be up to 4 bytes in size should you want to return additional
    /// information.
    #[pallet::error]
    pub enum Error<T> {
        /// The value retrieved was `None` as no value was previously set.
        NoneValue,
        /// There was an attempt to increment the value in storage over `u32::MAX`.
        StorageOverflow,
        ProofAlreadyExist,
        ProofNotExist,
        NotProofOwner,
        ProofAlreadyRevoked,
        CannotTransferToSelf,
    }

    /// The pallet's dispatchable functions ([`Call`]s).
    ///
    /// Dispatchable functions allows users to interact with the pallet and invoke state changes.
    /// These functions materialize as "extrinsics", which are often compared to transactions.
    /// They must always return a `DispatchResult` and be annotated with a weight and call index.
    ///
    /// The [`call_index`] macro is used to explicitly
    /// define an index for calls in the [`Call`] enum. This is useful for pallets that may
    /// introduce new dispatchables over time. If the order of a dispatchable changes, its index
    /// will also change which will break backwards compatibility.
    ///
    /// The [`weight`] macro is used to assign a weight to each call.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
     
        #[pallet::call_index(0)]
        #[pallet::weight({0})]
        pub fn create_claim(origin: OriginFor<T>,claim:BoundedVec<u8,T::MaxClaimLength>)
         -> DispatchResult
         {
            //let  who:<T as Config>::AccountId = ensure_signed(origin)?;
            let  who = ensure_signed(origin)?;
            ensure!{
                !Proofs::<T>::contains_key(&claim),
                Error::<T>::ProofAlreadyExist
            };
            Proofs::<T>::insert
            (
                &claim,
                (who.clone(),frame_system::Pallet::<T>::block_number(),true),
            );
            // 打印存储内容以便调试
            
            Self::deposit_event(Event::ClaimCreated{ owner: who, claim});     
            Ok(())
         }
         
         #[pallet::call_index(1)]
         #[pallet::weight({0})]
         pub fn revoke_claim(
             origin: OriginFor<T>,
             claim: BoundedVec<u8, T::MaxClaimLength>,
         ) -> DispatchResult {
            // 验证调用者身份
            let who = ensure_signed(origin)?;
         
            // 确保该 PoE 数据存在
            ensure!(
                Proofs::<T>::contains_key(&claim),
                Error::<T>::ProofNotExist
            );
         
        // 确保调用者是数据的所有者
            let (owner, _, is_active) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ProofNotExist)?;
            ensure!(who == owner, Error::<T>::NotProofOwner);
         
            // 确保数据当前是有效状态
            ensure!(is_active, Error::<T>::ProofAlreadyRevoked);
         
            // 更新状态为无效
            Proofs::<T>::insert(&claim, (who.clone(), frame_system::Pallet::<T>::block_number(), false));
         
            // 触发撤回事件
            Self::deposit_event(Event::ClaimRevoked { owner: who, claim });
         
            Ok(())
        }
    
        #[pallet::call_index(2)]
        #[pallet::weight({0})]
        pub fn transfer_claim(
            origin: OriginFor<T>,
            claim: BoundedVec<u8, T::MaxClaimLength>,
            new_owner: T::AccountId,
        ) -> DispatchResult {
            // 验证调用者签名
            let sender = ensure_signed(origin)?;
    
            // 校验数据是否存在
            let (current_owner, block_number, _) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ProofNotExist)?;
    
            // 确保调用者是当前所有者
            ensure!(current_owner == sender, Error::<T>::NotProofOwner);
    
            // 确保新所有者不同于当前所有者
            ensure!(current_owner != new_owner, Error::<T>::CannotTransferToSelf);
    
            // 更新存储，将所有权转移给新所有者
            Proofs::<T>::insert(&claim, (new_owner.clone(), block_number, true));
    
            // 触发事件
            Self::deposit_event(Event::ClaimTransferred {
                old_owner: sender,
                new_owner,
                claim,
            });
    
            Ok(())
        }
    }

}