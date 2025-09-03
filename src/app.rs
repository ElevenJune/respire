use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    DefaultTerminal,
};

use std::{
    time::{Duration, Instant},
};

use crate::breath_cycle::{BreathCycle, CycleState};

//use cli_log::*;
use color_eyre::Result;

const MIN_RADIUS: f64 = 20.0;
const MAX_RADIUS: f64 = 60.0;
const RADIUS_DIFF:f64 = MAX_RADIUS-MIN_RADIUS;

pub struct App {
    exit: bool,
    //Circle
    radius: f64,
    cycle : BreathCycle,
    cycle_state : CycleState,
    current_duration : f64,
    tick_count: u64
}

impl App {
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        let tick_rate = Duration::from_millis(50);
        
        let mut last_tick = Instant::now();
        while !self.exit {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            if event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    self.handle_key(key);
                };
            }

            if last_tick.elapsed() >= tick_rate {
                self.current_duration+=MAX_RADIUS;
                self.on_tick();
                last_tick = Instant::now();
            }
        }
        Ok(())
    }

    pub fn new() -> Self {
        let app =App {
            exit: false,
            radius: 20.0,
            cycle: BreathCycle::new(5000.0,1000.0,5000.0,1000.0),
            cycle_state : CycleState::Inhale,
            current_duration : 0.0,
            tick_count:0
        };
        app
    }

    //----Getters

    pub fn get_radius(&self) -> f64 {self.radius}
    pub fn get_duration(&self) -> f64 {self.current_duration}
    pub fn get_tick(&self) -> u64 {self.tick_count}

    //----Event handling

    fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        let _ctrl_pressed = key.modifiers.contains(KeyModifiers::CONTROL);
        match key.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Char('k') | KeyCode::Up => self.increment_radius(false, 1.0),
            KeyCode::Char('G') | KeyCode::Down => self.increment_radius(true, 1.0),
            _ => {}
        }
    }

    fn on_tick(& mut self) {
        self.tick_count+=1;

        let current_state_duration = self.cycle.get_state_duration(&self.cycle_state);

        if !self.cycle_state.is_break(){
            let mut animation_progress = self.current_duration/current_state_duration;
            animation_progress = self.ease_in_out_squad(animation_progress);
            self.radius = self.radius_per_frame(animation_progress);
        }

        if self.current_duration>current_state_duration {
            self.cycle_state = self.cycle_state.next();
            self.current_duration=0.0;
        }
    }

    fn increment_radius(&mut self, down: bool, step : f64) {
        if down && self.radius>0.0 {
            self.radius-=step;
        } else {
            self.radius+=step;
        }
    }

    fn ease_in_out_squad(&self, x: f64) -> f64 {
        if x < 0.5 {
            2.0 * x * x
        } else {
            1.0 - (-2.0 * x + 2.0).powf(2.0) / 2.0
        }
    }

    //Progress is between 0 (beginning) and 1 (end)
    fn radius_per_frame(&self, progress:f64) -> f64 {
        match self.cycle_state {
            CycleState::Inhale => MIN_RADIUS + progress * RADIUS_DIFF,
            CycleState::Exhale => MAX_RADIUS - progress * RADIUS_DIFF,
            CycleState::Break1 => MAX_RADIUS,
            CycleState::Break2 => MIN_RADIUS,
            CycleState::None => 0.0
            
        }
    }
}