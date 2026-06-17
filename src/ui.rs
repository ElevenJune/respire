//use color_eyre::owo_colors::OwoColorize;
use crate::{App, breath_cycle::CycleState};
//use crate::{breath_cycle::{BreathCycle, CycleState}};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{
        Color, Modifier, Style, Stylize, palette::tailwind::{AMBER, TEAL}
    },
    symbols,
    text::{Line, Span, Text},
    widgets::{
        Block, Paragraph, Widget, canvas::{Canvas, Circle}
    },
};
//use cli_log::*;

const LIGHT_COLOR: Color = TEAL.c100;
const FOCUS_COLOR: Color = AMBER.c300;
const PAUSED_COLOR: Color = AMBER.c500;
const FOCUS_UNSELECTED_COLOR: Color = TEAL.c400;
#[allow(dead_code)]
const NORMAL_ROW_BG: Color = TEAL.c900;
#[allow(dead_code)]
const ALT_ROW_BG_COLOR: Color = TEAL.c800;
const YELLOW: Color = AMBER.c100;

const _HEADER_STYLE: Style = Style::new()
    .fg(LIGHT_COLOR)
    .bg(ALT_ROW_BG_COLOR)
    .add_modifier(Modifier::BOLD);
const _BORDER_STYLE_NONE: symbols::border::Set = symbols::border::EMPTY;
const _BORDER_STYLE_SELECTED: symbols::border::Set = symbols::border::PROPORTIONAL_TALL;
const _SELECTED_STYLE: Style = Style::new().bg(TEAL.c600).fg(FOCUS_COLOR);
const _SELECTED_TAB_STYLE: Style = Style::new().bg(ALT_ROW_BG_COLOR).fg(FOCUS_COLOR);
const _NOT_SELECTED_TAB_STYLE: Style = Style::new().bg(ALT_ROW_BG_COLOR).fg(TEAL.c600);
const _GAUGE_STYLE: Style = Style::new().fg(LIGHT_COLOR).bg(ALT_ROW_BG_COLOR);

impl App {
    /// Renders header
    fn render_header(&self, area: Rect, buf: &mut Buffer) {
        let bg = if !self.is_edit_mode() {ALT_ROW_BG_COLOR} else {PAUSED_COLOR};
        let cycle_name = match self.get_current_cycle(){
            Some(c)=>c.name(),
            None=>"(No cycle selected)"
        };

        let second_line = if self.is_edit_mode() {
            format!("Editing {}...",cycle_name)
        } else {
            format!("{} {} {}","←",cycle_name,"→")
        };

        let lines = vec![Line::from("[[[  --  Respire  --  ]]]").bold(),
        Line::from(second_line).bg(bg),
        self.render_cycle_data()];
        
        let text = Text::from(lines);
        Paragraph::new(text)
            .centered()
            .bg(bg)
            .style(_HEADER_STYLE)
            .render(area, buf)
    }

    /// Renders footer
    fn render_footer(&self, area: Rect, buf: &mut Buffer) {
        let bg = if !self.is_edit_mode() {FOCUS_UNSELECTED_COLOR} else {PAUSED_COLOR};
        //Controls
        let normal_text = 
            " 's' : Toggle sound, 'q' : quit, Space : pause/play, 'e' : Edit cycle\n \
            ←→ : select breath cycle, up/down : adjust step duration";
        let edit_text = 
            " [EDIT MODE ENABLED] 'e' : exit edit mode, 'q' : quit\n \
             Tab : select step, up/down : adjust step duration (500ms)";
            let text = if self.is_edit_mode() {edit_text} else {normal_text};
        Paragraph::new(text)
            .left_aligned()
            .bg(bg)
            .fg(YELLOW)
            .render(area, buf);
    }

    //Renders duration of each state of the selected BreathCycle
    fn render_cycle_data(&self) -> Line {

        let Some(current_cycle) = self.get_current_cycle().copied() else {
            return Line::from("Pas de cycle sélectionné");
        };
        let selected_state = self.get_selected_cycle_state();
        let default_style = Style::default();
        let bold_style = default_style.bold().fg(FOCUS_COLOR);

        let states = [CycleState::Inhale,CycleState::Break1, CycleState::Exhale, CycleState::Break2];
        /*let spans = states
        .iter()
        .enumerate()
        .flat_map(|(index, state)|{
            let style = if *state == selected_state { bold_style } else { default_style };
            let main_span = Span::styled(
                format!("{}:{}",state.to_string(),current_cycle.get_state_duration(&state)),
                style);
            let iter = std::iter::once(main_span);
            let separator = if index < states.len() - 1 { Some(Span::raw(" - ")) } else { None };
            iter.chain(separator)
        }).collect::<Vec<Span>>();*/

        let mut spans = Vec::with_capacity(2 * states.len());
        
        for (index, state) in states.into_iter().enumerate() {
            let style = if state == selected_state { bold_style } else { default_style };
            let main_span = Span::styled(
                format!("{}:{}",state.to_string(),current_cycle.get_state_duration(&state)),
                style);
            spans.push(main_span);
            if index < states.len() - 1 {
                spans.push(Span::raw(" - "));
            }
        }

        Line::from(spans)
    }


    //Renders the mixer (right panel)
    fn render_main(&self, area: Rect, buf: &mut Buffer) {
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
            let duration = if self.current_cycle_duration() > self.get_duration() {
                (self.current_cycle_duration()-self.get_duration())/1000+1
            }else {0};
            ctx.print(-5.0,-10.0, format!("{}", duration).yellow());
        }
    }).render(area,buf);
    }
}

//Renders whole app
impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let footer_length = 2;
        let [header_area, main_area, footer_area] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Length(footer_length),
        ])
        .areas(area);

        self.render_header(header_area, buf);
        self.render_footer(footer_area, buf);
        self.render_main(main_area, buf);
    }
}

pub const fn _alternate_colors(i: usize) -> Color {
    if i % 2 == 0 {
        NORMAL_ROW_BG
    } else {
        ALT_ROW_BG_COLOR
    }
}
