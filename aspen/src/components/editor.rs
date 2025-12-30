use core::num::NonZeroUsize;

use parley::{
    editing::{Cursor, Selection},
    layout::Affinity,
    style::StyleProperty,
    BoundingBox, Layout,
};
use vello::{
    kurbo::{Point, Rect, Size},
    peniko::Brush,
};
use winit::{
    event::{ElementState, Modifiers},
    keyboard::{Key, NamedKey},
};

use crate::{
    context_stack::{Context, DrawContext, LayoutContext, UpdateContext},
    element::{Element, ElementPointer},
};

pub struct Editor {
    text_stroke: Brush,
    selection_fill: Brush,
    cursor_stroke: Brush,
    size: f32,
}

#[derive(Default)]
pub struct EditorState {
    layout: Layout<Brush>,
    previous_max_width: f64,
    buffer: String,
    selection: Selection,
    layout_dirty: bool,
}

impl Editor {
    pub fn new<'a>(
        text: String,
        size: f32,
        text_stroke: Brush,
        selection_fill: Brush,
        cursor_stroke: Brush,
        cx: &Context<'a>,
    ) -> ElementPointer<Self> {
        ElementPointer::new(Self {
            text_stroke,
            selection_fill,
            cursor_stroke,
            size,
        })
        .insert_state(
            EditorState {
                layout: Layout::default(),
                previous_max_width: f64::INFINITY,
                buffer: text,
                selection: Default::default(),
                layout_dirty: true,
            },
            cx,
        )
    }
}

impl EditorState {
    /// Make a cursor at a given byte index.
    fn cursor_at(&self, index: usize) -> Cursor {
        // TODO: Do we need to be non-dirty?
        // FIXME: `Selection` should make this easier
        if index >= self.buffer.len() {
            Cursor::from_byte_index(&self.layout, self.buffer.len(), Affinity::Upstream)
        } else {
            Cursor::from_byte_index(&self.layout, index, Affinity::Downstream)
        }
    }

    /// Update the selection
    fn set_selection(&mut self, new_sel: Selection) {
        self.selection = new_sel;
    }

    /// Insert at cursor, or replace selection.
    pub fn insert_or_replace_selection(&mut self, s: &str) {
        let range = self.selection.text_range();
        let start = range.start;
        if self.selection.is_collapsed() {
            self.buffer.insert_str(start, s);
        } else {
            self.buffer.replace_range(range.clone(), s);
        }

        self.layout_dirty = true;
        let new_index = start.saturating_add(s.len());
        let affinity = if s.ends_with("\n") {
            Affinity::Downstream
        } else {
            Affinity::Upstream
        };
        self.set_selection(Cursor::from_byte_index(&self.layout, new_index, affinity).into());
    }

    /// Delete the selection.
    pub fn delete_selection(&mut self) {
        self.insert_or_replace_selection("");
    }

    /// Delete the specified numbers of bytes before the selection.
    /// The selection is moved to the left by that number of bytes
    /// but otherwise unchanged.
    ///
    /// The deleted range is clamped to the start of the buffer.
    /// No-op if the start of the range is not a char boundary.
    pub fn delete_bytes_before_selection(&mut self, len: NonZeroUsize) {
        let old_selection = self.selection;
        let selection_range = old_selection.text_range();
        let range = selection_range.start.saturating_sub(len.get())..selection_range.start;
        if range.is_empty() || !self.buffer.is_char_boundary(range.start) {
            return;
        }
        self.buffer.replace_range(range.clone(), "");
        self.layout_dirty = true;
        let old_anchor = old_selection.anchor();
        let old_focus = old_selection.focus();
        // When doing the equivalent of a backspace on a collapsed selection,
        // always use downstream affinity, as `backdelete` does.
        let (anchor_affinity, focus_affinity) = if old_selection.is_collapsed() {
            (Affinity::Downstream, Affinity::Downstream)
        } else {
            (old_anchor.affinity(), old_focus.affinity())
        };
        self.set_selection(Selection::new(
            Cursor::from_byte_index(
                &self.layout,
                old_anchor.index() - range.len(),
                anchor_affinity,
            ),
            Cursor::from_byte_index(
                &self.layout,
                old_focus.index() - range.len(),
                focus_affinity,
            ),
        ));
    }

