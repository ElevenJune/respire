//use color_eyre::owo_colors::OwoColorize;
use crate::App;
use ratatui::{
    buffer::Buffer,
    layout::{self, Constraint, Layout, Rect},
    style::{
        palette::tailwind::{AMBER, TEAL},
        Color, Modifier, Style, Stylize,
    },
    symbols::{self, Marker},
    text::Line,
    widgets::{
        Block, Borders, Gauge, HighlightSpacing, List, ListItem, Paragraph, StatefulWidget, Tabs,
        Widget, Wrap,
        canvas::{Canvas, Circle, Map, MapResolution, Points, Rectangle, Line as DrawLine},
    },
};
use std::{fmt::format, sync::Arc};
//use cli_log::*;

const LIGHT_COLOR: Color = TEAL.c100;
const FOCUS_COLOR: Color = AMBER.c300;
const _PAUSED_COLOR: Color = AMBER.c500;
const FOCUS_UNSELECTED_COLOR: Color = TEAL.c400;
const NORMAL_ROW_BG: Color = TEAL.c900;
const ALT_ROW_BG_COLOR: Color = TEAL.c800;
const YELLOW: Color = AMBER.c100;

const HEADER_STYLE: Style = Style::new()
    .fg(LIGHT_COLOR)
    .bg(ALT_ROW_BG_COLOR)
    .add_modifier(Modifier::BOLD);
const BORDER_STYLE_NONE: symbols::border::Set = symbols::border::EMPTY;
const BORDER_STYLE_SELECTED: symbols::border::Set = symbols::border::PROPORTIONAL_TALL;
const SELECTED_STYLE: Style = Style::new().bg(TEAL.c600).fg(FOCUS_COLOR);
const SELECTED_TAB_STYLE: Style = Style::new().bg(ALT_ROW_BG_COLOR).fg(FOCUS_COLOR);
const NOT_SELECTED_TAB_STYLE: Style = Style::new().bg(ALT_ROW_BG_COLOR).fg(TEAL.c600);
const GAUGE_STYLE: Style = Style::new().fg(LIGHT_COLOR).bg(ALT_ROW_BG_COLOR);

impl App {
    /// Renders header
    fn render_header(&self, area: Rect, buf: &mut Buffer) {
        let bg = TEAL.c500;
        let text = format!(
            "SerenIT\n{}\n{}",
            self.get_duration(),
            self.get_tick()
        );
        Arc::new(
            Paragraph::new(text)
                .bold()
                .centered()
                .bg(bg)
                .fg(YELLOW)
                .render(area, buf),
        );
    }

    /// Renders footer
    fn render_footer(&self, area: Rect, buf: &mut Buffer) {
        let text = 
            " Tab : switch between sound/scenes, 's' : save, 'q' : quit, 'm' : switch to mixer\n \
            ←→ : select category, ctrl & ←→ : adjust the master volume\n \
            Enter : add/remove the selected sound, Space : pause/play, 'n' : create scene";

        Paragraph::new(text)
            .left_aligned()
            .bg(FOCUS_UNSELECTED_COLOR)
            .fg(YELLOW)
            .bold()
            .render(area, buf);
    }


    //Renders the mixer (right panel)
    fn render_mixer(&self, area: Rect, buf: &mut Buffer) {
        Canvas::default()
    .block(Block::new())
    .x_bounds([-180.0, 180.0])
    .y_bounds([-90.0, 90.0])
    //.marker(Marker::Dot)
    .background_color(Color::Black)
    .paint(|ctx| {
        ctx.draw(&Circle {
            x: 0.0,
            y: 0.0,
            radius: self.get_radius(),
            color: TEAL.c400,
        });
    }).render(area,buf);
    }
}

//Renders whole app
impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let footer_length = if true { 3 } else { 2 };
        let [header_area, main_area, footer_area] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Length(footer_length),
        ])
        .areas(area);

        self.render_header(header_area, buf);
        self.render_footer(footer_area, buf);
        self.render_mixer(main_area, buf);
    }
}

pub const fn _alternate_colors(i: usize) -> Color {
    if i % 2 == 0 {
        NORMAL_ROW_BG
    } else {
        ALT_ROW_BG_COLOR
    }
}
