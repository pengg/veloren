use crate::{
    comp::{beam, CharacterState, EnergySource, Ori, Pos, StateUpdate},
    event::ServerEvent,
    states::utils::*,
    sys::character_behavior::*,
};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use vek::Vec3;

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Data {
    /// Whether the attack can currently deal damage
    pub exhausted: bool,
    /// Used for particle stuffs
    pub particle_ori: Option<Vec3<f32>>,
    /// How long until state should deal damage or heal
    pub buildup_duration: Duration,
    /// How long until weapon can deal another tick of damage
    pub cooldown_duration: Duration,
    /// How long the state has until exiting
    pub recover_duration: Duration,
    /// How long each beam segment persists for
    pub beam_duration: Duration,
    /// Base healing per second
    pub base_hps: u32,
    /// Base damage per second
    pub base_dps: u32,
    /// Ticks of damage/healing per second
    pub tick_rate: f32,
    /// Max range
    pub range: f32,
    /// Max angle (45.0 will give you a 90.0 angle window)
    pub max_angle: f32,
    /// Lifesteal efficiency (0 gives 0% conversion of damage to health, 1 gives
    /// 100% conversion of damage to health)
    pub lifesteal_eff: f32,
    /// Energy regened per second
    pub energy_regen: u32,
}

impl CharacterBehavior for Data {
    fn behavior(&self, data: &JoinData) -> StateUpdate {
        let mut update = StateUpdate::from(data);

        handle_move(data, &mut update, 0.4);
        handle_jump(data, &mut update);

        if self.buildup_duration != Duration::default() {
            // Build up
            update.character = CharacterState::BasicBeam(Data {
                exhausted: self.exhausted,
                particle_ori: Some(*data.inputs.look_dir),
                buildup_duration: self
                    .buildup_duration
                    .checked_sub(Duration::from_secs_f32(data.dt.0))
                    .unwrap_or_default(),
                cooldown_duration: self.cooldown_duration,
                recover_duration: self.recover_duration,
                beam_duration: self.beam_duration,
                base_hps: self.base_hps,
                base_dps: self.base_dps,
                tick_rate: self.tick_rate,
                range: self.range,
                max_angle: self.max_angle,
                lifesteal_eff: self.lifesteal_eff,
                energy_regen: self.energy_regen,
            });
        } else if data.inputs.primary.is_pressed() && !self.exhausted {
            let damage = (self.base_dps as f32 / self.tick_rate) as u32;
            let heal = (self.base_hps as f32 / self.tick_rate) as u32;
            let speed = self.range / self.beam_duration.as_secs_f32();
            let properties = beam::Properties {
                angle: self.max_angle.to_radians(),
                speed,
                damage,
                heal,
                lifesteal_eff: self.lifesteal_eff,
                duration: self.beam_duration,
                owner: Some(*data.uid),
            };
            let pos = Pos(data.pos.0 + Vec3::new(0.0, 0.0, 1.0));
            // Create beam segment
            update.server_events.push_front(ServerEvent::Beam {
                properties,
                pos,
                ori: Ori(data.inputs.look_dir),
            });

            update.character = CharacterState::BasicBeam(Data {
                exhausted: true,
                particle_ori: Some(*data.inputs.look_dir),
                buildup_duration: self.buildup_duration,
                recover_duration: self.recover_duration,
                cooldown_duration: Duration::from_secs_f32(1.0 / self.tick_rate),
                beam_duration: self.beam_duration,
                base_hps: self.base_hps,
                base_dps: self.base_dps,
                tick_rate: self.tick_rate,
                range: self.range,
                max_angle: self.max_angle,
                lifesteal_eff: self.lifesteal_eff,
                energy_regen: self.energy_regen,
            });
        } else if data.inputs.primary.is_pressed() && self.cooldown_duration != Duration::default()
        {
            // Cooldown until next tick of damage
            update.character = CharacterState::BasicBeam(Data {
                exhausted: self.exhausted,
                particle_ori: Some(*data.inputs.look_dir),
                buildup_duration: self.buildup_duration,
                cooldown_duration: self
                    .cooldown_duration
                    .checked_sub(Duration::from_secs_f32(data.dt.0))
                    .unwrap_or_default(),
                recover_duration: self.recover_duration,
                beam_duration: self.beam_duration,
                base_hps: self.base_hps,
                base_dps: self.base_dps,
                tick_rate: self.tick_rate,
                range: self.range,
                max_angle: self.max_angle,
                lifesteal_eff: self.lifesteal_eff,
                energy_regen: self.energy_regen,
            });
        } else if data.inputs.primary.is_pressed() {
            update.character = CharacterState::BasicBeam(Data {
                exhausted: false,
                particle_ori: Some(*data.inputs.look_dir),
                buildup_duration: self.buildup_duration,
                recover_duration: self.recover_duration,
                cooldown_duration: self.cooldown_duration,
                beam_duration: self.beam_duration,
                base_hps: self.base_hps,
                base_dps: self.base_dps,
                tick_rate: self.tick_rate,
                range: self.range,
                max_angle: self.max_angle,
                lifesteal_eff: self.lifesteal_eff,
                energy_regen: self.energy_regen,
            });
        } else if self.recover_duration != Duration::default() {
            // Recovery
            update.character = CharacterState::BasicBeam(Data {
                exhausted: self.exhausted,
                particle_ori: Some(*data.inputs.look_dir),
                buildup_duration: self.buildup_duration,
                cooldown_duration: self.cooldown_duration,
                recover_duration: self
                    .recover_duration
                    .checked_sub(Duration::from_secs_f32(data.dt.0))
                    .unwrap_or_default(),
                beam_duration: self.beam_duration,
                base_hps: self.base_hps,
                base_dps: self.base_dps,
                tick_rate: self.tick_rate,
                range: self.range,
                max_angle: self.max_angle,
                lifesteal_eff: self.lifesteal_eff,
                energy_regen: self.energy_regen,
            });
        } else {
            // Done
            update.character = CharacterState::Wielding;
        }

        update
    }
}
