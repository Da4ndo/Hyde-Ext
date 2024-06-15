use inquire::ui::{Attributes, Color, RenderConfig, StyleSheet, Styled};


pub fn get_render_config() -> RenderConfig<'static> {
    let mut render_config = RenderConfig::default();
    render_config.prompt = StyleSheet::empty().with_fg(Color::DarkYellow);
    render_config.prompt_prefix = Styled::new("?").with_fg(Color::DarkYellow);
    render_config.answered_prompt_prefix = Styled::new(">").with_fg(Color::DarkYellow);
    render_config.highlighted_option_prefix = Styled::new(">").with_fg(Color::DarkYellow);

    render_config.selected_checkbox = Styled::new("[✔]").with_fg(Color::DarkGreen);
    render_config.unselected_checkbox = Styled::new("[ ]");

    render_config.error_message = render_config
        .error_message
        .with_prefix(Styled::new("❌").with_fg(Color::DarkRed));

    render_config.answer = StyleSheet::new()
        .with_attr(Attributes::ITALIC)
        .with_fg(Color::LightYellow);

    render_config.help_message = StyleSheet::new().with_fg(Color::DarkYellow);
    render_config.selected_option = None;
    render_config
}

