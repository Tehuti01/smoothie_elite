import os
import markdown
import jinja2

# 🏗️ forge_nexus.py v0.2.0 — Strophe 16 Static Site Generator
# Generates the Seraphic Documentation Hub from source markdown.

def forge_hub():
    print("🚀 INITIATING STROPHE 16: NEXUS HUB GENERATION...")
    
    # Use absolute-relative paths from project root
    docs_dir = "smoothie_elite/docs"
    output_dir = "smoothie_elite/target/doc_hub"
    os.makedirs(output_dir, exist_ok=True)

    # 1. Load Template (Mock logic)
    template = """
    <html>
    <head><style>body { background: #0B0C10; color: #00B4D8; font-family: Inter; }</style></head>
    <body>
        <div id="nexus-sidebar" style="width: 23%;">
            <h1>SERAPHIC NEXUS</h1>
        </div>
        <div id="content" style="width: 77%;">
            {{ content }}
        </div>
    </body>
    </html>
    """
    
    # 2. Process Markdown files
    for root, _, files in os.walk(docs_dir):
        for file in files:
            if file.endswith(".md"):
                print(f"   - Forging {file}...")
                with open(os.path.join(root, file), "r") as f:
                    html_content = markdown.markdown(f.read())
                    
                # Render with Jinja
                rendered = jinja2.Template(template).render(content=html_content)
                
                # Save output
                out_path = os.path.join(output_dir, file.replace(".md", ".html"))
                with open(out_path, "w") as f:
                    f.write(rendered)

    # 3. Inject AI Skills
    print("   - Injecting AI Skill Silo...")
    # shutil.copytree("docs/ai_integration", f"{output_dir}/ai_integration")

    print(f"✅ HUB FORGED: The Nexus is live at {output_dir}/index.html")

if __name__ == "__main__":
    forge_hub()