    /// Delete the selection or the next cluster (typical ‘delete’ behavior).
    pub fn delete(&mut self) {
        if self.selection.is_collapsed() {
            // Upstream cluster range
            if let Some(range) = self.selection.focus().logical_clusters(&self.layout)[1]
                .as_ref()
                .map(|cluster| cluster.text_range())
                .and_then(|range| (!range.is_empty()).then_some(range))
            {
                self.buffer.replace_range(range.clone(), "");
                self.layout_dirty = true;
            }
        } else {
            self.delete_selection();
        }
    }

    /// Delete the selection or up to the next word boundary (typical ‘ctrl + delete’ behavior).
    pub fn delete_word(&mut self) {
        if self.selection.is_collapsed() {
            let focus = self.selection.focus();
            let start = focus.index();
            let end = focus.next_logical_word(&self.layout).index();
            if self.buffer.get(start..end).is_some() {
                self.buffer.replace_range(start..end, "");
                self.layout_dirty = true;
                self.set_selection(
                    Cursor::from_byte_index(&self.layout, start, Affinity::Downstream).into(),
                );
            }
        } else {
            self.delete_selection();
        }
    }

    /// Delete the selection or the previous cluster (typical ‘backspace’ behavior).
    pub fn backdelete(&mut self) {
        if self.selection.is_collapsed() {
            // Upstream cluster
            if let Some(cluster) = self.selection.focus().logical_clusters(&self.layout)[0].clone()
            {
                let range = cluster.text_range();
                let end = range.end;
                let start = if cluster.is_hard_line_break() || cluster.is_emoji() {
                    // For newline sequences and emoji, delete the previous cluster
                    range.start
                } else {
                    // Otherwise, delete the previous character
                    let Some((start, _)) = self
                        .buffer
                        .get(..end)
                        .and_then(|str| str.char_indices().next_back())
                    else {
                        return;
                    };
                    start
                };
                self.buffer.replace_range(start..end, "");
                self.layout_dirty = true;
                self.set_selection(
                    Cursor::from_byte_index(&self.layout, start, Affinity::Downstream).into(),
                );
            }
        } else {
            self.delete_selection();
        }
    }

    /// Delete the selection or back to the previous word boundary (typical ‘ctrl + backspace’ behavior).
    pub fn backdelete_word(&mut self) {
        if self.selection.is_collapsed() {
            let focus = self.selection.focus();
            let end = focus.index();
            let start = focus.previous_logical_word(&self.layout).index();
            if self.buffer.get(start..end).is_some() {
                self.buffer.replace_range(start..end, "");
                self.layout_dirty = true;
                self.set_selection(
                    Cursor::from_byte_index(&self.layout, start, Affinity::Downstream).into(),
                );
            }
        } else {
            self.delete_selection();
        }
    }

    // --- MARK: Cursor Movement ---
    /// Move the cursor to the cluster boundary nearest this point in the layout.
    pub fn move_to_point(&mut self, point: Point) {
        self.set_selection(Selection::from_point(
            &self.layout,
            point.x as f32,
            point.y as f32,
        ));
    }

    /// Move the cursor to a byte index.
    ///
    /// No-op if index is not a char boundary.
    pub fn move_to_byte(&mut self, index: usize) {
        if self.buffer.is_char_boundary(index) {
            self.set_selection(self.cursor_at(index).into());
        }
    }

    /// Move the cursor to the start of the buffer.
    pub fn move_to_text_start(&mut self) {
        self.set_selection(self.selection.move_lines(&self.layout, isize::MIN, false));
    }

    /// Move the cursor to just after the previous hard line break (such as `\n`).
    pub fn move_to_hard_line_start(&mut self) {
        self.set_selection(self.selection.hard_line_start(&self.layout, false));
    }

    /// Move the cursor to the start of the physical line.
    pub fn move_to_line_start(&mut self) {
        self.set_selection(self.selection.line_start(&self.layout, false));
    }

    /// Move the cursor to the end of the buffer.
    pub fn move_to_text_end(&mut self) {
        self.set_selection(self.selection.move_lines(&self.layout, isize::MAX, false));
    }

    /// Move the cursor to just before the next hard line break (such as `\n`).
    pub fn move_to_hard_line_end(&mut self) {
        self.set_selection(self.selection.hard_line_end(&self.layout, false));
    }

    /// Move the cursor to the end of the physical line.
    pub fn move_to_line_end(&mut self) {
        self.set_selection(self.selection.line_end(&self.layout, false));
    }

    /// Move up to the closest physical cluster boundary on the previous line, preserving the horizontal position for repeated movements.
    pub fn move_up(&mut self) {
        self.set_selection(self.selection.previous_line(&self.layout, false));
    }

