---
id: fi-2476-compress-context.py
category: f-01-secbrain
---

# 📉 compress_context.py — Sanctuary Auditor
# Audits the current context usage and proposes compression targets.

def audit_context():
    print("🚀 Auditing Information Sanctuary...")
    
    # [Simulate measuring token usage]
    tokens_used = 15420
    max_tokens = 200000
    
    print(f"📊 Usage: {tokens_used}/{max_tokens} tokens.")
    
    if tokens_used > 100000:
        print("⚠️ WARNING: Context Sanctuary is becoming bloated.")
        print("💡 Recommendation: Summarize previous 10 turns and clear the buffer.")
    else:
        print("✓ Sanctuary integrity within nominal limits.")

if __name__ == "__main__":
    audit_context()
