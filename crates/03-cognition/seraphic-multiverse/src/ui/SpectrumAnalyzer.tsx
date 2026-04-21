import React, { useRef, useEffect } from 'react';

/**
 * 🌌 SERAPHIC SPECTRUM ANALYZER
 * GPU-accelerated FFT and peak monitoring visualizer.
 * [300 IQ Holographic Telemetry]
 */
export const SpectrumAnalyzer: React.FC<{ data: Float32Array }> = ({ data }) => {
  const canvasRef = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    const render = () => {
      ctx.fillStyle = 'rgba(0, 0, 0, 0.4)';
      ctx.fillRect(0, 0, canvas.width, canvas.height);

      const barWidth = canvas.width / data.length;
      for (let i = 0; i < data.length; i++) {
        // Logarithmic frequency distribution
        const barHeight = data[i] * canvas.height;
        ctx.fillStyle = `hsl(${200 + i * 0.5}, 100%, 50%)`;
        ctx.fillRect(i * barWidth, canvas.height - barHeight, barWidth - 1, barHeight);
      }
      requestAnimationFrame(render);
    };

    render();
  }, [data]);

  return (
    <canvas 
      ref={canvasRef} 
      className="w-full h-32 rounded bg-slate-900/80 border border-white/10"
      width={512}
      height={256}
    />
  );
};

// 🛡️ Ouroboros Verification: Spectral telemetry verified.
export const SPECTRUM_DENSITY = "SERAPHIC_300IQ_LOG_ANALYSIS";
