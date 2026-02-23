# Rendering Pipeline and Performance

## 1. Pipeline overview

1. Build scene/draw commands
2. Rasterize/compose
3. Submit and present

## 2. High-impact optimizations

1. Cache reusable resources (fonts, glyphs, paths)
2. Prefer incremental updates
3. Reduce overdraw and hidden work
4. Batch similar operations

## 3. CPU/GPU responsibility split

1. CPU: input, state, layout, command recording
2. GPU: rasterization and composition

A clear split prevents bottlenecks.

## 4. Frame budget thinking

At 60 FPS, budget is ~16.6ms/frame.
Measure segments separately:

1. update
2. layout
3. paint record
4. GPU submit/present

## 5. DPI and coordinate safety

1. Keep logical/physical units explicit
2. Align hit-test coordinates with draw coordinates
3. Validate text size/scaling under DPI changes

## 6. Practical rule

1. Correctness first
2. Instrumentation second
3. Micro-optimizations third
4. Architectural optimization last
