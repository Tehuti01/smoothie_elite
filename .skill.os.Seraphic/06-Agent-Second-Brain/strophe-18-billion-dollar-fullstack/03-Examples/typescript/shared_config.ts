// 🏗️ Example: Shared Config Package
// Centralized linting and type rules.
export const baseConfig = {
  extends: ["next/core-web-vitals", "prettier"],
  rules: {
    "@typescript-eslint/no-explicit-any": "error",
  }
};
