import os
import ed25519

# 🛡️ sign_asset.py v0.2.0 — Strophe 8 Security Tool
# Generates Ed25519 signatures for Seraphic Assets.

def sign_sovereign_asset(asset_path, private_key_hex):
    print(f"🚀 INITIATING STROPHE 8: CRYPTOGRAPHIC SIGNING ({asset_path})...")
    
    if not os.path.exists(asset_path):
        print("❌ ERROR: Asset not found.")
        return

    # Load signing key
    try:
        signing_key = ed25519.SigningKey(bytes.fromhex(private_key_hex))
    except Exception as e:
        print(f"❌ ERROR: Invalid private key format. ({e})")
        return

    # Read data
    with open(asset_path, "rb") as f:
        data = f.read()

    # Generate Ed25519 Signature
    signature = signing_key.sign(data)
    
    sig_path = f"{asset_path}.sig"
    with open(sig_path, "wb") as f:
        f.write(signature)

    print(f"✅ SUCCESS: Asset signed. Signature stored at {sig_path}.")
    print(f"   - Signature Hex: {signature.hex()}")

if __name__ == "__main__":
    # Demo Key (Replace in production)
    DEMO_KEY = "0" * 64 
    
    # Create a dummy asset
    with open("demo_asset.json", "w") as f:
        f.write('{"seraphic": "mandate"}')
        
    sign_sovereign_asset("demo_asset.json", DEMO_KEY)