    /// Move down to the closest physical cluster boundary on the next line, preserving the horizontal position for repeated movements.
    pub fn move_down(&mut self) {
        self.set_selection(self.selection.next_line(&self.layout, false));
    }

    /// Move to the next cluster left in visual order.
    pub fn move_left(&mut self) {
        self.set_selection(self.selection.previous_visual(&self.layout, false));
    }

    /// Move to the next cluster right in visual order.
    pub fn move_right(&mut self) {
        self.set_selection(self.selection.next_visual(&self.layout, false));
    }

    /// Move to the next word boundary left.
    pub fn move_word_left(&mut self) {
        self.set_selection(self.selection.previous_visual_word(&self.layout, false));
    }

    /// Move to the next word boundary right.
    pub fn move_word_right(&mut self) {
        self.set_selection(self.selection.next_visual_word(&self.layout, false));
    }

    /// Select the whole buffer.
    pub fn select_all(&mut self) {
        self.set_selection(
            Selection::from_byte_index(&self.layout, 0_usize, Affinity::default()).move_lines(
                &self.layout,
                isize::MAX,
                true,
            ),
        );
    }

    /// Collapse selection into caret.
    pub fn collapse_selection(&mut self) {
        self.set_selection(self.selection.collapse());
    }

    /// Move the selection focus point to the start of the buffer.
    pub fn select_to_text_start(&mut self) {
        self.set_selection(self.selection.move_lines(&self.layout, isize::MIN, true));
    }

    /// Move the selection focus point to just after the previous hard line break (such as `\n`).
    pub fn select_to_hard_line_start(&mut self) {
        self.set_selection(self.selection.hard_line_start(&self.layout, true));
    }

    /// Move the selection focus point to the start of the physical line.
    pub fn select_to_line_start(&mut self) {
        self.set_selection(self.selection.line_start(&self.layout, true));
    }

    /// Move the selection focus point to the end of the buffer.
    pub fn select_to_text_end(&mut self) {
        self.set_selection(self.selection.move_lines(&self.layout, isize::MAX, true));
    }

    /// Move the selection focus point to just before the next hard line break (such as `\n`).
    pub fn select_to_hard_line_end(&mut self) {
        self.set_selection(self.selection.hard_line_end(&self.layout, true));
    }

    /// Move the selection focus point to the end of the physical line.
    pub fn select_to_line_end(&mut self) {
        self.set_selection(self.selection.line_end(&self.layout, true));
    }

    /// Move the selection focus point up to the nearest cluster boundary on the previous line, preserving the horizontal position for repeated movements.
    pub fn select_up(&mut self) {
        self.set_selection(self.selection.previous_line(&self.layout, true));
    }

    /// Move the selection focus point down to the nearest cluster boundary on the next line, preserving the horizontal position for repeated movements.
    pub fn select_down(&mut self) {
        self.set_selection(self.selection.next_line(&self.layout, true));
    }

    /// Move the selection focus point to the next cluster left in visual order.
    pub fn select_left(&mut self) {
        self.set_selection(self.selection.previous_visual(&self.layout, true));
    }

    /// Move the selection focus point to the next cluster right in visual order.
    pub fn select_right(&mut self) {
        self.set_selection(self.selection.next_visual(&self.layout, true));
    }

    /// Move the selection focus point to the next word boundary left.
    pub fn select_word_left(&mut self) {
        self.set_selection(self.selection.previous_visual_word(&self.layout, true));
    }

    /// Move the selection focus point to the next word boundary right.
    pub fn select_word_right(&mut self) {
        self.set_selection(self.selection.next_visual_word(&self.layout, true));
    }

    /// Select the word at the point.
    pub fn select_word_at_point(&mut self, x: f32, y: f32) {
        self.set_selection(Selection::word_from_point(&self.layout, x, y));
    }

    /// Select the physical line at the point.
    ///
    /// Note that this metehod determines line breaks for any reason, including due to word wrapping.
    /// To select the text between explicit newlines, use [`select_hard_line_at_point`](Self::select_hard_line_at_point).
    /// In most text editing cases, this is the preferred behaviour.
    pub fn select_line_at_point(&mut self, x: f32, y: f32) {
        let line = Selection::line_from_point(&self.layout, x, y);
        self.set_selection(line);
    }

    /// Select the "logical" line at the point.
    ///
    /// The logical line is defined by line break characters, such as `\n`, rather than due to soft-wrapping.
    pub fn select_hard_line_at_point(&mut self, x: f32, y: f32) {
        let hard_line = Selection::hard_line_from_point(&self.layout, x, y);
        self.set_selection(hard_line);
    }

