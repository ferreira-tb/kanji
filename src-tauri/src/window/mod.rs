#[cfg(desktop)]
pub mod desktop;
#[cfg(mobile)]
pub mod mobile;

use crate::settings::Settings;
use serde_json::json;
use tauri::WebviewUrl;

fn url() -> WebviewUrl {
  WebviewUrl::App("index.html".into())
}

fn script() -> String {
  let mut script = String::new();
  macro_rules! define {
    ($name:literal, $value:expr) => {{
      let name = $name;
      let value = json!($value);
      let snippet = format! {"
        Object.defineProperty(window, '{name}', {{
          configurable: false,
          enumerable: true,
          writable: false,
          value: {value},
        }});
      "};

      script.push_str(&snippet);
    }};
  }

  define!("__DEBUG_ASSERTIONS__", cfg!(debug_assertions));
  define!("__DEFAULT_SETTINGS__", Settings::default());
  define!("__DESKTOP__", cfg!(desktop));
  define!("__MOBILE__", cfg!(mobile));
  define!("__VERSION__", env!("CARGO_PKG_VERSION"));

  script
}
