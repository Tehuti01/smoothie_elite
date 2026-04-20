# 📜 INTELLIGENCE LAWS v0.2.0 (PRACTICES)

To maintain Meta-Intelligence sovereignty, the following laws must be strictly enforced:

### 1. The Multi-Tiered Router Rule
The `SKILL.md` (Router) must not contain implementation details.
- **Requirement:** Direct the agent to the sub-folders immediately.
- **Goal:** Keep the "Attention Surface" of the root skill minimal.

### 2. Mandatory Tool-Schema Validation
Every MCP tool must be validated against the `seraphic_tool_v1` schema.
- **Law:** Tools that return raw tracebacks or unstructured text are rejected.
- **Guideline:** Return JSON with a `status`, `message`, and `data` field.

### 3. Imperative Constraint
Instructions must be phrased as direct commands to the silicon.
- **Good:** "Perform surgical read on lines 10-20."
- **Bad:** "You should think about reading some lines."

### 4. Zero-Context Loading
Reference large external docs only via URL or `material_link`.
- **Constraint:** Never paste large documents directly into a skill's `01-Core`.

---
*Intelligence Pipeline Protocol: ENFORCED.*