    /// Move the selection focus point to the cluster boundary closest to point.
    ///
    /// If the initial selection was created from a word or line, then the new
    /// selection will be extended at the same granularity.
    pub fn extend_selection_to_point(&mut self, point: Point) {
        // FIXME: This is usually the wrong way to handle selection extension for mouse moves, but not a regression.
        self.set_selection(self.selection.extend_to_point(
            &self.layout,
            point.x as f32,
            point.y as f32,
        ));
    }

    /// Move the selection focus point to the cluster boundary closest to point.
    pub fn shift_click_extension(&mut self, x: f32, y: f32) {
        self.set_selection(self.selection.shift_click_extension(&self.layout, x, y));
    }

    /// Move the selection focus point to a byte index.
    ///
    /// No-op if index is not a char boundary.
    pub fn extend_selection_to_byte(&mut self, index: usize) {
        if self.buffer.is_char_boundary(index) {
            self.set_selection(self.selection.extend(self.cursor_at(index)));
        }
    }

    /// Select a range of byte indices.
    ///
    /// No-op if either index is not a char boundary.
    pub fn select_byte_range(&mut self, start: usize, end: usize) {
        if self.buffer.is_char_boundary(start) && self.buffer.is_char_boundary(end) {
            self.set_selection(Selection::new(self.cursor_at(start), self.cursor_at(end)));
        }
    }

    /// If the current selection is not collapsed, returns the text content of
    /// that selection.
    pub fn selected_text(&self) -> Option<&str> {
        if !self.selection.is_collapsed() {
            self.buffer.get(self.selection.text_range())
        } else {
            None
        }
    }

    /// Get rectangles, and their corresponding line indices, representing the selected portions of
    /// text.
    pub fn selection_geometry(&self) -> Vec<(BoundingBox, usize)> {
        // We do not check `self.show_cursor` here, as the IME handling code collapses the
        // selection to a caret in that case.
        self.selection.geometry(&self.layout)
    }

    /// Invoke a callback with each rectangle representing the selected portions of text, and the
    /// indices of the lines to which they belong.
    pub fn selection_geometry_with(&self, f: impl FnMut(BoundingBox, usize)) {
        // We do not check `self.show_cursor` here, as the IME handling code collapses the
        // selection to a caret in that case.
        self.selection.geometry_with(&self.layout, f);
    }

    /// Get a rectangle representing the current caret cursor position.
    pub fn cursor_geometry(&self, size: f32) -> BoundingBox {
        self.selection.focus().geometry(&self.layout, size)
    }

    pub fn handle_pressed_key(&mut self, key: Key, modifiers: Modifiers) -> bool {
        let state = modifiers.state();

        let action_mod = state.control_key();
        let shift = state.shift_key();

        match key {
            Key::Character(c) if action_mod && matches!(c.as_str(), "c" | "x" | "v") => {
                use clipboard_rs::{Clipboard, ClipboardContext};
                match c.to_lowercase().as_str() {
                    "c" => {
                        if let Some(text) = self.selected_text() {
                            let clipboard = ClipboardContext::new().unwrap();
                            clipboard.set_text(text.to_owned()).ok();
                        }
                    }
                    "x" => {
                        if let Some(text) = self.selected_text() {
                            let clipboard = ClipboardContext::new().unwrap();
                            clipboard.set_text(text.to_owned()).ok();
                            self.delete_selection();
                        }
                    }
                    "v" => {
                        let clipboard = ClipboardContext::new().unwrap();
                        let text = clipboard.get_text().unwrap_or_default();
                        self.insert_or_replace_selection(&text);
                    }
                    _ => (),
                }
                return true;
            }
            Key::Character(c) if action_mod && matches!(c.to_lowercase().as_str(), "a") => {
                if shift {
                    self.collapse_selection();
                } else {
                    self.select_all();
                }
                return true;
            }
            Key::Named(NamedKey::ArrowLeft) => {
                if action_mod {
                    if shift {
                        self.select_word_left();
                    } else {
                        self.move_word_left();
                    }
                } else if shift {
                    self.select_left();
                } else {
                    self.move_left();
                }
                return true;
            }
            Key::Named(NamedKey::ArrowRight) => {
                if action_mod {
                    if shift {
                        self.select_word_right();
                    } else {
                        self.move_word_right();
                    }
                } else if shift {
                    self.select_right();
                } else {
                    self.move_right();
                }
                return true;
            }
            Key::Named(NamedKey::ArrowUp) => {
                if shift {
                    self.select_up();
                } else {
                    self.move_up();
                }
                return true;
            }
            Key::Named(NamedKey::ArrowDown) => {
                if shift {
                    self.select_down();
                } else {
                    self.move_down();
                }
                return true;
            }
            Key::Named(NamedKey::Home) => {
                if action_mod {
                    if shift {
                        self.select_to_text_start();
                    } else {
                        self.move_to_text_start();
                    }
                } else if shift {
                    self.select_to_line_start();
                } else {
                    self.move_to_line_start();
                }
                return true;
            }
            Key::Named(NamedKey::End) => {
                if action_mod {
                    if shift {
                        self.select_to_text_end();
                    } else {
                        self.move_to_text_end();
                    }
                } else if shift {
                    self.select_to_line_end();
                } else {
                    self.move_to_line_end();
                }
                return true;
            }
            Key::Named(NamedKey::Delete) => {
                if action_mod {
                    self.delete_word();
                } else {
                    self.delete();
                }
                return true;
            }
            Key::Named(NamedKey::Backspace) => {
                if action_mod {
                    self.backdelete_word();
                } else {
                    self.backdelete();
                }
                return true;
            }
            Key::Named(NamedKey::Enter) => {
                self.insert_or_replace_selection("\n");
                return true;
            }
            Key::Named(NamedKey::Space) => {
                self.insert_or_replace_selection(" ");
                return true;
            }
            Key::Named(NamedKey::Escape) => {
                self.collapse_selection();
                return true;
            }
            Key::Character(s) => {
                self.insert_or_replace_selection(&s.to_string());
                return true;
            }
            _ => {
                return false;
            }
        }
    }
}

