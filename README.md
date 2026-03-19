<p align="center">
  <img src="assets/logo.png" alt="nanaban" width="256">
</p>

# nanaban

CLI for AI image generation. Supports Nano Banana 2 (Flash), Nano Banana Pro, and Imagen 4 (Fast/Standard/Ultra). Displays images inline in Kitty-capable terminals (Ghostty, Kitty).

## Install

```bash
brew install bradleydwyer/tap/nanaban
```

Or from source (Rust 1.85+):

```bash
cargo install --git https://github.com/bradleydwyer/nanaban
```

### API key

Requires a Gemini API key. Get one at [Google AI Studio](https://aistudio.google.com/apikey).

```bash
export GEMINI_API_KEY=your-key-here
```

Also accepts `GOOGLE_API_KEY` as a fallback.

## Usage

### Generate an image

```
$ nanaban generate "a pixel art banana character with a bow tie"
Estimated cost: ~$0.04 (flash)
/Users/you/nanaban_20260317_143022_a1b2c3.png
```

### Edit an image

```bash
nanaban edit photo.png "make it look like a watercolor painting"
nanaban edit logo.png "change the background to dark blue" -m pro
```

Note: editing is only supported with Nano Banana models (flash, pro). Imagen 4 is generation-only.

### Models

```
$ nanaban models
Name           API Identifier                             Cost  Description
-----------------------------------------------------------------------------------------------
flash          gemini-3.1-flash-image-preview           ~$0.04  Fast, cheap — Nano Banana 2 (default)
pro            gemini-3-pro-image-preview               ~$0.13  High quality — Nano Banana Pro
imagen-fast    imagen-4.0-fast-generate-001             ~$0.02  Fastest generation — Imagen 4 Fast
imagen         imagen-4.0-generate-001                  ~$0.04  Balanced quality/speed — Imagen 4
imagen-ultra   imagen-4.0-ultra-generate-001            ~$0.06  Highest quality — Imagen 4 Ultra
```

### Flags

```
-m, --model <MODEL>    Model: flash (default), pro, imagen-fast, imagen, imagen-ultra
-o, --output <PATH>    Output file path (default: auto-generated)
-a, --aspect <RATIO>   Aspect ratio: 1:1, 16:9, 9:16, 4:3, 3:4, etc.
-s, --size <SIZE>      Resolution: 512, 1K (default), 2K, 4K
-r, --ref <PATH>       Reference image(s), repeatable, up to 14
-p, --prompt-flag      Alternative to positional prompt, supports @file.txt
    --json             Structured JSON output to stdout
    --dry-run          Show cost estimate without calling the API
    --open             Also open in system viewer (Preview on macOS)
    --copy             Copy image to clipboard
-v, --verbose          Increase verbosity (-v, -vv, -vvv)
```

### Reference images

```bash
nanaban generate "portrait in this style" --ref style_reference.png
nanaban generate "combine these" --ref img1.png --ref img2.png --ref img3.png
```

### Prompt input

```bash
nanaban generate "a cat"                    # positional
nanaban generate -p "a cat"                 # flag
nanaban generate -p @prompt.txt             # from file
echo "a cat" | nanaban generate             # stdin
```

### JSON output

For programmatic use (e.g., from Claude Code):

```bash
nanaban generate "a banana" --json
```

```json
{
  "status": "success",
  "images": [{"path": "/abs/path.png", "width": 1024, "height": 1024}],
  "model": "gemini-3.1-flash-image-preview",
  "model_short": "flash",
  "elapsed_seconds": 12.3,
  "estimated_cost_usd": 0.04
}
```

## Claude Code Skill

nanaban includes a [skill](skill/SKILL.md) for Claude Code. Install it with [equip](https://github.com/bradleydwyer/equip):

```bash
equip install bradleydwyer/nanaban
```

This lets Claude Code generate and edit images directly when you ask it to create visual assets.

## Terminal image display

In Ghostty or Kitty, generated images display inline in the terminal. Other terminals fall back to opening in the system viewer.

| Terminal | Default | `--open` | `--copy` |
|----------|---------|----------|----------|
| Ghostty / Kitty | Inline display | Inline + Preview | Copies to clipboard |
| macOS Terminal / other | Opens Preview | Opens Preview | Copies to clipboard |
| `--json` mode | No display | No display | No copy |

## License

MIT

## More Tools

**Naming & Availability**
- [available](https://github.com/bradleydwyer/available) — AI-powered project name finder (uses parked, staked & published)
- [parked](https://github.com/bradleydwyer/parked) — Domain availability checker (DNS → WHOIS → RDAP)
- [staked](https://github.com/bradleydwyer/staked) — Package registry name checker (npm, PyPI, crates.io + 19 more)
- [published](https://github.com/bradleydwyer/published) — App store name checker (App Store & Google Play)

**AI Tooling**
- [sloppy](https://github.com/bradleydwyer/sloppy) — AI prose/slop detector
- [caucus](https://github.com/bradleydwyer/caucus) — Multi-LLM consensus engine
- [equip](https://github.com/bradleydwyer/equip) — Cross-agent skill manager
