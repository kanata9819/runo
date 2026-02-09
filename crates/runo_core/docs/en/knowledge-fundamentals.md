# GUI Library Fundamentals

## 1. Main responsibilities

A GUI library usually needs:

1. Window/event-loop management
2. Input normalization
3. UI state management
4. Layout computation
5. Draw command generation
6. GPU/OS presentation

## 2. Immediate vs retained mode

1. Immediate mode
   Re-declare UI every frame; simple and explicit
2. Retained mode
   Keep UI tree/state over time; scalable for larger systems

Many production systems are hybrid.

## 3. State layering

Keep layers separated:

1. App/domain state
2. Widget interaction state (hover/focus/pressed/scroll)
3. Rendering state (font cache, textures, GPU resources)

## 4. IDs and identity

Stable, unique IDs are critical in retained systems:

1. Stable across frames
2. Unique within scope
3. Reproducible generation rules

## 5. Typical frame stages

1. Input collection
2. State update
3. Layout
4. Paint command recording
5. GPU submit
6. Frame cleanup

## 6. Common pitfalls

1. Mixing input and rendering concerns
2. Unstable or colliding widget IDs
3. Blending layout and paint responsibilities
4. Unclear cache invalidation rules