impl Element for Editor {
    fn update(&mut self, cx: &mut UpdateContext) {
        if !cx.is_directly_focused() {
            return;
        }

        let mut request_redraw = false;
        cx.with_state(|state: &mut EditorState| {
            let modifiers = cx.modifiers();
            let key_events: Vec<_> = cx.key_events().into();
            for key_event in key_events {
                if key_event.state == ElementState::Released {
                    continue;
                }

                request_redraw |= state.handle_pressed_key(key_event.logical_key, modifiers);
            }
        });

        if request_redraw {
            cx.request_redraw();
        }
    }

    fn layout(&mut self, min: Size, max: Size, cx: &mut LayoutContext) -> Size {
        cx.push_default_text_style(StyleProperty::FontSize(self.size));
        cx.push_default_text_style(StyleProperty::Brush(self.text_stroke.clone()));

        cx.with_state(|state: &mut EditorState, cx| {
            let max_width = max.width;
            if state.layout_dirty || max_width != state.previous_max_width {
                if max_width.is_finite() {
                    state.layout = cx.layout_within(&state.buffer, max_width as f32);
                } else {
                    state.layout = cx.layout(&state.buffer);
                }

                state.selection = state.selection.refresh(&state.layout);
                state.layout_dirty = false;
                state.previous_max_width = max_width;
            }

            let size = Size::new(
                state.layout.full_width() as f64,
                state.layout.height() as f64,
            );
            let size = size.clamp(min, max);
            size
        })
    }

    fn draw(&self, cx: &mut DrawContext) {
        let region = cx.region();
        if cx.is_directly_focused() {
            cx.mouse_region(region)
                .on_down(|cx| {
                    cx.with_state(|state: &mut EditorState, cx| {
                        state.move_to_point(cx.mouse_position().unwrap());
                        cx.request_redraw();
                    });
                })
                .on_drag(|cx| {
                    cx.with_state(|state: &mut EditorState, cx| {
                        state.extend_selection_to_point(cx.mouse_position().unwrap());
                        cx.request_redraw();
                    });
                });
        } else {
            cx.mouse_region(region).on_click(move |cx| {
                cx.focus();
            });
        }

        cx.with_state(|state: &mut EditorState, cx| {
            if cx.is_directly_focused() {
                state.selection_geometry_with(|rect, _| {
                    cx.set_fill_brush(self.selection_fill.clone());
                    cx.fill(&Rect::new(rect.x0, rect.y0, rect.x1, rect.y1));
                });

                let cursor = state.cursor_geometry(1.0);
                cx.set_fill_brush(self.cursor_stroke.clone());
                cx.fill(&Rect::new(cursor.x0, cursor.y0, cursor.x1, cursor.y1));
            }

            cx.set_stroke_brush(self.text_stroke.clone());
            let top_left = region.origin();
            cx.draw_layout_at(&state.layout, top_left);
        });
    }
}
