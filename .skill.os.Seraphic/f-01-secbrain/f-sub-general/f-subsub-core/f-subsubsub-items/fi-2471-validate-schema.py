---
id: fi-2471-validate-schema.py
category: f-01-secbrain
---

import json
import jsonschema

# 🛠️ validate_schema.py — MCP Forge Auditor
# Validates MCP tool schemas against the Seraphic standard.

def validate_schema(schema_path):
    print(f"🚀 Validating MCP schema at {schema_path}...")
    
    with open(schema_path, "r") as f:
        schema = json.load(f)
        
    # [Strophe 13]: Standard Seraphic Tool Requirements
    required_keys = ["name", "description", "inputSchema"]
    
    for key in required_keys:
        if key not in schema:
            print(f"❌ ERROR: Missing required key: {key}")
            return
            
    print("✓ Schema satisfies Seraphic Standard.")

if __name__ == "__main__":
    validate_schema("tool_schema.json")
