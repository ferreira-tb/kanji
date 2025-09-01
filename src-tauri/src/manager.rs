use crate::database::DatabaseHandle;
use tauri::{Manager, State, Wry};

pub trait ManagerExt: Manager<Wry> {
  fn database(&self) -> State<'_, DatabaseHandle> {
    self.app_handle().state::<DatabaseHandle>()
  }
}

impl<T: Manager<Wry>> ManagerExt for T {}
