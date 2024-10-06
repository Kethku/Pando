# Aspen

Aspen is a simple UI framework based on Neovide's new wgpu
renderer Vide and the Winit windowing library. Aspen takes
ideas for it's design from Zed's gpui (contexts as the 
primary interface to the OS), Flutter (update -> layout -> 
draw flow), and Druid (immediate mode graphics as a 
viable ui framework primitive) and tries to be as simple as 
possible while still being powerful enough to make real apps.

## Concepts

### Vide

Aspen uses Vide for all of it's rendering. Vide is built on
a scene model where elements are added in draw order to
layers. Examples for its usage can be found in the Vide test
suite [here](https://github.com/neovide/vide/blob/main/src/test.rs).

### Element

The `Element` trait is the core of Aspen. Every UI component
must implement `Element` in order to be drawn and updated on
screen. Further, `Element`s are stored in `ElementPointer`s
which are used to manage id's and keep track of framework
specific state by element.

### Contexts

The `Element` trait has 3 functions on it: `update`,
`layout`, and `draw` each of which have an associated
context passed as an argument. `Element`s can use these
contexts to interact with the OS, access user input, and
trigger redraws among other things.

### Flow

Aspen calls `update`, `layout` and `draw` in that order on
the root element. It is then up to the root element to call
`update`, `layout`, and `draw` on each of it's children.

`update` is used to give the app a chance to make state
changes.

`layout` each element must report the size that it will take
up given a minimum and maximum bound. Further, each child of
the `element` will return a `LayoutResult` which reports its
size to its parent. `position` must be called on each
`LayoutResult` to place the child relative to it's parent.

Min and Max size bounds are passed down the tree, dimensions
are passed back up through the tree backwords, and final
positions are computed by adjusting all of the child regions
as the `Element` tree is traversed back up.

Finally `draw` is called to give the tree a chance to draw
to the `DrawContext`'s `vide::scene::Scene`. The final
computed region from the `layout` phase can be accessed by
any element in the tree via the `region` function.

### Mouse Region

Mouse input is managed either by functions on the various
`Context` functions or by the higher level `MouseRegion`
structs. The basic idea is that during the `draw` phase,
`Element`s can call `DrawContext::add_mouse_region` to claim
a part of the window as a region that it is interested in
for various mouse events. 

For example, this is the `Button` element's usage of Mouse
Regions to manage it's hover states and click functionality:

```rust
cx.add_mouse_region(
    MouseRegion::new(cx.token(), cx.region())
        .on_hover({
            let state = self.state.clone();
            move |_cx| {
                let mut state = state.borrow_mut();
                if !state.hovered {
                    state.hovered = true;
                    state.hover_start = Instant::now();
                }
            }
        })
        .on_leave({
            let state = self.state.clone();
            move |_cx| {
                let mut state = state.borrow_mut();
                if state.hovered {
                    state.hovered = false;
                    state.hover_start = Instant::now();
                }
            }
        })
        .on_clicked({
            let state = self.state.clone();
            move |cx| {
                let state = state.borrow();
                (state.on_clicked)(cx);
            }
        }),
);
```

The neat part about this approach is that the mouse region's
are processed in draw order, so the last region to be drawn
in a given area will get the mouse events first. This makes
mouse handling predictable and easy to reason about.
