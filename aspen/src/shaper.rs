use parley::{
    fontique::Collection, style::StyleProperty, FontContext, Layout, LayoutContext, RangedBuilder,
};
use vello::peniko::Brush;

pub struct Shaper {
    font_context: FontContext,
    layout_context: LayoutContext<Brush>,
    default_styles: Vec<StyleProperty<'static, Brush>>,
}

impl Shaper {
    pub fn new() -> Self {
        Self {
            font_context: FontContext::default(),
            layout_context: LayoutContext::new(),
            default_styles: Vec::new(),
        }
    }

    pub fn layout_with<'a>(
        &'a mut self,
        text: &'a str,
        build: impl FnOnce(&mut RangedBuilder<'a, Brush>),
    ) -> Layout<Brush> {
        let mut builder = self.layout_builder(text);

        build(&mut builder);

        let mut layout = builder.build(text);

        layout.break_all_lines(None);

        layout
    }

    pub fn layout_within_with<'a>(
        &'a mut self,
        text: &'a str,
        max_advance: f32,
        build: impl FnOnce(&mut RangedBuilder<'a, Brush>),
    ) -> Layout<Brush> {
        let mut builder = self.layout_builder(text);

        build(&mut builder);

        let mut layout = builder.build(text);

        layout.break_all_lines(Some(max_advance));

        layout
    }

    pub fn layout(&mut self, text: &str) -> Layout<Brush> {
        let mut builder = self.layout_builder(text);
        let mut layout = builder.build(text);
        layout.break_all_lines(None);
        layout
    }

    pub fn layout_within(&mut self, text: &str, max_advance: f32) -> Layout<Brush> {
        let mut builder = self.layout_builder(text);
        let mut layout = builder.build(text);
        layout.break_all_lines(Some(max_advance));
        layout
    }

    pub fn layout_builder<'a>(&'a mut self, text: &'a str) -> RangedBuilder<'a, Brush> {
        let mut builder =
            // TODO: Dig through if this display scale is doing something important we need to
            // replicate
            self.layout_context
                .ranged_builder(&mut self.font_context, text, 1., true);
        for style in &self.default_styles {
            builder.push_default(style.clone());
        }

        builder
    }

    pub fn push_default(&mut self, style: StyleProperty<'static, Brush>) {
        self.default_styles.push(style);
    }

    pub fn clear_defaults(&mut self) {
        self.default_styles.clear();
    }

    pub fn font_collection(&mut self) -> &mut Collection {
        &mut self.font_context.collection
    }
}

impl Default for Shaper {
    fn default() -> Self {
        Self::new()
    }
}
