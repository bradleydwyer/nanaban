# Nano Banana CLI Tooling Landscape Analysis

Status: Active research — informing CLI tool design

## Context
Research to inform building a standalone CLI tool for Gemini image generation (Nano Banana 2 / Pro), integrated with Brad's Gemini skill in Claude Code. Similar to the equip tool research approach — understand the full landscape before building.

---

## CLI Tools (Standalone)

### Primary Contenders

| Tool | Language | Stars | Models | Install | Status |
|------|----------|-------|--------|---------|--------|
| [gemini-cli-extensions/nanobanana](https://github.com/gemini-cli-extensions/nanobanana) | TypeScript | ~920 | Flash, Pro, 2.5-flash | `gemini extensions install` | Active |
| [kingbootoshi/nano-banana-2-skill](https://github.com/kingbootoshi/nano-banana-2-skill) | TypeScript/Bun | ~197 | Flash, Pro, custom | `bun link` | Active |
| [dnvriend/gemini-nano-banana-tool](https://github.com/dnvriend/gemini-nano-banana-tool) | Python 3.14+ (Click) | ~1 | Flash, Pro, Imagen 4 (3 tiers) | `uv tool install .` | Active |
| [quinnypig/imagemage](https://github.com/quinnypig/imagemage) | Go | ~109 | Flash, Pro | Single binary | Active |
| [minimaxir/gemimg](https://github.com/minimaxir/gemimg) | Python | ~349 | Flash, Pro | pip | Active |
| [The-Focus-AI/nano-banana-cli](https://github.com/The-Focus-AI/nano-banana-cli) | TypeScript | ~12 | Flash, 2.5-flash | `npx @the-focus-ai/nano-banana` | Active |
| [erikceballos/nano-banana-cli](https://github.com/erikceballos/nano-banana-cli) | Go(?) | ~3 | Gemini (generic) | Binary download | Inactive |
| [simonliu-ai-product/simon-nb](https://github.com/simonliu-ai-product/simon-nb) | Python/FastAPI/ADK | ~9 | Gemini | `pip install -e .` | Active |
| [ryanbbrown/stylegen](https://github.com/ryanbbrown/stylegen) | Python | ~4 | Flash, Pro | pip | Active |
| [PederHP/image-gen-cli](https://github.com/PederHP/image-gen-cli) | C# | ~1 | Gemini + OpenAI + FLUX + Poe | dotnet | Active |

---

## Feature Matrix — CLI Tools

| Feature | nanobanana (ext) | nb2-skill | gemini-tool (dnvriend) | imagemage | gemimg | Focus-AI | stylegen |
|---------|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| **Generation** | | | | | | | |
| Text-to-image | Y | Y | Y | Y | Y | Y | Y |
| Image editing | Y | Y | Y | Y | — | Y | — |
| Image restoration | Y | — | — | Y | — | — | — |
| Icon generation | Y | — | — | Y | — | — | — |
| Pattern/texture creation | Y | — | — | Y | — | — | — |
| Diagram generation | Y | — | — | — | — | — | — |
| Story/sequence generation | Y | — | — | — | — | — | — |
| Video generation | — | — | — | — | — | Y | — |
| **Image Control** | | | | | | | |
| Aspect ratio control | — | Y (9) | Y (10) | — | Y | — | Y |
| Resolution control (512-4K) | — | Y | Y (1K/2K/4K) | — | — | — | Y |
| Reference images | — | Y | Y (3-14) | — | — | — | Y |
| Multiple ref images | — | Y | Y (up to 14) | — | — | — | Y |
| Transparent/green screen | — | Y (3-stage) | — | — | — | — | — |
| Background removal | — | — | — | — | — | — | — |
| Sprite sheet/grid | — | — | — | — | Y (up to 16) | — | — |
| Seed/reproducibility | — | — | — | — | — | — | — |
| **Prompt Features** | | | | | | | |
| AI prompt enhancement | — | — | Y (6 templates) | — | — | — | — |
| Prompt from file | — | — | Y | — | — | Y | — |
| Stdin piping | — | — | Y | — | — | — | — |
| **Output** | | | | | | | |
| JSON structured output | — | — | Y | — | — | — | Y |
| Cost tracking | — | Y | Y (per-token) | — | — | — | — |
| Cost estimation/dry-run | — | Y | Y | — | — | — | — |
| Auto-open image | — | — | — | — | — | — | — |
| Smart file naming | Y | — | Y | — | — | — | — |
| Batch processing | — | — | — | — | Y | Y | Y (parallel) |
| **Multi-turn** | | | | | | | |
| Conversational refinement | — | — | Y | — | — | — | — |
| Auto-reference previous output | — | — | Y | — | — | — | — |
| **Developer Experience** | | | | | | | |
| Verbose logging levels | — | — | Y (-v/-vv/-vvv) | — | — | — | — |
| Shell completion | — | — | Y | — | — | — | — |
| Dual library + CLI | — | — | Y | — | Y | — | — |
| Type-safe (mypy strict) | — | — | Y | — | — | — | — |
| **Models** | | | | | | | |
| NB2 (3.1 Flash Image) | Y | Y | Y | — | Y | Y | — |
| NB Pro (3 Pro Image) | Y | Y | Y | Y | Y | — | Y |
| 2.5 Flash Image | Y | — | Y (default) | — | — | Y | — |
| Imagen 4 (Fast/Std/Ultra) | — | — | Y | — | — | — | — |
| Custom model IDs | — | Y | — | — | — | — | — |
| **Integration** | | | | | | | |
| Gemini CLI extension | Y | — | — | — | — | — | — |
| Claude Code skill/plugin | — | Y | — | — | — | Y | — |
| MCP server | — | — | — | — | — | — | — |
| Library import | — | — | Y | — | Y | — | — |

---

## MCP Servers (Model Context Protocol)

| Server | Language | Stars | Key Differentiator |
|--------|----------|-------|--------------------|
| [zhongweili/nanobanana-mcp-server](https://github.com/zhongweili/nanobanana-mcp-server) | Python | ~181 | Multi-model, grounding, subject consistency, batch |
| [ConechoAI/Nano-Banana-MCP](https://github.com/ConechoAI/Nano-Banana-MCP) | TypeScript | ~126 | Iterative refinement, multi-ref images |
| [shinpr/mcp-image](https://github.com/shinpr/mcp-image) | TypeScript | ~83 | Auto prompt enhancement (Subject-Context-Style), quality presets |
| [brunoqgalvao/gemini-image-mcp-server](https://github.com/brunoqgalvao/gemini-image-mcp-server) | Python/JS | — | Dual deployment modes |
| [daveremy/nano-banana-2-mcp](https://github.com/daveremy/nano-banana-2-mcp) | TypeScript | — | NB2 default |
| [nanana-app/mcp-server-nano-banana](https://github.com/nanana-app/mcp-server-nano-banana) | TypeScript | — | Claude Desktop integration |
| [tasopen/mcp-alphabanana](https://github.com/tasopen/mcp-alphabanana) | TypeScript | ~3 | Transparency + resizing |
| 5+ more on npm | Various | — | Assorted wrappers |

**npm packages**: `nano-banana-mcp`, `@nanana-ai/mcp-server-nano-banana`, `@aeven/nanobanana-mcp`, `@lyalindotcom/nano-banana-mcp`, `@the-focus-ai/nano-banana`, `@ycse/nanobanana-mcp`, `@willh/nano-banana-mcp`, `@ansonzeng/nano-banana-mcp-server`

---

## Claude Code Skills

| Skill | Language | Stars | Key Differentiator |
|-------|----------|-------|--------------------|
| [kkoppenhaver/cc-nano-banana](https://github.com/kkoppenhaver/cc-nano-banana) | JS/Shell | ~185 | Full feature set (gen, edit, restore, icons, patterns) |
| [feedtailor/ccskill-nanobanana](https://github.com/feedtailor/ccskill-nanobanana) | Python 3.10+ | ~17 | Multi-resolution, up to 14 ref images |
| [guinacio/claude-image-gen](https://github.com/guinacio/claude-image-gen) | JavaScript | ~20 | One-step install (skill + CLI + MCP) |
| [AgriciDaniel/claude-banana](https://github.com/AgriciDaniel/claude-banana) | — | — | Creative Director mode with 6-component reasoning |
| [chongdashu/cc-skills-nanobananapro](https://github.com/chongdashu/cc-skills-nanobananapro) | — | — | ThreeJS support |
| [Ceeon/gemini-image-skill](https://github.com/Ceeon/gemini-image-skill) | Python | — | Basic text/image-to-image |
| [devonjones/skill-nano-banana](https://github.com/devonjones/skill-nano-banana) | Python | ~2 | Archived (moved to marketplace) |

---

## Other Ecosystems

### GUI Apps
| Tool | Stars | Type | Standout Feature |
|------|-------|------|-----------------|
| [NanoBananaEditor](https://github.com/markfulton/NanoBananaEditor) | ~645 | React web app | Region masks, version history, Konva.js canvas |
| [fal-nanobanana-studio](https://github.com/amrrs/fal-nanobanana-studio) | — | Web app | Photoshop-style UI |
| [Nano-Banana-Desktop](https://github.com/danielrosehill/Nano-Banana-Desktop) | — | Desktop | Inpainting utility |
| [duartium/nano-banana-desktop-editor](https://github.com/duartium/nano-banana-desktop-editor) | — | .NET Desktop | C# native |

### Code Editor Extensions
- [doggy8088/vscode-nanobanana](https://github.com/doggy8088/vscode-nanobanana) — VS Code, right-click generation, 12 styles, 7 languages
- [48Nauts-Operator/opencode-nanobanana](https://github.com/48Nauts-Operator/opencode-nanobanana) — OpenCode, image + video (Veo 3)

### Rust Crates
- `gems` — CLI + TUI + SDK for Gemini API with image gen
- `gemini-rust` / `gemini-rs` — Rust clients
- `gemini-watermark-removal` — Watermark utility
- [frostdev-ops/gemini-cli](https://github.com/frostdev-ops/gemini-cli) — Full Rust toolkit with MCP

### Specialized
- [GeminiWatermarkTool](https://github.com/allenk/GeminiWatermarkTool) — C++20, GPU-accelerated watermark removal, MCP server
- [Nano-PDF](https://github.com/gavrielc/Nano-PDF) — ~1.2k stars, NL PDF slide editing
- [SmartSolarium/ImagiTranslate](https://github.com/SmartSolarium/ImagiTranslate) — Image translation preserving layout
- Raycast extension exists (Clinton Halpin)
- No Alfred workflow found
- No Homebrew formula exists

### n8n Workflow Nodes
- `@prama13/n8n-nodes-nano-banana`
- `@nectopro/n8n-nodes-nano-banana`

### ComfyUI Nodes
- `ShmuelRonen/ComfyUI-NanoBanano`
- `ru4ls/ComfyUI_Nano_Banana` (NB2 + Pro, 14 refs, batch, grounding)
- `Tdragoni/Comfyui-NanoBananaProAPI`

---

## Key Observations

1. **Most feature-complete CLI**: `dnvriend/gemini-nano-banana-tool` (Python/Click) — prompt enhancement, multi-turn, JSON output, Imagen 4, verbose logging, library import, type-safe. Low stars but highest feature density.

2. **Most popular CLI**: `gemini-cli-extensions/nanobanana` (~920 stars) — but it's a Gemini extension, not standalone. Has the widest command set (generate, edit, restore, icons, patterns, stories, diagrams).

3. **Best transparency pipeline**: `kingbootoshi/nano-banana-2-skill` — 3-stage green screen removal (prompt augmentation → FFmpeg despill → ImageMagick auto-crop).

4. **Gap in the market**: No tool combines ALL of: standalone CLI + JSON output + cost tracking + prompt enhancement + multi-turn + transparency + reference images + verbose logging. The dnvriend tool comes closest but lacks transparency processing.

5. **MCP server saturation**: 10+ MCP servers exist. Building another would add little value. A standalone CLI that MCP servers/skills can shell out to is more useful.

6. **Language split**: TypeScript dominates (especially MCP/skills), Python is the CLI/library choice, Go has two entries (imagemage is notable at 109 stars for its single-binary approach).

---

## Recommended Approach

Build a standalone Python CLI (`nb`) that combines the best features from the landscape:

**From dnvriend/gemini-nano-banana-tool**: Click framework, JSON stdout, verbose logging, prompt enhancement, multi-turn, Imagen 4 support, library+CLI dual design, type-safe
**From kingbootoshi/nano-banana-2-skill**: Transparency pipeline, cost tracking with history, resolution control
**From nanobanana extension**: Wide command vocabulary (icons, patterns, diagrams)
**From imagemage**: Clean single-command UX
**Original**: Integration with Brad's Gemini skill via shell invocation

### Differentiators from existing tools
- Designed as a **Gemini skill companion** (JSON contract optimized for Claude Code consumption)
- Combines **prompt enhancement + transparency + cost tracking + multi-turn** (no existing tool has all four)
- **Pipeline-first** design (JSON stdout, stderr logs, exit codes)
- Owned and customizable

---

## Next Steps
1. Finalize which features to include in v0.1 vs later
2. Design the CLI interface
3. Implement
