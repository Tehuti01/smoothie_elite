---
id: fi-2456-shared-config.ts.md
category: f-01-secbrain
---

// 🏗️ Example: Shared Config Package
// Centralized linting and type rules.
export const baseConfig = {
  extends: ["next/core-web-vitals", "prettier"],
  rules: {
    "@typescript-eslint/no-explicit-any": "error",
  }
};
