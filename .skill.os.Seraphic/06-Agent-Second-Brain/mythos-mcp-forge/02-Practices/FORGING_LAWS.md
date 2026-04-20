# 📜 FORGING LAWS (PRACTICES)

To maintain Tool sovereignty, following laws are mandated for all MCP development:

### 1. The Standard Tool Pattern
Every tool must provide a concise `description` that triggers correctly based on user intent.
- **Requirement:** Include an `inputSchema` with `required` fields.

### 2. Silent Failures (Agentic Ergonomics)
Tools must not flood the agent with raw stack traces.
- **Guideline:** Catch errors and return a JSON object with a `content` field containing high-signal error messages.

### 3. Resource Mapping
If a tool provides access to a large file, expose it as an MCP `resource` rather than returning the full content as a string.

### 4. Security Lockdown
MCP servers must never expose environment variables or private keys unless explicitly required for the current invariant.

---
*Tool Pipeline Protocol: ENFORCED.*
