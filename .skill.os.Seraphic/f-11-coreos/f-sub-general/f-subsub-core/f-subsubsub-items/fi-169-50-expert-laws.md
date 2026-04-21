---
id: fi-169-50-expert-laws.md
category: f-11-coreos
---

# 📜 50 EXPERT LAWS OF GITHUB SOVEREIGNTY

To maintain a world-class project, these 50 laws must be observed without exception.

## 🏛️ PART I: ARCHITECTURE & STRUCTURE
1.  **Law of the README:** The root `README.md` must contain a high-signal overview, architecture diagram, and "Quickstart."
2.  **License Finality:** Every repo MUST have a `LICENSE` file. Unlicensed code is "Void Code."
3.  **Virtual Manifests:** Use a virtual manifest for monorepos to prevent root pollution.
4.  **The `.gitignore` Seal:** Never commit `.env`, `target/`, or OS metadata (`.DS_Store`).
5.  **GitHub Actions Stratum:** Place all CI logic in `.github/workflows/`.
6.  **Issue Templates:** Force structured feedback using YAML issue templates.
7.  **Sovereign Labels:** Use color-coded, semantic labels (e.g., `tier:silicon`, `status:ascending`).
8.  **Project Boards:** Use GitHub Projects to automate task movement based on PR state.
9.  **CODEOWNERS Mandate:** Explicitly define who owns each crate path to ensure expert reviews.
10. **Branch Protection:** Require 1+ approval and passing status checks for `main`.

## 🚀 PART II: WORKFLOW & AUTOMATION
11. **Conventional Commits:** Use `feat:`, `fix:`, `docs:`, `perf:` prefixes.
12. **Linear History:** Rebase or Squash-Merge only. Reject merge-bubbles.
13. **Semantic Versioning:** Bump `vX.Y.Z` based on logic shifts, not calendar time.
14. **Automatic Changelog:** Generate `CHANGELOG.md` from commit messages.
15. **Stale Branch Pruning:** Delete branches immediately after merge.
16. **Draft PRs:** Use Draft mode for work-in-progress to prevent accidental merge.
17. **Dependency Upgrades:** Use `Renovate` or `Dependabot` with automated testing.
18. **Release Drafts:** Automate GitHub Release creation on tag push.
19. **Artifact Archiving:** Save build binaries for exactly 30 days in GitHub Actions.
20. **Self-Documenting CLI:** The repo must have a CLI (e.g., `cargo-smoothie`) for local automation.

## 🛡️ PART III: SECURITY & COMPLIANCE
21. **Secret Scanning:** Enable GitHub Secret Scanning to block leaked API keys.
22. **CodeQL Analysis:** Run static security analysis on every push to `main`.
23. **Dependency Review:** Audit new dependencies for malware/bloat in PRs.
24. **Signed Commits:** Enforce GPG/SSH signing for all developer commits.
25. **Security Policy:** Provide a `SECURITY.md` with a vulnerability disclosure path.
26. **Audit Logs:** Periodically review repo access and 2FA status of contributors.
27. **Pinned Actions:** Use SHA hashes instead of version tags for GitHub Actions (e.g., `v4.1.1` -> `sha...`).
28. **Environment Secrets:** Use GitHub Environments for production secrets.
29. **Code Sanitization:** Truncate log outputs in CI to prevent information leaks.
30. **No-Force-Push:** Disable force-pushing to protected branches.

## 🎨 PART IV: DOCUMENTATION & UI
31. **Mermaid Diagrams:** Use Mermaid in markdown for live architecture rendering.
32. **Table of Contents:** Every file >100 lines must have a TOC.
33. **Consistent Spacing:** Use Prettier to enforce 2-space or 4-space indenting across the repo.
34. **Contributor Guide:** Provide `CONTRIBUTING.md` with dev-setup instructions.
35. **Code of Conduct:** Establish the Seraphic persona expectations in `CODE_OF_CONDUCT.md`.
36. **Repository Social Preview:** Use a 1280x640 brand image for the repo preview.
37. **Discussions Hub:** Enable GitHub Discussions for RFCs and community support.
38. **Wiki Silo:** Move long-form tutorials to the GitHub Wiki or a dedicated `docs/` site.
39. **Hyper-linked APIs:** Cross-link `cargo doc` outputs into the README.
40. **Badge Sovereignty:** Use badges for Build Status, Coverage, and Versioning.

## 📈 PART V: EVOLUTION & SINGULARITY
41. **Weekly Audits:** Run `repo_auditor.py` every 7 days.
42. **Bottleneck Hunting:** Use CI timing logs to find slow test suites.
43. **Technical Debt Issues:** Tag debt explicitly and track its "Interest" (impact).
44. **Refactor Sprints:** Dedicate 1 in 4 iterations purely to structural cleanup.
45. **Agentic Integration:** Provide the `.Seraphic.skill` folder for all contributors.
46. **Zero-Warning CI:** CI must fail if a single warning exists in the build log.
47. **Code Coverage Floors:** Fail the build if coverage drops below the previous commit.
48. **PR Comment Finality:** All threads must be resolved before merging.
49. **Recursive Learning:** Document every major bug root-cause in `internals/`.
50. **The Ouroboros Seal:** A project is never "done"; it is only in a state of stable ascension.

---
*Finality and Sovereignty achieved.*
