interface Props { size?: number; className?: string; }

export function SeraphicLogo({ size = 40, className = "" }: Props) {
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 100 100"
      className={className}
      xmlns="http://www.w3.org/2000/svg"
    >
      {/* Outer ring */}
      <circle cx="50" cy="50" r="46" fill="none" stroke="#ff781e" strokeWidth="4" />
      {/* Inner geometric — stylised S + waveform */}
      <path
        d="M30 35 Q50 20 70 35 Q50 50 30 65 Q50 80 70 65"
        fill="none"
        stroke="#ff781e"
        strokeWidth="5"
        strokeLinecap="round"
        strokeLinejoin="round"
      />
      {/* Center dot */}
      <circle cx="50" cy="50" r="4" fill="#ff781e" />
      {/* Subtle inner ring */}
      <circle cx="50" cy="50" r="22" fill="none" stroke="#ff781e" strokeWidth="1.5" opacity="0.4" />
    </svg>
  );
}
