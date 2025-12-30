use parley::{
    FontContext, Layout, LayoutContext, RangedBuilder, fontique::Collection, style::StyleProperty,
};
use vello::peniko::Brush;

pub struct Shaper {
    pub(crate) font_context: FontContext,
    pub(crate) layout_context: LayoutContext<Brush>,
}

impl Shaper {
    pub fn new() -> Self {
        Self {
            font_context: FontContext::default(),
            layout_context: LayoutContext::new(),
        }
    }

    pub fn layout(
        &mut self,
        text: &str,
        default_styles: &Vec<StyleProperty<'static, Brush>>,
    ) -> Layout<Brush> {
        let builder = self.layout_builder(text, default_styles);
        let mut layout = builder.build(text);
        layout.break_all_lines(None);
        layout
    }

    pub fn layout_within(
        &mut self,
        text: &str,
        max_advance: f32,
        default_styles: &Vec<StyleProperty<'static, Brush>>,
    ) -> Layout<Brush> {
        let builder = self.layout_builder(text, default_styles);
        let mut layout = builder.build(text);
        layout.break_all_lines(Some(max_advance));
        layout
    }

    pub fn layout_with<'a>(
        &'a mut self,
        text: &'a str,
        default_styles: &Vec<StyleProperty<'static, Brush>>,
        build: impl FnOnce(&mut RangedBuilder<'a, Brush>),
    ) -> Layout<Brush> {
        let mut builder = self.layout_builder(text, default_styles);

        build(&mut builder);

        let mut layout = builder.build(text);

        layout.break_all_lines(None);

        layout
    }

    pub fn layout_within_with<'a>(
        &'a mut self,
        text: &'a str,
        max_advance: f32,
        default_styles: &Vec<StyleProperty<'static, Brush>>,
        build: impl FnOnce(&mut RangedBuilder<'a, Brush>),
    ) -> Layout<Brush> {
        let mut builder = self.layout_builder(text, default_styles);

        build(&mut builder);

        let mut layout = builder.build(text);

        layout.break_all_lines(Some(max_advance));

        layout
    }

    pub fn layout_builder<'a>(
        &'a mut self,
        text: &'a str,
        default_styles: &Vec<StyleProperty<'static, Brush>>,
    ) -> RangedBuilder<'a, Brush> {
        let mut builder =
            // TODO: Dig through if this display scale is doing something important we need to
            // replicate
            self.layout_context
                .ranged_builder(&mut self.font_context, text, 1., true);
        for style in default_styles {
            builder.push_default(style.clone());
        }

        builder
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
