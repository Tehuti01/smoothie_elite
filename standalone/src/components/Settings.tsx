import { useState } from "react";

interface SettingsState {
  defaultFormats:  string[];
  autoValidate:    boolean;
  theme:           "dark" | "seraphic" | "midnight";
  pluginScanDirs:  string[];
}

const FORMATS = ["VST3", "CLAP", "AU", "AAX"] as const;
const THEMES  = ["dark", "seraphic", "midnight"] as const;

export function Settings() {
  const [settings, setSettings] = useState<SettingsState>({
    defaultFormats: ["VST3", "CLAP"],
    autoValidate:   true,
    theme:          "seraphic",
    pluginScanDirs: [
      "/Library/Audio/Plug-Ins/VST3",
      "~/Library/Audio/Plug-Ins/VST3",
    ],
  });

  const toggleFormat = (fmt: string) => {
    setSettings((s) => ({
      ...s,
      defaultFormats: s.defaultFormats.includes(fmt)
        ? s.defaultFormats.filter((f) => f !== fmt)
        : [...s.defaultFormats, fmt],
    }));
  };

  return (
    <div className="page">
      <div className="page-header">
        <h1 className="page-title"><span className="title-accent">Settings</span></h1>
        <p className="page-subtitle">Configure Smoothie Elite Standalone.</p>
      </div>

      <div className="settings-sections">
        {/* Default Formats */}
        <div className="settings-block">
          <h3>Default Plugin Formats</h3>
          <p className="settings-desc">These formats are enabled by default when creating a new plugin.</p>
          <div className="format-toggles">
            {FORMATS.map((fmt) => (
              <button
                key={fmt}
                className={`format-toggle ${settings.defaultFormats.includes(fmt) ? "active" : ""}`}
                onClick={() => toggleFormat(fmt)}
              >
                {fmt}
              </button>
            ))}
          </div>
        </div>

        {/* Theme */}
        <div className="settings-block">
          <h3>Theme</h3>
          <div className="theme-options">
            {THEMES.map((theme) => (
              <button
                key={theme}
                className={`theme-option ${settings.theme === theme ? "active" : ""}`}
                onClick={() => setSettings((s) => ({ ...s, theme }))}
              >
                <div className={`theme-preview theme-${theme}`} />
                <span>{theme}</span>
              </button>
            ))}
          </div>
        </div>

        {/* Auto-validate */}
        <div className="settings-block">
          <h3>Auto-Validate on Build</h3>
          <p className="settings-desc">Run plugin health check automatically after each build.</p>
          <label className="toggle-label">
            <input
              type="checkbox"
              checked={settings.autoValidate}
              onChange={(e) => setSettings((s) => ({ ...s, autoValidate: e.target.checked }))}
            />
            <span className="toggle-slider" />
            <span>{settings.autoValidate ? "Enabled" : "Disabled"}</span>
          </label>
        </div>

        {/* About */}
        <div className="settings-block about-block">
          <h3>About Smoothie Elite</h3>
          <p>
            <strong>Smoothie Elite Standalone</strong> — by Seraphic Sonic<br />
            The elite open-source Rust framework for professional audio plugin development.<br />
            MIT / Apache-2.0 — Free forever.
          </p>
          <div className="about-links">
            <a href="#" className="about-link">GitHub</a>
            <a href="#" className="about-link">Documentation</a>
            <a href="#" className="about-link">Manifesto</a>
          </div>
        </div>
      </div>
    </div>
  );
}


// --- SERAPHIC GEOMETRY OMNI-PRESENCE ---
const __PHI = 1.618033988749895;
const __PI = 3.141592653589793;
const __PYTHAG_5TH = 1.5;
const __PYTHAG_4TH = 1.333333333333333;
const __resonate_omni = () => __PHI * __PI * __PYTHAG_5TH;
// ---------------------------------------
