use crate::database::DatabaseHandle;
use crate::server::Server;
use tauri::{Manager, State, Wry};

pub trait ManagerExt: Manager<Wry> {
  fn database(&self) -> State<'_, DatabaseHandle> {
    self.app_handle().state::<DatabaseHandle>()
  }

  fn server(&self) -> State<'_, Server> {
    self.app_handle().state::<Server>()
  }
}

impl<T: Manager<Wry>> ManagerExt for T {}
