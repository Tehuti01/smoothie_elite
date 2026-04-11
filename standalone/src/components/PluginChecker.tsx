import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";

interface HealthReport {
  plugin_path: string;
  is_healthy:  boolean;
  passed:      number;
  warnings:    number;
  failed:      number;
  json:        string;
}

interface ValidationSection {
  name:   string;
  checks: ValidationCheck[];
}

interface ValidationCheck {
  name:     string;
  status:   "Pass" | "Warning" | "Fail" | "NotApplicable";
  message:  string;
  detail:   string | null;
  fix:      string | null;
}

export function PluginChecker() {
  const [report,   setReport]   = useState<HealthReport | null>(null);
  const [sections, setSections] = useState<ValidationSection[]>([]);
  const [loading,  setLoading]  = useState(false);
  const [error,    setError]    = useState<string | null>(null);

  const runSelfCheck = async () => {
    setLoading(true);
    setError(null);
    try {
      const r = await invoke<HealthReport>("check_plugin_health");
      setReport(r);
      const parsed = JSON.parse(r.json);
      setSections(parsed.sections || []);
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  };

  const browsePlugin = async () => {
    const selected = await open({
      multiple: false,
      filters:  [{ name: "Plugin", extensions: ["vst3", "clap", "component", "aaxplugin"] }],
    });
    if (!selected) return;

    setLoading(true);
    setError(null);
    try {
      const r = await invoke<HealthReport>("validate_plugin", { path: selected });
      setReport(r);
      const parsed = JSON.parse(r.json);
      setSections(parsed.sections || []);
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  };

  const statusIcon = (s: string) => {
    if (s === "Pass")           return <span className="check-pass">✅</span>;
    if (s === "Warning")        return <span className="check-warn">⚠️</span>;
    if (s === "Fail")           return <span className="check-fail">❌</span>;
    return                             <span className="check-na">—</span>;
  };

  return (
    <div className="page">
      <div className="page-header">
        <h1 className="page-title"><span className="title-accent">Plugin</span> Health Checker</h1>
        <p className="page-subtitle">
          Validate your plugin for format support, DAW compatibility, security, and system requirements.
        </p>
      </div>

      <div className="action-row">
        <button className="btn btn-primary" onClick={runSelfCheck} disabled={loading}>
          {loading ? "Checking…" : "⚡ Run Self-Check"}
        </button>
        <button className="btn btn-secondary" onClick={browsePlugin} disabled={loading}>
          📂 Browse Plugin File
        </button>
      </div>

      {error && (
        <div className="error-box">{error}</div>
      )}

      {report && (
        <>
          {/* Summary bar */}
          <div className={`health-summary ${report.is_healthy ? "healthy" : "unhealthy"}`}>
            <div className="health-status">
              {report.is_healthy ? "✅ HEALTHY" : "❌ ISSUES FOUND"}
            </div>
            <div className="health-counts">
              <span className="count-pass">{report.passed} passed</span>
              <span className="count-warn">{report.warnings} warnings</span>
              <span className="count-fail">{report.failed} failed</span>
            </div>
            <div className="health-path">{report.plugin_path}</div>
          </div>

          {/* Sections */}
          {sections.map((section) => (
            <div key={section.name} className="check-section">
              <h3 className="section-title">{section.name}</h3>
              <div className="check-list">
                {section.checks.map((check, i) => (
                  <div key={i} className={`check-item check-${check.status.toLowerCase()}`}>
                    <div className="check-row">
                      {statusIcon(check.status)}
                      <span className="check-name">{check.name}</span>
                      <span className="check-msg">{check.message}</span>
                    </div>
                    {check.detail && (
                      <div className="check-detail">ℹ {check.detail}</div>
                    )}
                    {check.fix && (
                      <div className="check-fix">🔧 {check.fix}</div>
                    )}
                  </div>
                ))}
              </div>
            </div>
          ))}
        </>
      )}
    </div>
  );
}
