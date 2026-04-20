# 📜 EFFICIENCY LAWS (PRACTICES)

To maintain Sanctuary sovereignty, following laws are mandated for all AI interactions:

### 1. The Call-to-Read Pattern
Do not load full files into the context. Guided by instructions, the agent must perform surgical reads.
- **Requirement:** Use `grep_search` first to find the relevant line numbers.

### 2. No Context Flooding
Avoid using tools that return more than 2000 lines of output.
- **Guideline:** If a tool output is too large, paginate or truncate it immediately.

### 3. Progressive Disclosure
Move deep technical details (schemas, API docs) to `references/` and only read them when required by the user.

### 4. Memory Pruning
Periodically summarize the conversation and clear transient thoughts from the agent's buffer.

---
*Sanctuary Pipeline Protocol: ENFORCED.*
