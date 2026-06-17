use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    DefaultTerminal,
};

use std::{
    time::{Duration, Instant},
};

use crate::{breath_cycle::{BreathCycle, CycleState}, breath_manager::BreathManager};

//use cli_log::*;
use color_eyre::Result;

const MIN_RADIUS: f64 = 20.0;
const MAX_RADIUS: f64 = 60.0;
const RADIUS_DIFF:f64 = MAX_RADIUS-MIN_RADIUS;

pub struct App {
    exit: bool,
    //Circle
    radius: f64,
    //Backend
    manager : BreathManager,
    //App data
    tick_count: u64,
    tick_rate: u16,
    selected_cycle_state:CycleState,
    paused:bool
}

impl App {
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        let tick_rate = Duration::from_millis(self.tick_rate as u64);
        
        let mut last_tick = Instant::now();
        while !self.exit {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            if event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    self.handle_key(key);
                };
            }

            if last_tick.elapsed() >= tick_rate && !self.paused {
                let elapsed = last_tick.elapsed().as_millis() as u16;
                self.on_tick();
                self.manager.update_cycle(elapsed);
                last_tick = Instant::now();
            }
        }
        Ok(())
    }

    pub fn new() -> Self {
        let app =App {
            exit: false,
            radius: 20.0,
            manager : BreathManager::new(),
            tick_count:0,
            tick_rate:20,
            selected_cycle_state:CycleState::None,
            paused:false
        };
        app
    }

    //----Getters

    pub fn get_radius(&self) -> f64 {self.radius}
    pub fn is_edit_mode(&self) -> bool {self.selected_cycle_state!=CycleState::None}
    pub fn get_duration(&self) -> u16 {self.manager.current_duration()}
    pub fn get_tick(&self) -> u64 {self.tick_count}
    pub fn is_break(&self) -> bool {self.manager.current_cycle_state().is_break()}
    pub fn get_selected_cycle_state(&self) -> CycleState {self.selected_cycle_state}
    pub fn current_cycle_duration(&self) -> u16 {
        let current_cycle = self.manager.current_cycle();
        if let Some(cycle) = current_cycle {
            let current_cycle_state = self.manager.current_cycle_state();
            cycle.get_state_duration(&current_cycle_state)
        } else {
            0
        }
    }
    pub fn get_current_cycle(&self) -> Option<&BreathCycle>{
        self.manager.current_cycle()
    }

    //----Event handling

    fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        let _ctrl_pressed = key.modifiers.contains(KeyModifiers::CONTROL);
        match key.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Char('a') | KeyCode::Up => self.increment_state_duration(500),
            KeyCode::Char('z') | KeyCode::Down => self.increment_state_duration(-500),
            KeyCode::Char('c') | KeyCode::Right => self.select_next(false),
            KeyCode::Char('x') | KeyCode::Left => self.select_next(true),
            KeyCode::Char('s') => self.manager.toggle_sound_enabled(),
            KeyCode::Tab => if self.is_edit_mode() {self.switch_selected_state(false)},
            KeyCode::Char('e') => self.switch_edit_mode(),
            _ => {}
        }
    }

    fn on_tick(& mut self) {
        self.tick_count+=1;
        
        let current_cycle = self.manager.current_cycle();

        if let Some(cycle) = current_cycle {
            let current_cycle_state = self.manager.current_cycle_state();
            let current_state_duration = cycle.get_state_duration(&current_cycle_state);
            let cd = self.manager.current_duration() as f64;
            let csd = current_state_duration as f64;
            let mut animation_progress:f64 = cd/csd;

            if !current_cycle_state.is_break(){
                animation_progress = self.ease_in_out_squad(animation_progress);
                self.radius = self.radius_per_frame(animation_progress);
            } else {
                animation_progress = self.hold_radius_per_frame(animation_progress);
                self.radius = self.radius_per_frame(animation_progress);
            }
        }
    }

    #[allow(dead_code)]
    fn increment_radius(&mut self, down: bool, step : f64) {
        if down && self.radius>0.0 {
            self.radius-=step;
        } else {
            self.radius+=step;
        }
    }

    fn switch_cycle(&mut self, up: bool){
        if self.is_edit_mode() {return;}
        match self.manager.current_cycle_index() {
            None => {
                self.manager.set_current_cycle_index(0);
            },
            Some(i) => {
                let new_index = if up {i+1} else {i.saturating_add_signed(-1)};
                self.manager.set_current_cycle_index(new_index);
            }
        }
    }

    fn select_next(&mut self, left: bool){
        if self.is_edit_mode(){
            self.switch_selected_state(left)
        } else {
            self.switch_cycle(!left)
        }
    }

    fn switch_edit_mode(&mut self){
        let was_editing = self.is_edit_mode();
        self.selected_cycle_state=if was_editing {CycleState::None} else {CycleState::Inhale};
        self.paused = !was_editing;
        self.manager.reset_cycle_state();
        self.on_tick();
    }

    fn switch_selected_state(&mut self, left: bool){
        if !left {
            self.selected_cycle_state.roll();
        }else{
            for _i in 0..3 {self.selected_cycle_state.roll();}
        }
    }

    fn increment_state_duration(&mut self, step:i16){
        self.manager.increment_current_cycle_state_duration(&self.selected_cycle_state, step);
    }

    fn ease_in_out_squad(&self, x: f64) -> f64 {
        if x < 0.5 {
            2.0 * x * x
        } else {
            1.0 - (-2.0 * x + 2.0).powf(2.0) / 2.0
        }
    }

    fn hold_radius_per_frame(&self, x:f64) -> f64 {
        /*if x>0.5 {
            -2.0*x+2.0
        } else {
            x*2.0
        }*/
        if x < 0.25 {
            4.0*x
        } else if x>=0.25 && x < 0.75 {
            -4.0*x+2.0
        } else {
            4.0*x-4.0
        }
    }

    //Progress is between 0 (beginning) and 1 (end)
    fn radius_per_frame(&self, progress:f64) -> f64 {
        match self.manager.current_cycle_state() {
            CycleState::Inhale => MIN_RADIUS + progress * RADIUS_DIFF,
            CycleState::Exhale => MAX_RADIUS - progress * RADIUS_DIFF,
            CycleState::Break1 => MAX_RADIUS + progress * 0.5,
            CycleState::Break2 => MIN_RADIUS - progress * 0.5,
            CycleState::None => 0.0
            
        }
    }
}