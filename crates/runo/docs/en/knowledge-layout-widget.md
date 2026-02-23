# Layout and Widget Design

## 1. Layout strategies

Common strategies:

1. Flow/Stack (simple vertical/horizontal)
2. Flex (space distribution)
3. Constraint-based layouts (powerful but complex)

A practical start is Stack + minimal Flex behavior.

## 2. Measure and arrange

Use two-phase layout:

1. Measure: child reports desired size
2. Arrange: parent assigns final rect

This keeps variable text/content handling stable.

## 3. Widget API principles

1. Declarative and readable
2. Safe defaults
3. Separate identity (ID) from style/content
4. Separate layout concerns from paint concerns

## 4. Stateful widgets

Widgets beyond buttons require richer state:

1. Text fields: value, caret, selection, composition
2. Scroll views: viewport, offset, inertia
3. Select/combo: open state, highlighted item

## 5. Styling model

Typical options:

1. Direct builder properties
2. Theme object
3. CSS-like selector system

For early stages, builder + theme is usually enough.

## 6. Widget addition checklist

1. Add node type
2. Add state upsert APIs
3. Add input handling rules
4. Add paint logic
5. Add UI/builder API
6. Add examples/tests
