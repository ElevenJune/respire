use crate::breath_cycle::{BreathCycle, CycleState};
use crate::sink_handle::SinkHandle;

pub struct BreathManager{
    cycles: Vec<BreathCycle>,
    current_cycle_index : Option<usize>,
    current_cycle_state : CycleState,
    current_duration : u16,

    sound_enabled : bool,
    sound_inhale : SinkHandle,
    sound_exhale : SinkHandle,
    sound_hold : SinkHandle
}

impl BreathManager{
    pub fn new() -> Self{
        let mut bm = BreathManager {
            cycles: vec!(),
            current_cycle_index:None,
            current_cycle_state : CycleState::Inhale,
            current_duration : 0,
            sound_enabled : false,
            sound_inhale : SinkHandle::new(),
            sound_exhale : SinkHandle::new(),
            sound_hold : SinkHandle::new(),
        };

        bm.populate();
        bm.set_sounds();

        if bm.cycles.len() > 0 {
            bm.current_cycle_index = Some(0);
        }

        bm
    }

    //Getters
    pub fn _cycles(&self) -> &Vec<BreathCycle> {&self.cycles}
    pub fn current_cycle_state(&self) -> CycleState {self.current_cycle_state}
    pub fn current_duration(&self) -> u16 {self.current_duration}
    pub fn current_cycle_index(&self) -> Option<usize> {self.current_cycle_index}

    pub fn current_cycle(&self) -> Option<&BreathCycle> {
        if let Some(index) = self.current_cycle_index {
            self.cycles.get(index)
        } else {
            None
        }
    }


    //Actions
    pub fn set_current_cycle_index(&mut self, index: usize){
        if index >= self.cycles.len() {self.current_cycle_index=None;}
        else {self.current_cycle_index=Some(index);}
        self.current_cycle_state=CycleState::Inhale;
        self.current_duration=0;

        self.sound_exhale.stop();
        self.sound_hold.stop();
        self.sound_inhale.stop();
        //if self.current_cycle().is_some() {self.sound_inhale.play_once()}
    }

    pub fn toggle_sound_enabled(&mut self){
        self.sound_enabled = ! self.sound_enabled;
    }

    pub fn update_cycle(&mut self, millis_spent: u16) {
        if let Some(current_cycle) = self.current_cycle().cloned() {
            let current_state_duration = current_cycle.get_state_duration(&self.current_cycle_state);

            //Start cycle behaviour
            if self.sound_enabled 
            && self.current_duration == 0
            && current_state_duration>=1000{
                match self.current_cycle_state {
                    CycleState::Inhale => self.sound_inhale.play_once(),
                    CycleState::Exhale => self.sound_exhale.play_once(),
                    _ => {self.sound_hold.play_once()}
                }
            }
            self.current_duration+=millis_spent;

            //End cycle behaviour
            if self.current_duration>current_state_duration {
                let mut next_state = self.current_cycle_state.next();
                while current_cycle.get_state_duration(&next_state) == 0
                && next_state != CycleState::None {
                    next_state.roll();
                }
                self.current_cycle_state=next_state;
                self.current_duration=0;
            }
        }
    }

    //Private
    fn populate(&mut self) {
        self.cycles.push(BreathCycle::new(4000, 0, 4000, 0));
        self.cycles.push(BreathCycle::new(1000, 1000, 1000, 1000));
        self.cycles.push(BreathCycle::new(6000, 0, 6000, 0));
        self.cycles.push(BreathCycle::new(4000, 4000, 4000, 4000));
    }

    fn set_sounds(&mut self){
        self.sound_inhale.set_source_as_filepath("./sounds/laurainhale.mp3");
        self.sound_inhale.set_volume(0.5);
        self.sound_exhale.set_source_as_filepath("./sounds/lauraexhale.mp3");
        self.sound_exhale.set_volume(0.5);
        self.sound_hold.set_source_as_filepath("./sounds/laurahold.mp3");
        self.sound_hold.set_volume(0.5);
    }

}