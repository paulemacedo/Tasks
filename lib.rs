#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), no_main)]


#[ink::contract]
mod task_management {
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
    use ink::storage::Mapping;
    use parity_scale_codec::{Decode, Encode};

    #[derive(Decode, Encode, Debug, PartialEq, Clone, scale_info::TypeInfo)]
    #[cfg_attr(feature = "std", derive(ink::storage::traits::StorageLayout))]
    pub struct Task {
        title: String,
        description: String,
        priority: u8,
        completed: bool,
        created_at: u64,
    }

    #[ink(storage)]
    pub struct TaskManager {
        tasks: ink::storage::Mapping<u32, Task>, // Usando Mapping corretamente
        task_count: u32,
    }

    impl Default for TaskManager {
        fn default() -> Self {
            Self::new()
        }
    }

    #[ink(event)]
    pub struct TaskCreated {
        #[ink(topic)]
        task_id: u32,
        title: String,
        priority: u8,
    }

    #[ink(event)]
    pub struct TaskCompleted {
        #[ink(topic)]
        task_id: u32,
    }

    #[ink(event)]
    pub struct TaskDeleted {
        #[ink(topic)]
        task_id: u32,
    }


    impl TaskManager {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                tasks: Mapping::default(),
                task_count: 0,
            }
        }

        #[ink(message)]
        pub fn create_task(&mut self, title: String, description: String, priority: u8) -> u32 {
            let task_id = self.task_count.saturating_add(1);
            let priority = priority.clamp(1, 5);
            let task = Task {
                title: title.clone(),
                description,
                priority,
                completed: false,
                created_at: Self::env().block_timestamp(),
            };
            self.tasks.insert(task_id, &task);
            self.task_count = task_id;

            self.env().emit_event(TaskCreated {
                task_id,
                title,
                priority,
            });

            task_id
        }

        #[ink(message)]
        pub fn update_priority(&mut self, task_id: u32, new_priority: u8) {
            if let Some(mut task) = self.tasks.get(task_id) {
                task.priority = new_priority.clamp(1, 5);
                self.tasks.insert(task_id, &task);
            }
        }

        #[ink(message)]
        pub fn get_tasks(&self) -> Vec<(u32, Task)> {
            let mut tasks = Vec::new();
            for task_id in 1..=self.task_count {
                if let Some(task) = self.tasks.get(task_id) {
                    tasks.push((task_id, task));
                }
            }
            tasks
        }

        #[ink(message)]
        pub fn complete_task(&mut self, task_id: u32) -> bool {
            if let Some(mut task) = self.tasks.get(task_id) {
                if !task.completed {
                    task.completed = true;
                    self.tasks.insert(task_id, &task);
                    self.env().emit_event(TaskCompleted { task_id });
                    true
                } else {
                    false
                }
            } else {
                false
            }
        }

        #[ink(message)]
        pub fn update_task(&mut self, task_id: u32, title: Option<String>, priority: Option<u8>) -> bool {
            if let Some(mut task) = self.tasks.get(task_id) {
                if let Some(new_title) = title {
                    task.title = new_title;
                }
                if let Some(new_priority) = priority {
                    task.priority = new_priority.clamp(1, 5);
                }
                self.tasks.insert(task_id, &task);
                true
            } else {
                false
            }
        }

        #[ink(message)]
        pub fn delete_task(&mut self, task_id: u32) -> bool {
            if self.tasks.get(task_id).is_some() {
                self.tasks.remove(task_id);
                self.env().emit_event(TaskDeleted { task_id });
                true
            } else {
                false
            }
        }

        #[ink(message)]
        pub fn get_total_tasks(&self) -> u32 {
            self.task_count
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink::env::test;

        #[ink::test]
        fn test_create_task() {
            let mut task_manager = TaskManager::new();
            let task_id = task_manager.create_task("Test Task".to_string(), "Description".to_string(), 3);

            let task = task_manager.get_task(task_id).unwrap();
            assert_eq!(task.title, "Test Task");
            assert_eq!(task.priority, 3);
            assert_eq!(task.completed, false);
        }

        #[ink::test]
        fn test_complete_task() {
            let mut task_manager = TaskManager::new();
            let task_id = task_manager.create_task("Test Task".to_string(), "Description".to_string(), 3);

            assert!(task_manager.complete_task(task_id));
            let task = task_manager.get_task(task_id).unwrap();
            assert!(task.completed);

            // Tentativa de completar tarefa já completa
            assert!(!task_manager.complete_task(task_id));
        }

        #[ink::test]
        fn test_update_task() {
            let mut task_manager = TaskManager::new();
            let task_id = task_manager.create_task("Original Task".to_string(), "Description".to_string(), 3);

            // Atualizar título
            assert!(task_manager.update_task(task_id, Some("Updated Task".to_string()), None));
            let task = task_manager.get_task(task_id).unwrap();
            assert_eq!(task.title, "Updated Task");

            // Atualizar prioridade
            assert!(task_manager.update_task(task_id, None, Some(5)));
            let task = task_manager.get_task(task_id).unwrap();
            assert_eq!(task.priority, 5);
        }

        #[ink::test]
        fn test_delete_task() {
            let mut task_manager = TaskManager::new();
            let task_id = task_manager.create_task("Task to Delete".to_string(), "Description".to_string(), 3);

            assert!(task_manager.delete_task(task_id));
            assert!(task_manager.get_task(task_id).is_none());

            // Tentativa de deletar tarefa inexistente
            assert!(!task_manager.delete_task(task_id));
        }

        #[ink::test]
        fn test_priority_constraints() {
            let mut task_manager = TaskManager::new();

            // Testar limite inferior de prioridade
            let task_id1 = task_manager.create_task("Low Priority Task".to_string(), "Description".to_string(), 0);
            let task1 = task_manager.get_task(task_id1).unwrap();
            assert_eq!(task1.priority, 1);

            // Testar limite superior de prioridade
            let task_id2 = task_manager.create_task("High Priority Task".to_string(), "Description".to_string(), 10);
            let task2 = task_manager.get_task(task_id2).unwrap();
            assert_eq!(task2.priority, 5);
        }

        #[ink::test]
        fn test_task_count() {
            let mut task_manager = TaskManager::new();

            assert_eq!(task_manager.get_total_tasks(), 0);

            task_manager.create_task("Task 1".to_string(), "Description".to_string(), 3);
            assert_eq!(task_manager.get_total_tasks(), 1);

            task_manager.create_task("Task 2".to_string(), "Description".to_string(), 4);
            assert_eq!(task_manager.get_total_tasks(), 2);
        }

        #[ink::test]
        fn test_invalid_task_id() {
            let mut task_manager = TaskManager::new();
            assert!(task_manager.get_task(1).is_none());
            assert!(!task_manager.complete_task(1));
            assert!(!task_manager.delete_task(1));
        }

        #[ink::test]
        fn test_update_nonexistent() {
            let mut task_manager = TaskManager::new();
            assert!(!task_manager.update_task(1, Some("New".into()), Some(3)));
        }
    }
}