# 🛠️ TOOL ENGINEERING (PRACTICES)

All tools within the Seraphic ecosystem must be "Real Code"—deterministic, high-performance, and stack-agnostic.

## 📜 CODING STANDARDS

### I. Rust Tools (The Muscle)
- **Use Case:** High-speed file processing, binary auditing, performance measurement.
- **Rule:** Use `std` where possible for portability, but prioritize `no-std` logic for DSP auditing.
- **Goal:** ALU-direct speed for matrix management.

### II. Python Tools (The Brain)
- **Use Case:** Complex data manipulation, JSON/YAML parsing, quick prototyping.
- **Rule:** Use standard libraries only (no external `pip` dependencies unless critical).
- **Goal:** High-signal utility scripts.

### III. React/TypeScript (The Eye)
- **Use Case:** Visualizing the matrix hub, interactive documentation, UI components.
- **Rule:** Use Functional Components with Tailwind/Shadcn alignment.
- **Goal:** Professional-grade visual sovereignty.

---
*Tool Engineering Protocol: ENFORCED.*
