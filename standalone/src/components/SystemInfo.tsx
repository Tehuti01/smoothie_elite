import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

interface SysInfo {
  os:               string;
  arch:             string;
  os_version:       string | null;
  smoothie_version: string;
}

interface InstalledPlugin {
  path: string;
}

export function SystemInfo() {
  const [sysInfo, setSysInfo] = useState<SysInfo | null>(null);
  const [plugins, setPlugins] = useState<string[]>([]);

  useEffect(() => {
    invoke<SysInfo>("get_system_info").then(setSysInfo).catch(console.error);
    invoke<string[]>("list_installed_plugins").then(setPlugins).catch(console.error);
  }, []);

  return (
    <div className="page">
      <div className="page-header">
        <h1 className="page-title"><span className="title-accent">System</span> Information</h1>
        <p className="page-subtitle">Your platform details and installed plugins.</p>
      </div>

      {sysInfo && (
        <div className="info-grid">
          <InfoCard label="Operating System"       value={sysInfo.os} />
          <InfoCard label="Architecture"           value={sysInfo.arch} />
          <InfoCard label="OS Version"             value={sysInfo.os_version ?? "Unknown"} />
          <InfoCard label="Smoothie Elite Version" value={sysInfo.smoothie_version} />
          <InfoCard
            label="VST3 Support"
            value="Available"
            status="pass"
          />
          <InfoCard
            label="CLAP Support"
            value="Available"
            status="pass"
          />
          <InfoCard
            label="AU Support"
            value={sysInfo.os === "macos" ? "Available" : "macOS only"}
            status={sysInfo.os === "macos" ? "pass" : "warn"}
          />
          <InfoCard
            label="AAX Support"
            value="Requires Avid SDK"
            status="warn"
          />
        </div>
      )}

      <div className="section-block">
        <div className="section-header-row">
          <h2>Installed Plugins</h2>
          <button
            className="btn btn-secondary btn-small"
            onClick={() => invoke("open_plugin_folder")}
          >
            Open Folder
          </button>
        </div>
        {plugins.length === 0 ? (
          <p className="empty-state">No plugins found in standard directories.</p>
        ) : (
          <div className="plugin-list">
            {plugins.map((p, i) => (
              <div key={i} className="plugin-item">{p}</div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}

function InfoCard({
  label, value, status,
}: { label: string; value: string; status?: "pass" | "warn" | "fail" }) {
  const color = status === "pass" ? "#50c850" : status === "warn" ? "#f0c832" : status === "fail" ? "#ff5050" : "inherit";
  return (
    <div className="info-card">
      <div className="info-label">{label}</div>
      <div className="info-value" style={{ color }}>{value}</div>
    </div>
  );
}
