pub mod compositor;
pub mod xdg_shell;
pub mod shm;

use crate::state::GensouState;

smithay::delegate_dispatch2!(GensouState);