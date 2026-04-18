import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

interface DawEntry {
  name:  string;
  vst3:  boolean;
  clap:  boolean;
  au:    boolean;
  aax:   boolean;
  notes: string;
}

export function DAWSupport() {
  const [daws, setDaws] = useState<DawEntry[]>([]);

  useEffect(() => {
    invoke<DawEntry[]>("get_daw_support").then(setDaws).catch(console.error);
  }, []);

  const tick  = <span className="supported">✓</span>;
  const cross = <span className="unsupported">✗</span>;

  return (
    <div className="page">
      <div className="page-header">
        <h1 className="page-title"><span className="title-accent">DAW</span> Support Matrix</h1>
        <p className="page-subtitle">
          Which DAWs support each plugin format on your current platform.
        </p>
      </div>

      <div className="table-wrapper">
        <table className="daw-table">
          <thead>
            <tr>
              <th>DAW</th>
              <th>VST3</th>
              <th>CLAP</th>
              <th>AU</th>
              <th>AAX</th>
              <th>Notes</th>
            </tr>
          </thead>
          <tbody>
            {daws.map((daw) => (
              <tr key={daw.name}>
                <td className="daw-name">{daw.name}</td>
                <td>{daw.vst3 ? tick : cross}</td>
                <td>{daw.clap ? tick : cross}</td>
                <td>{daw.au  ? tick : cross}</td>
                <td>{daw.aax ? tick : cross}</td>
                <td className="daw-notes">{daw.notes}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>

      <div className="format-legend">
        <h3>Format Legend</h3>
        <div className="legend-grid">
          {[
            { name: "VST3",  color: "#ff781e", desc: "Virtual Studio Technology 3 — Steinberg. Supported by nearly every DAW." },
            { name: "CLAP",  color: "#50b4f0", desc: "CLever Audio Plugin — Modern open standard. FL Studio 21+, Bitwig, Reaper." },
            { name: "AU",    color: "#50c850", desc: "Audio Units — Apple's format. macOS only. Logic, GarageBand, Ableton (macOS)." },
            { name: "AAX",   color: "#f0c832", desc: "Avid Audio eXtension — Pro Tools only. Requires Avid SDK + signing." },
          ].map((f) => (
            <div key={f.name} className="legend-item">
              <span className="legend-tag" style={{ borderColor: f.color, color: f.color }}>{f.name}</span>
              <p>{f.desc}</p>
            </div>
          ))}
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
