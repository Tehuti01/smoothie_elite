import os
import shutil
import hashlib
import re

ROOT = '.skill.os.Seraphic'
BACKUP = '.skill.os.Seraphic.backup'

def get_hash(filepath):
    hasher = hashlib.md5()
    try:
        with open(filepath, 'rb') as f:
            buf = f.read()
            hasher.update(buf)
        return hasher.hexdigest()
    except Exception:
        return None

def main():
    if os.path.exists(BACKUP):
        shutil.rmtree(BACKUP)
    shutil.copytree(ROOT, BACKUP)
    
    # Collect all files
    all_files = []
    for root, dirs, files in os.walk(ROOT):
        if '/target/' in root or '/.git/' in root:
            continue
        for file in files:
            all_files.append(os.path.join(root, file))
            
    # Find duplicates
    hashes = {}
    unique_files = []
    duplicates = []
    for f in all_files:
        if 'target/' in f: continue
        h = get_hash(f)
        if not h: continue
        if h in hashes:
            duplicates.append(f)
        else:
            hashes[h] = f
            unique_files.append(f)
            
    print(f"Total files: {len(all_files)}")
    print(f"Unique files: {len(unique_files)}")
    print(f"Duplicates: {len(duplicates)}")
    
    # Create categories
    categories = [
        ("f-01-secbrain", "Second Brain"),
        ("f-02-math", "Mathematics & Physics"),
        ("f-03-frontend", "Frontend & UI"),
        ("f-04-backend", "Backend & APIs"),
        ("f-05-sysarch", "System Architecture"),
        ("f-06-dsp", "DSP & Audio"),
        ("f-07-testing", "Testing & QA"),
        ("f-08-security", "Security"),
        ("f-09-tools", "Tools & Frameworks"),
        ("f-10-devops", "DevOps & CI"),
        ("f-11-coreos", "Core OS"),
        ("f-12-meta", "Meta & Scripts")
    ]
    
    NEW_ROOT = '.skill.os.Seraphic_new'
    if os.path.exists(NEW_ROOT):
        shutil.rmtree(NEW_ROOT)
    os.makedirs(NEW_ROOT)
    
    # Create Root Files
    desc_content = "# Seraphic OS Description\nThis is the Seraphic Skill OS. It contains categorized skills for the autonomous agent. Read `fi-01-manual.md` next."
    manual_content = "# Seraphic Manual\nThis folder contains 12 categories of skills. Read `fi-02-constants.md` for constant instructions."
    constants_content = "# Constant Instructions\nThese skills are triggered every message. Read `fi-03-index.md` for the index."
    index_content = "# Skill Index\nLibrary of all skills.\n"
    
    with open(os.path.join(NEW_ROOT, "fi-00-description.md"), 'w') as f: f.write(desc_content)
    with open(os.path.join(NEW_ROOT, "fi-01-manual.md"), 'w') as f: f.write(manual_content)
    with open(os.path.join(NEW_ROOT, "fi-02-constants.md"), 'w') as f: f.write(constants_content)
    with open(os.path.join(NEW_ROOT, "fi-03-index.md"), 'w') as f: f.write(index_content)
    
    # Signature
    const_dir = os.path.join(NEW_ROOT, "f-00-constant")
    os.makedirs(const_dir)
    if os.path.exists("signature.md"):
        shutil.copy("signature.md", os.path.join(const_dir, "fi-00-signature.skill"))
        
    # Process unique files and move them
    # For simplicity, map them based on keywords
    cat_map = {
        'brain': 'f-01-secbrain', 'llm': 'f-01-secbrain', 'prompt': 'f-01-secbrain', 'agent': 'f-01-secbrain',
        'math': 'f-02-math', 'astro': 'f-02-math', 'physics': 'f-02-math', 'geom': 'f-02-math', 'celestial': 'f-02-math',
        'react': 'f-03-frontend', 'ui': 'f-03-frontend', 'front': 'f-03-frontend', 'holo': 'f-03-frontend',
        'node': 'f-04-backend', 'express': 'f-04-backend', 'api': 'f-04-backend', 'backend': 'f-04-backend',
        'arch': 'f-05-sysarch', 'rust': 'f-05-sysarch', 'cargo': 'f-05-sysarch', 'system': 'f-05-sysarch',
        'dsp': 'f-06-dsp', 'audio': 'f-06-dsp', 'simd': 'f-06-dsp', 'spectral': 'f-06-dsp',
        'test': 'f-07-testing', 'qa': 'f-07-testing', 'valid': 'f-07-testing',
        'sec': 'f-08-security', 'auth': 'f-08-security', 'crypto': 'f-08-security',
        'tool': 'f-09-tools', 'frame': 'f-09-tools',
        'devops': 'f-10-devops', 'pipe': 'f-10-devops', 'ci': 'f-10-devops',
        'os': 'f-11-coreos', 'core': 'f-11-coreos',
        'meta': 'f-12-meta', 'script': 'f-12-meta'
    }
    
    file_id = 100
    for f in unique_files:
        name = os.path.basename(f).lower()
        path_lower = f.lower()
        
        target_cat = 'f-12-meta'
        for kw, cat in cat_map.items():
            if kw in path_lower:
                target_cat = cat
                break
                
        # Generate nested structure (3 layers)
        # We'll use static layers for now to guarantee 3 layers
        sub = "f-sub-general"
        subsub = "f-subsub-core"
        subsubsub = "f-subsubsub-items"
        
        dest_dir = os.path.join(NEW_ROOT, target_cat, sub, subsub, subsubsub)
        os.makedirs(dest_dir, exist_ok=True)
        
        # Clean up filename
        safe_name = re.sub(r'[^a-z0-9.]', '-', name)
        new_name = f"fi-{file_id}-{safe_name}"
        if not new_name.endswith('.md') and not new_name.endswith('.rs') and not new_name.endswith('.py') and not new_name.endswith('.json') and not new_name.endswith('.sh') and not new_name.endswith('.skill'):
            new_name += '.md'
            
        # Add template to file if it's text
        try:
            with open(f, 'r') as src:
                content = src.read()
            with open(os.path.join(dest_dir, new_name), 'w') as dst:
                dst.write(f"---\nid: {new_name}\ncategory: {target_cat}\n---\n\n" + content)
        except UnicodeDecodeError:
            shutil.copy(f, os.path.join(dest_dir, new_name))
            
        file_id += 1
        
    print("Done restructuring. Removing old and renaming new.")
    shutil.rmtree(ROOT)
    os.rename(NEW_ROOT, ROOT)
    
if __name__ == "__main__":
    main()
