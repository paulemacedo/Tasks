# Task Pallet

A Substrate pallet that implements a task management system.

## Features
- Create tasks
- Update task priority
- Complete tasks
- Delete tasks
- Update task details

## Installation

Add this to your runtime's `Cargo.toml`:

```toml
[dependencies]
task-pallet = { path = "../../frame/task-pallet" }
```

```toml
[features]
default = ["std"]

std = [
  "task-pallet/std",
]
```
## Usage

```rust
parameter_types! {
  pub const MaxTasks: u32 = 100;
}

impl task_pallet::Config for Runtime {
  type RuntimeEvent = RuntimeEvent;
  type MaxTasks = MaxTasks;
  type TaskId = u32;
}

construct_runtime!(
  pub enum Runtime {
    System: frame_system,
    Babe: pallet_babe,
    SubstrateTest: substrate_test_pallet::pallet,
    Balances: pallet_balances,
    Task: pallet_task::{Pallet, Call, Storage, Event<T>},
  }
);
```

## Testing
Run the tests with:
```bash
cargo test -p task-pallet
```