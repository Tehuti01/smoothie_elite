import hashlib
import ed25519

# 🛡️ sign_binary.py — Strophe 8 Security Auditor
# Signs binary assets using Ed25519 for Seraphic verification.

def sign_asset(asset_path, private_key_hex):
    print(f"🚀 Signing asset at {asset_path}...")
    
    # Load private key
    signing_key = ed25519.SigningKey(bytes.fromhex(private_key_hex))
    
    # Load and hash the asset
    with open(asset_path, "rb") as f:
        data = f.read()
    
    # Generate signature
    signature = signing_key.sign(data)
    
    # Save signature
    with open(f"{asset_path}.sig", "wb") as f:
        f.write(signature)
        
    print(f"✓ Signature created: {asset_path}.sig")

if __name__ == "__main__":
    # Example (Use real keys in production)
    sign_asset("plugin.vst3", "0" * 64)
