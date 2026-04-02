# Roadmap

## What is stable today

- `.gui` parsing, imports, merging, and validation
- CLI inspection commands
- HTML scan for page, section, layout, action, index, and dialog nodes
- nav extraction with several over-detection guards
- dialog trigger extraction and shared-layout promotion

## Current weak points

- ranking primary vs auxiliary navigation
- consolidating multiple URLs that represent one logical page
- distinguishing docs taxonomy from site-wide navigation at larger scale
- recognizing JS-only modal triggers without semantic attributes
- richer dialog ownership inference beyond page/layout scope
- inferring common layouts from shared DOM structure instead of path heuristics alone

## Recommended solutions

- Add an alias-normalization stage between scan and final abstraction.
  - Use `canonical`, `og:url`, normalized path, title similarity, and breadcrumb
    evidence to collapse multiple URLs into one logical page id.
- Rank navigation clusters before emitting them.
  - Score each nav by position, recurrence across pages, target-set stability,
    active-state signals, and accessibility labels.
  - Classify nav as `primary`, `secondary`, `footer`, or `local`.
- Preserve large documentation structures as taxonomy rather than dropping them.
  - Add a dedicated `taxonomy`-like classification instead of forcing every
    large index into `page` or suppressing it entirely.
- Add a stronger layout-inference stage based on DOM fingerprints.
  - Compare repeated non-root subtrees across pages to infer shared layouts,
    instead of relying mainly on first path segment and shared nav targets.
- Extend dialog trigger inference with confidence-based heuristics.
  - Inspect `onclick`, `data-*`, nearby labels, and id similarity, but emit a
    confidence signal so weak guesses are distinguishable from semantic links.

## Priority order

1. Alias normalization
2. Navigation ranking
3. Taxonomy classification for docs-style structures
4. DOM-fingerprint layout inference
5. Confidence-based dialog trigger inference

## Implementation tasks

- Introduce an explicit `normalize` stage in the scan pipeline.
  - `scan -> normalize -> classify -> abstract`
- Define intermediate records for:
  - page aliases
  - nav scores and nav roles
  - taxonomy candidates
  - subtree fingerprints
  - dialog trigger confidence
- Add fixture sets for at least:
  - marketing site
  - docs site
  - commerce site
  - app/dashboard site
- Add debug-oriented output for intermediate scan stages so heuristics can be
  inspected without patching source.

## Likely next steps

- stronger nav ranking and grouping
- page alias normalization
- broader dialog trigger heuristics
- richer docs/site taxonomy handling
- optional debug output for intermediate scan stages
