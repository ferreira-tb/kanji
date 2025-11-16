package tsukilabs.kanji

import android.content.res.Configuration
import android.os.Bundle
import androidx.activity.enableEdgeToEdge
import androidx.appcompat.app.AppCompatDelegate
import androidx.appcompat.app.AppCompatDelegate.MODE_NIGHT_YES
import androidx.core.view.WindowCompat
import androidx.core.view.WindowInsetsCompat
import androidx.core.view.WindowInsetsControllerCompat.BEHAVIOR_SHOW_TRANSIENT_BARS_BY_SWIPE

class MainActivity : TauriActivity() {
  override fun onCreate(savedInstanceState: Bundle?) {
    enableEdgeToEdge()
    super.onCreate(savedInstanceState)
    this.hideStatusBar()
    this.setNightMode()
  }

  override fun onResume() {
    super.onResume()
    this.hideStatusBar()
    this.setNightMode()
  }

  override fun onConfigurationChanged(newConfig: Configuration) {
    super.onConfigurationChanged(newConfig)
    this.hideStatusBar()
    this.setNightMode()
  }

  override fun onWindowFocusChanged(hasFocus: Boolean) {
    super.onWindowFocusChanged(hasFocus)
    if (hasFocus) {
      this.hideStatusBar()
      this.setNightMode()
    }
  }

  private fun hideStatusBar() {
    WindowCompat.getInsetsController(window, window.decorView).apply {
      this.systemBarsBehavior = BEHAVIOR_SHOW_TRANSIENT_BARS_BY_SWIPE
      this.hide(WindowInsetsCompat.Type.statusBars())
    }
  }

  private fun setNightMode() {
    AppCompatDelegate.setDefaultNightMode(MODE_NIGHT_YES)
  }
}