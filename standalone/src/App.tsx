import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { SeraphicLogo } from "./components/SeraphicLogo";
import { PluginChecker } from "./components/PluginChecker";
import { DAWSupport } from "./components/DAWSupport";
import { SystemInfo } from "./components/SystemInfo";
import { Settings } from "./components/Settings";

type Tab = "dashboard" | "checker" | "daw-support" | "system" | "settings";

interface FrameworkInfo {
  version: string;
  os: string;
  arch: string;
}

export default function App() {
  const [activeTab, setActiveTab] = useState<Tab>("dashboard");
  const [frameworkInfo, setFrameworkInfo] = useState<FrameworkInfo | null>(null);

  useEffect(() => {
    Promise.all([
      invoke<string>("get_framework_version"),
      invoke<{ os: string; arch: string; smoothie_version: string }>("get_system_info"),
    ]).then(([version, sysInfo]) => {
      setFrameworkInfo({
        version,
        os:   sysInfo.os,
        arch: sysInfo.arch,
      });
    }).catch(console.error);
  }, []);

  const tabs: { id: Tab; label: string; icon: string }[] = [
    { id: "dashboard",   label: "Dashboard",     icon: "◈" },
    { id: "checker",     label: "Plugin Checker", icon: "⚡" },
    { id: "daw-support", label: "DAW Support",    icon: "🎛" },
    { id: "system",      label: "System",         icon: "⚙" },
    { id: "settings",    label: "Settings",       icon: "≡" },
  ];

  return (
    <div className="app">
      {/* ── Sidebar ── */}
      <aside className="sidebar">
        <div className="sidebar-logo">
          <SeraphicLogo size={48} />
          <div className="sidebar-title">
            <span className="sidebar-name">Smoothie Elite</span>
            <span className="sidebar-sub">Standalone</span>
          </div>
        </div>

        <nav className="sidebar-nav">
          {tabs.map((tab) => (
            <button
              key={tab.id}
              className={`nav-item ${activeTab === tab.id ? "active" : ""}`}
              onClick={() => setActiveTab(tab.id)}
            >
              <span className="nav-icon">{tab.icon}</span>
              <span className="nav-label">{tab.label}</span>
            </button>
          ))}
        </nav>

        <div className="sidebar-footer">
          {frameworkInfo && (
            <>
              <div className="version-badge">
                {frameworkInfo.version}
              </div>
              <div className="system-badge">
                {frameworkInfo.os} · {frameworkInfo.arch}
              </div>
            </>
          )}
        </div>
      </aside>

      {/* ── Main content ── */}
      <main className="main-content">
        {activeTab === "dashboard"   && <Dashboard />}
        {activeTab === "checker"     && <PluginChecker />}
        {activeTab === "daw-support" && <DAWSupport />}
        {activeTab === "system"      && <SystemInfo />}
        {activeTab === "settings"    && <Settings />}
      </main>
    </div>
  );
}

function Dashboard() {
  const cards = [
    {
      icon: "⚡",
      title: "Plugin Checker",
      desc:  "Validate your plugin for format support, DAW compatibility, and security.",
      color: "#ff781e",
    },
    {
      icon: "🎛",
      title: "DAW Support Matrix",
      desc:  "See which DAWs support VST3, CLAP, AU, and AAX on your platform.",
      color: "#50b4f0",
    },
    {
      icon: "🎸",
      title: "VST3 · CLAP · AU · AAX",
      desc:  "Smoothie Elite builds for all major plugin formats from a single Rust codebase.",
      color: "#50c850",
    },
    {
      icon: "🦀",
      title: "Built in Rust",
      desc:  "Bare metal performance. Zero allocations on the audio thread. Type-safe DSP.",
      color: "#b450ff",
    },
    {
      icon: "⚙",
      title: "cargo smoothie new",
      desc:  "Scaffold a new professional plugin in seconds from the command line.",
      color: "#f0c832",
    },
    {
      icon: "✦",
      title: "Open Source · Free Forever",
      desc:  "MIT + Apache-2.0. No license fees. No commercial restrictions. Ever.",
      color: "#ff5050",
    },
  ];

  return (
    <div className="page">
      <div className="page-header">
        <h1 className="page-title">
          <span className="title-accent">Smoothie Elite</span> Framework
        </h1>
        <p className="page-subtitle">
          The elite open-source Rust framework for professional audio plugin development.
          Better than JUCE. Faster than everything else.
        </p>
      </div>

      <div className="card-grid">
        {cards.map((card) => (
          <div key={card.title} className="feature-card">
            <div className="card-icon" style={{ color: card.color }}>{card.icon}</div>
            <h3 className="card-title">{card.title}</h3>
            <p className="card-desc">{card.desc}</p>
          </div>
        ))}
      </div>

      <div className="quickstart">
        <h2>Quick Start</h2>
        <div className="code-block">
          <pre>{`# Install the CLI
cargo install cargo-smoothie

# Create a new plugin
cargo smoothie new my-reverb --formats vst3,clap

# Build + validate
cargo smoothie build
cargo smoothie validate

# Bundle for distribution
cargo smoothie bundle --all`}</pre>
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
