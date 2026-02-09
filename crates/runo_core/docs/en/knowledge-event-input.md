# Event Loop and Input Model

## 1. Event loop basics

The loop receives and dispatches:

1. Window events (resize, close, minimize)
2. Pointer events (move/down/up/wheel)
3. Keyboard events
4. Text input (including IME)

## 2. Input normalization

Normalize raw events into frame-level state:

1. Instant flags: `pressed`, `released`
2. Persistent state: `is_down`, cursor position
3. Derived state: `clicked`, `dragging`, double-click

## 3. Hit-testing

Determine event target by:

1. Z-order
2. Visibility/disabled state
3. Clipping bounds
4. Pointer capture rules

## 4. Focus model

Keyboard and accessibility depend on focus:

1. Focused node ID
2. Tab navigation rules
3. Mouse-to-focus transition
4. Focus cleanup on removal

## 5. Pointer capture

Needed for consistent drag behavior:

1. Capture on pointer down
2. Keep routing move/up to captured target
3. Release on up/cancel

## 6. Input quality checks

1. Press/release consistency per frame
2. No coordinate drift under high DPI
3. Stable behavior after minimize/restore
4. Defined behavior for multi-button input
