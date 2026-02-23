# Testing Strategy and Release Checklist

## 1. What to test

Split tests into layers:

1. Pure logic
   layout math, ID generation, input transitions
2. Integration behavior
   event -> state update -> draw command flow
3. Visual/manual checks
   spacing, text quality, hover/focus states

## 2. Critical input scenarios

1. Basic click
2. press -> move out -> release
3. Drag start/end behavior
4. Resize during interaction
5. Minimize/restore consistency

## 3. Layout scenarios

1. Vertical/horizontal spacing
2. Variable text lengths
3. Zero/extreme sizes
4. DPI scale changes

## 4. Snapshot/golden testing

Useful but environment-sensitive:

1. Fix fonts
2. Fix backend/config
3. Use threshold-based comparison when needed

## 5. API compatibility discipline

1. Separate experimental vs stable API
2. Deprecate before breaking
3. Keep changelog explicit

## 6. Pre-release checklist

1. Examples run correctly
2. Core docs are updated
3. Widget interaction is regression-checked
4. No major perf regression
5. Dependency updates reviewed
