use crate::command;
use tauri::Wry;
use tauri_specta::{Builder, ErrorHandlingMode, collect_commands};

pub fn collect() -> Builder {
  let builder = Builder::<Wry>::new()
    .error_handling(ErrorHandlingMode::Throw)
    .commands(collect_commands![
      command::create_tray_icon,
      command::pick_folder,
      command::search_kanji,
      command::show_window
    ]);

  #[cfg(debug_assertions)]
  export(&builder);

  builder
}

#[cfg(debug_assertions)]
fn export(specta: &Builder) {
  use specta_typescript::{BigIntExportBehavior, Typescript};

  let ts = Typescript::default()
    .bigint(BigIntExportBehavior::BigInt)
    .header("// @ts-nocheck");

  specta
    .export(ts, "../src/api/bindings.ts")
    .expect("failed to export typescript bindings");
}
