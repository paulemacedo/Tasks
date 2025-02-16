use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok, BoundedVec};

#[test]
fn create_task_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        let title = b"Test Task".to_vec();
        let description = b"Test Description".to_vec();
        let priority = 3;

        assert_ok!(TaskPallet::create_task(
            RuntimeOrigin::signed(1),
            title.clone(),
            description.clone(),
            priority
        ));

        let task = TaskPallet::tasks(0).unwrap();
        assert_eq!(task.priority, priority);
        assert_eq!(task.completed, false);

        System::assert_has_event(Event::TaskCreated { 
            task_id: 0, 
            title, 
            priority 
        }.into());
    });
}

#[test]
fn update_priority_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        // Create a task first
        let title = b"Task 1".to_vec();
        let description = b"Description".to_vec();
        assert_ok!(TaskPallet::create_task(
            RuntimeOrigin::signed(1),
            title,
            description,
            3
        ));

        // Update the priority
        assert_ok!(TaskPallet::update_priority(RuntimeOrigin::signed(1), 0, 5));
        
        // Verify the update
        let task = TaskPallet::tasks(0).unwrap();
        assert_eq!(task.priority, 5);
    });
}

#[test]
fn complete_task_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        // Create a task
        assert_ok!(TaskPallet::create_task(
            RuntimeOrigin::signed(1),
            b"Task 1".to_vec(),
            b"Description".to_vec(),
            3
        ));

        // Complete the task
        assert_ok!(TaskPallet::complete_task(RuntimeOrigin::signed(1), 0));
        
        // Verify completion
        let task = TaskPallet::tasks(0).unwrap();
        assert!(task.completed);
        
        // Verify event emission
        System::assert_has_event(Event::TaskCompleted { task_id: 0 }.into());
    });
}

#[test]
fn delete_task_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        // Create a task
        assert_ok!(TaskPallet::create_task(
            RuntimeOrigin::signed(1),
            b"Task 1".to_vec(),
            b"Description".to_vec(),
            3
        ));

        // Delete the task
        assert_ok!(TaskPallet::delete_task(RuntimeOrigin::signed(1), 0));
        
        // Verify deletion
        assert!(TaskPallet::tasks(0).is_none());
        
        // Verify event emission
        System::assert_has_event(Event::TaskDeleted { task_id: 0 }.into());
    });
}

#[test]
fn update_task_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        // Create a task
        assert_ok!(TaskPallet::create_task(
            RuntimeOrigin::signed(1),
            b"Task 1".to_vec(),
            b"Description".to_vec(),
            3
        ));

        // Update the task
        let new_title = b"Updated Task".to_vec();
        let new_priority = 4;
        
        assert_ok!(TaskPallet::update_task(
            RuntimeOrigin::signed(1),
            0,
            Some(new_title.clone()),
            Some(new_priority)
        ));
        
        // Verify updates
        let task = TaskPallet::tasks(0).unwrap();
        assert_eq!(task.title, BoundedVec::<u8, frame_support::traits::ConstU32<256>>::try_from(new_title).unwrap());
        assert_eq!(task.priority, new_priority);
    });
}

#[test]
fn fails_for_invalid_task() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        assert_noop!(
            TaskPallet::complete_task(RuntimeOrigin::signed(1), 0),
            Error::<Test>::TaskNotFound
        );

        assert_noop!(
            TaskPallet::delete_task(RuntimeOrigin::signed(1), 0),
            Error::<Test>::TaskNotFound
        );

        assert_noop!(
            TaskPallet::update_task(
                RuntimeOrigin::signed(1),
                0,
                Some(b"New Title".to_vec()),
                Some(4)
            ),
            Error::<Test>::TaskNotFound
        );
    });
}

#[test]
fn fails_for_already_completed_task() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        // Create and complete a task
        assert_ok!(TaskPallet::create_task(
            RuntimeOrigin::signed(1),
            b"Task 1".to_vec(),
            b"Description".to_vec(),
            3
        ));
        
        assert_ok!(TaskPallet::complete_task(RuntimeOrigin::signed(1), 0));
        
        // Try to complete it again
        assert_noop!(
            TaskPallet::complete_task(RuntimeOrigin::signed(1), 0),
            Error::<Test>::AlreadyCompleted
        );
    });
}

#[test]
fn fails_for_unauthorized_access() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        assert_noop!(
            TaskPallet::create_task(
                RuntimeOrigin::none(),
                b"Task 1".to_vec(),
                b"Description".to_vec(),
                3
            ),
            frame_support::error::BadOrigin
        );
    });
}