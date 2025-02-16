#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;


use frame_support::{
    pallet_prelude::*,
};
use frame_system::pallet_prelude::*;
use parity_scale_codec::{Encode, Decode, Codec, EncodeLike};
use sp_runtime::traits::AtLeast32BitUnsigned;

const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type TaskId: Member + Default + Copy + AtLeast32BitUnsigned + MaxEncodedLen + Codec + TypeInfo + EncodeLike;
    }
    
    #[pallet::pallet]
    #[pallet::storage_version(STORAGE_VERSION)]
    pub struct Pallet<T>(_);
    
    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct Task<T: Config> {
        pub title: BoundedVec<u8, ConstU32<256>>,
        pub description: BoundedVec<u8, ConstU32<1024>>,
        pub priority: u8,
        pub completed: bool,
        pub created_at: BlockNumberFor<T>,
    }

    #[pallet::storage]
    #[pallet::getter(fn tasks)]
    pub type Tasks<T: Config> = StorageMap<_, Blake2_128Concat, T::TaskId, Task<T>>;

    #[pallet::storage]
    #[pallet::getter(fn task_count)]
    pub type TaskCount<T: Config> = StorageValue<_, T::TaskId, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        TaskCreated { task_id: T::TaskId, title: Vec<u8>, priority: u8 },
        TaskCompleted { task_id: T::TaskId },
        TaskDeleted { task_id: T::TaskId },
    }

    #[pallet::error]
    pub enum Error<T> {
        TaskNotFound,
        AlreadyCompleted,
        InvalidPriority,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(T::DbWeight::get().reads_writes(1, 1))]
        pub fn create_task(
            origin: OriginFor<T>,
            title: Vec<u8>,
            description: Vec<u8>,
            priority: u8,
        ) -> DispatchResult {
            ensure_signed(origin)?;
            
            let priority = priority.clamp(1, 5);
            let task_id = Self::task_count();
            
            // Clone title before conversion
            let bounded_title: BoundedVec<u8, ConstU32<256>> = title
                .clone()
                .try_into()
                .map_err(|_| Error::<T>::InvalidPriority)?;
        
            let task = Task {
                title: bounded_title,
                description: description.try_into().map_err(|_| Error::<T>::InvalidPriority)?,
                priority,
                completed: false,
                created_at: frame_system::Pallet::<T>::block_number(),
            };
        
            <Tasks<T>>::insert(task_id, task);
            <TaskCount<T>>::put(task_id + T::TaskId::one());
        
            Self::deposit_event(Event::TaskCreated {
                task_id,
                title,  // Now we can use the original title
                priority,
            });
        
            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(T::DbWeight::get().reads_writes(1, 1))]
        pub fn update_priority(
            origin: OriginFor<T>,
            task_id: T::TaskId,
            new_priority: u8,
        ) -> DispatchResult {
            ensure_signed(origin)?;
            
            let mut task = <Tasks<T>>::get(task_id).ok_or(Error::<T>::TaskNotFound)?;
            task.priority = new_priority.clamp(1, 5);
            <Tasks<T>>::insert(task_id, task);

            Ok(())
        }

        #[pallet::call_index(2)]
        #[pallet::weight(T::DbWeight::get().reads_writes(1, 1))]
        pub fn complete_task(
            origin: OriginFor<T>,
            task_id: T::TaskId,
        ) -> DispatchResult {
            ensure_signed(origin)?;
            
            let mut task = <Tasks<T>>::get(task_id).ok_or(Error::<T>::TaskNotFound)?;
            
            ensure!(!task.completed, Error::<T>::AlreadyCompleted);
            task.completed = true;
            
            <Tasks<T>>::insert(task_id, task);
            Self::deposit_event(Event::TaskCompleted { task_id });

            Ok(())
        }

        #[pallet::call_index(3)]
        #[pallet::weight(T::DbWeight::get().reads_writes(1, 1))]
        pub fn delete_task(
            origin: OriginFor<T>,
            task_id: T::TaskId,
        ) -> DispatchResult {
            ensure_signed(origin)?;
            
            ensure!(<Tasks<T>>::contains_key(task_id), Error::<T>::TaskNotFound);
            <Tasks<T>>::remove(task_id);
            Self::deposit_event(Event::TaskDeleted { task_id });

            Ok(())
        }

        #[pallet::call_index(4)]
        #[pallet::weight(T::DbWeight::get().reads_writes(1, 1))]
        pub fn update_task(
            origin: OriginFor<T>,
            task_id: T::TaskId,
            new_title: Option<Vec<u8>>,
            new_priority: Option<u8>,
        ) -> DispatchResult {
            ensure_signed(origin)?;
            
            let mut task = <Tasks<T>>::get(task_id).ok_or(Error::<T>::TaskNotFound)?;

            if let Some(title) = new_title {
                task.title = title.try_into().map_err(|_| Error::<T>::InvalidPriority)?;
            }

            if let Some(priority) = new_priority {
                task.priority = priority.clamp(1, 5);
            }

            <Tasks<T>>::insert(task_id, task);

            Ok(())
        }
    }
}