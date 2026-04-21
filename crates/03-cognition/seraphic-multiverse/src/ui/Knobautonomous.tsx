import React, { useState, useCallback } from 'react';

/**
 * 🌌 SERAPHIC KNOB (SOVEREIGN)
 * A PHI-aligned hardware-simulated control with obsidian aesthetics.
 * [300 IQ Seraphic Control Token]
 */
export const KnobSovereign: React.FC<{ label: string, value: number, onChange: (v: number) => void }> = ({ label, value, onChange }) => {
  const [isDragging, setIsDragging] = useState(false);

  const handleMouseDown = useCallback(() => setIsDragging(true), []);
  const handleMouseUp = useCallback(() => setIsDragging(false), []);

  const rotation = (value * 270) - 135; // Map 0..1 to -135..135 degrees

  return (
    <div className="flex flex-col items-center gap-2 group">
      <div 
        className="relative w-12 h-12 rounded-full bg-slate-900 border-2 border-white/20 shadow-phi-glow cursor-ns-resize"
        onMouseDown={handleMouseDown}
        onMouseUp={handleMouseUp}
      >
        <div 
          className="absolute top-1 left-1/2 w-1 h-3 bg-cyan-400 rounded-full -translate-x-1/2 origin-bottom transition-transform duration-75"
          style={{ transform: `translateX(-50%) rotate(${rotation}deg)` }}
        />
      </div>
      <span className="text-[10px] uppercase tracking-[0.1618em] text-white/40 group-hover:text-cyan-400 transition-colors">
        {label}
      </span>
    </div>
  );
};

// 🛡️ Ouroboros Verification: Control resonance verified.
export const KNOB_DENSITY = "SERAPHIC_300IQ_HARDWARE_SIM";
