---
name: nanaban
description: Use this skill any time the user wants to CREATE or EDIT an image file. This is Claude's only way to generate images or modify photos — there is no other tool for this. Covers generating illustrations, logos, icons, avatars, social media graphics, pixel art, banners, or any visual artwork; editing or transforming existing photos (style transfer, background removal, color correction, restoration, retouching); creating visual assets at specific sizes or aspect ratios. Trigger on any request whose end result is a new or modified image file — "generate art", "edit this photo", "create an icon", "restore this picture", "remove the background", "make this look like a painting", etc. Do NOT trigger for image-related code tasks (file optimization, lazy loading, resizing via code libraries), converting mockups to code, architectural diagrams, or image analysis.
allowed-tools:
  - Bash(nanaban:*)
  - Bash(source:*)
  - Read
user-invocable: true
argument-hint: '"prompt to generate" or edit image.png "instructions"'
---

# nanaban — Gemini Image Generation CLI

Generate and edit images via the Gemini API using the `nanaban` CLI.

## Generating an Image

Run nanaban with `--json` so you can parse the result. Craft a detailed prompt — the model responds better to specifics about subject, style, composition, lighting, and colors than to vague descriptions.

```bash
nanaban generate "a detailed prompt describing what you want" --json 2>/dev/null
```

Parse the JSON from stdout. On success, read the image file at `images[0].path` using the Read tool to show the user what was generated.

```json
{
  "status": "success",
  "images": [{"path": "/absolute/path/to/image.png", "width": 1024, "height": 1024}],
  "text": "optional model commentary",
  "model_short": "flash",
  "elapsed_seconds": 12.3,
  "estimated_cost_usd": 0.04
}
```

On error, `status` will be `"error"` with an `error` field describing what went wrong.

## Editing an Image

Pass the source image as the first argument, then the edit instructions:

```bash
nanaban edit /path/to/source.png "describe the changes you want" --json 2>/dev/null
```

The JSON response is the same format. The edited image is saved to a new file (the original is not modified).

## Editing Limitation

Editing (`nanaban edit`) only works with Nano Banana models (flash, pro). Imagen 4 models are generation-only.

## Choosing a Model

| Flag | Model | Cost | When to use |
|------|-------|------|-------------|
| `-m flash` | Nano Banana 2 | ~$0.04 | Default — good for most tasks, supports editing |
| `-m pro` | Nano Banana Pro | ~$0.13 | High quality, fine detail, professional output, supports editing |
| `-m imagen-fast` | Imagen 4 Fast | ~$0.02 | Cheapest generation, no editing |
| `-m imagen` | Imagen 4 | ~$0.04 | Balanced quality/speed, no editing |
| `-m imagen-ultra` | Imagen 4 Ultra | ~$0.06 | Highest quality generation, no editing |

Use `flash` by default. Switch to `pro` when the user asks for "high quality", "professional", "detailed", or similar. Use Imagen models when the user specifically asks for Imagen, or when they want the cheapest possible generation (`imagen-fast` at $0.02).

## Useful Flags

Add these based on what the user asks for:

| Flag | When to add |
|------|-------------|
| `-a 16:9` | User wants landscape, banner, wallpaper, widescreen |
| `-a 9:16` | User wants portrait, phone wallpaper, story format |
| `-a 1:1` | User wants square (this is the default) |
| `-s 2K` or `-s 4K` | User wants higher resolution |
| `-r ref.png` | User provides a style reference or wants "in the style of" |
| `--copy` | User wants to paste the image somewhere |
| `--open` | User wants to see it in Preview |
| `-o filename.png` | User specifies an output filename |
| `--dry-run` | User asks about cost before generating |

Multiple reference images: use `-r` repeatedly, up to 14 images.

## After Generation

1. Parse the JSON to get the image path
2. Read the image file with the Read tool to show it to the user
3. Tell the user the file path, dimensions, and cost
4. If the user isn't happy, refine the prompt and generate again — iteration is cheap at $0.04/image

## Crafting Prompts

The image model responds well to specific, descriptive prompts. When the user gives a vague request like "make me a logo", expand it into something richer before passing it to nanaban. For example:

- User says: "make me a cat picture"
- Better prompt: "a fluffy orange tabby cat sitting on a windowsill, soft natural lighting, shallow depth of field, cozy home interior"

Include details about: subject, style, composition, lighting, colors, mood, and any text that should appear. But don't over-engineer — start with a good first attempt and iterate based on user feedback.
