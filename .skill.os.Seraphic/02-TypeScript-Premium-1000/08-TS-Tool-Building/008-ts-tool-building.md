# 🔧 TYPESCRIPT TOOL BUILDING

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
                    🔧 TYPESCRIPT TOOL BUILDING 🔧
              Build Executable TS Tools
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

## TOOL FROM SPEC

```typescript
interface ToolSpec {
    name: string;
    description: string;
    input: string;
    output: string;
    handler: string;
}

async function buildTool(spec: ToolSpec): Promise<void> {
    // Write file
    await writeFile(`tools/${spec.name}.ts`, spec.handler);
    
    // Install deps
    await exec('npm install');
    
    // Build
    await exec('npx tsc');
}
```

---

*TS Tool Building | Version: 1.0*