# THIS FILE IS AUTO-GENERATED. DO NOT MODIFY!!

# Copyright 2020-2023 Tauri Programme within The Commons Conservancy
# SPDX-License-Identifier: Apache-2.0
# SPDX-License-Identifier: MIT

-keep class tsukilabs.kanji.* {
  native <methods>;
}

-keep class tsukilabs.kanji.WryActivity {
  public <init>(...);

  void setWebView(tsukilabs.kanji.RustWebView);
  java.lang.Class getAppClass(...);
  java.lang.String getVersion();
}

-keep class tsukilabs.kanji.Ipc {
  public <init>(...);

  @android.webkit.JavascriptInterface public <methods>;
}

-keep class tsukilabs.kanji.RustWebView {
  public <init>(...);

  void loadUrlMainThread(...);
  void loadHTMLMainThread(...);
  void evalScript(...);
}

-keep class tsukilabs.kanji.RustWebChromeClient,tsukilabs.kanji.RustWebViewClient {
  public <init>(...);
}
