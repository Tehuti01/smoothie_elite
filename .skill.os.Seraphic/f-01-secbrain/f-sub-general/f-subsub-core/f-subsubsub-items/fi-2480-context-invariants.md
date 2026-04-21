---
id: fi-2480-context-invariants.md
category: f-01-secbrain
---

# 📥 CONTEXT INVARIANTS (CORE)

Information bloat is a terminal violation of the agentic path. The Context Manager governs the **Information Sanctuary**.

## 🌀 THE PRINCIPLES OF THE SANCTUARY

### I. Surgical Reading
- **Definition:** Reading only the specific lines of code required for the current task.
- **Goal:** Minimize context window usage and prevent hallucination.
- **Path:** We utilize `read_file` with `start_line` and `end_line` parameters for every operation.

### II. Token Sanctuary Rules
- **Definition:** Every token in the prompt must have a physical reason for being there.
- **Goal:** Maximum reasoning density.
- **Rule:** Re-state information only if it is critical to the current invariant (e.g., L0, A0).

### III. Context Compression
- **Definition:** Condensing long discussions or code summaries into short, high-signal manifests.
- **Path:** We utilize the `compress_context.py` tool to audit and prune the agent's memory.

---
*Information Sanctuary achieved.*
