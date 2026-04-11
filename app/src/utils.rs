#[cfg(feature = "mobile")]
pub fn set_android_flags() {
  use dioxus::mobile::wry::prelude::dispatch;
  dispatch(|env, activity, _webview| {
    // Get the window
    let window = env
      .call_method(activity, "getWindow", "()Landroid/view/Window;", &[])
      .unwrap()
      .l()
      .unwrap();

    // Set status bar color
    let color = 0xFF09090Bu32 as i32; // ARGB
    env
      .call_method(&window, "setStatusBarColor", "(I)V", &[color.into()])
      .unwrap();

    // Set navigation bar color
    env
      .call_method(&window, "setNavigationBarColor", "(I)V", &[color.into()])
      .unwrap();
  });
}
