//use color_eyre::owo_colors::OwoColorize;
use crate::{App, breath_cycle::CycleState};
//use crate::{breath_cycle::{BreathCycle, CycleState}};
use ratatui::{
    buffer::Buffer,
    layout::{self, Constraint, Layout, Rect},
    style::{
        Color, Modifier, Style, Stylize, palette::tailwind::{AMBER, TEAL}
    },
    symbols::{self, Marker, line},
    text::{Line, Text, Span},
    widgets::{
        Block, Borders, Gauge, HighlightSpacing, List, ListItem, Paragraph, StatefulWidget, Tabs, Widget, Wrap, canvas::{Canvas, Circle, Line as DrawLine, Map, MapResolution, Points, Rectangle}
    },
};
use std::{fmt::format, io::empty, sync::Arc};
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
            "Respire\n{}\n{}",
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

        let [cycle_data, footer] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(area.height-1),
        ])
        .areas(area);

        //Cycle data
        let bg_style = if self.get_selected_cycle_state()!=CycleState::None {FOCUS_COLOR} else {FOCUS_UNSELECTED_COLOR};
        Paragraph::new(self.render_cycle_data())
            .centered()
            .bg(bg_style)
            .fg(LIGHT_COLOR)
            .render(cycle_data, buf);

        //Controls
        let text = 
            " Tab : switch between breath cycle, 's' : save, 'q' : quit\n \
            ←→ : select category, ctrl & ←→ : adjust the master volume\n \
            Enter : add/remove the selected sound, Space : pause/play, 'n' : create scene";
        Paragraph::new(text)
            .left_aligned()
            .bg(FOCUS_UNSELECTED_COLOR)
            .fg(YELLOW)
            .render(footer, buf);
    }

    //Renders duration of each state of the selected BreathCycle
    fn render_cycle_data(&self) -> Line {
        let current_cycle = self.get_current_cycle().cloned().unwrap();
        let selected_state = self.get_selected_cycle_state();
        let default_style = Style::default();
        let bold_style = default_style.bold().red();

        let states = [CycleState::Inhale,CycleState::Break1, CycleState::Exhale, CycleState::Break2];
        let spans = states
        .iter()
        .enumerate()
        .map(|(index, state)|{
            let style = if *state == selected_state { bold_style } else { default_style };
            let separator = if index!= states.len()-1 {" , "} else {""};
            Span::styled(
                format!("{}:{}{}",state.to_string(),current_cycle.get_state_duration(&state),separator),
                style)
        }).collect::<Vec<Span>>();

        Line::from(spans)
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
        if self.is_break() {
            ctx.print(-5.0,0.0, "Hold".yellow());
            ctx.print(-5.0,-10.0, format!("{}", (self.current_cycle_duration()-self.get_duration())/1000+1).yellow());
        }
    }).render(area,buf);
    }
}

//Renders whole app
impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let footer_length = if true { 4 } else { 2 };
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
