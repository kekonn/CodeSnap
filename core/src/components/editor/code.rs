use crate::{
    components::interface::{
        component::{Component, ComponentContext, RenderParams},
        render_error,
        style::{ComponentStyle, RawComponentStyle, Size, Style},
    },
    utils::code::{calc_wh_with_min_width, prepare_code, CHAR_WIDTH},
    utils::highlight::Highlight,
    utils::text::{create_file_system_from_binary, FontRenderer},
};

const CASKAYDIA_COVE_NERD_FONT: &[u8] =
    include_bytes!("../../../assets/fonts/CaskaydiaCoveNerdFont-Regular.ttf");

pub struct Code {
    children: Vec<Box<dyn Component>>,
    line_height: f32,
    font_size: f32,
    value: String,
}

impl Component for Code {
    fn children(&self) -> &Vec<Box<dyn Component>> {
        &self.children
    }

    fn style(&self) -> RawComponentStyle {
        let (w, h) = calc_wh_with_min_width(&self.value, CHAR_WIDTH, self.line_height);

        Style::default().size(Size::Num(w), Size::Num(h))
    }

    fn draw_self(
        &self,
        pixmap: &mut tiny_skia::Pixmap,
        context: &ComponentContext,
        render_params: &RenderParams,
        style: &ComponentStyle,
        _parent_style: &ComponentStyle,
    ) -> render_error::Result<()> {
        let params = &context.take_snapshot_params;
        let highlight = Highlight::new(self.value.clone(), params.code.font_family.clone());
        let (mut highlight_lines, syntax_set) = context.theme_provider.highlight();
        let highlight_result = highlight.parse(&mut highlight_lines, syntax_set)?;

        FontRenderer::new(
            self.font_size,
            self.line_height,
            context.scale_factor,
            create_file_system_from_binary(
                CASKAYDIA_COVE_NERD_FONT,
                &context.take_snapshot_params.fonts_folder,
            ),
        )
        .draw_text(
            render_params.x,
            render_params.y,
            style.width,
            style.height,
            highlight_result.clone(),
            pixmap,
        );

        Ok(())
    }
}

impl Code {
    pub fn new(value: &str, line_height: f32, font_size: f32) -> Code {
        Code {
            value: prepare_code(&value),
            line_height,
            font_size,
            children: vec![],
        }
    }
}
