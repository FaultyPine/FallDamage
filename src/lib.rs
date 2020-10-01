#![feature(proc_macro_hygiene)]
#![allow(non_snake_case)]

use smash::app::{self, lua_bind::*};
use smash::lib::lua_const::*;
use smash::lua2cpp::L2CFighterCommon;

unsafe fn get_player_number(boma: &mut app::BattleObjectModuleAccessor) -> usize {
    app::lua_bind::WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize
}

static mut Y_SPEED: [f32;8] = [0.0;8];

fn once_per_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let entry_id = get_player_number(boma);

        Y_SPEED[entry_id] = KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);

        let prev_status_kind = StatusModule::prev_status_kind(boma, 0);
        let status_kind = StatusModule::status_kind(boma);

        if [*FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, *FIGHTER_STATUS_KIND_LANDING_DAMAGE_LIGHT, *FIGHTER_STATUS_KIND_DAMAGE_AIR, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL]
           .contains(&prev_status_kind) || 
           [*FIGHTER_STATUS_KIND_DOWN, *FIGHTER_STATUS_KIND_DAMAGE]
           .contains(&status_kind) {
                ColorBlendModule::cancel_main_color(boma, 0);
        }

    }
}

#[skyline::hook(replace = StatusModule::init_settings)]
pub unsafe fn init_settings_hook(boma: &mut app::BattleObjectModuleAccessor, situation_kind: i32, param_3: i32, param_4: i32, param_5: u64, param_6: bool, param_7: i32, param_8: i32, param_9: i32, param_10: i32){
    
    if smash::app::utility::get_category(boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        let status_kind = StatusModule::status_kind(boma);
        if [*FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, *FIGHTER_STATUS_KIND_DOWN, *FIGHTER_STATUS_KIND_LANDING_DAMAGE_LIGHT, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL]
           .contains(&status_kind) || (status_kind == *FIGHTER_STATUS_KIND_DAMAGE && StatusModule::prev_status_kind(boma, 0) == *FIGHTER_STATUS_KIND_DAMAGE_AIR) {
                DamageModule::add_damage(boma, Y_SPEED[get_player_number(boma)] * -2.0, 0);
                SoundModule::play_se(boma, smash::phx::Hash40::new("se_system_spiritspoint"), false, false, false, false, app::enSEType(0));
                let colorflashvec1 = smash::phx::Vector4f { /* Red */ x: 1.0, /* Green */ y: 0.0, /* Blue */ z: 0.0, /* Alpha? */ w: 0.0}; // setting this and the next vector's .w to 1 seems to cause a ghostly effect
                let colorflashvec2 = smash::phx::Vector4f { /* Red */ x: 1.0, /* Green */ y: 0.0, /* Blue */ z: 0.0, /* Alpha? */ w: 0.0};
                ColorBlendModule::set_main_color(boma, &colorflashvec1, &colorflashvec2, 0.7, 0.2, 25, true); //int here is opacity
        }
    }

    original!()(boma, situation_kind, param_3, param_4, param_5, param_6, param_7, param_8, param_9, param_10)
}

#[skyline::main(name = "FallDamage")]
pub fn main() {
    acmd::add_custom_hooks!(once_per_fighter_frame);
    skyline::install_hook!(init_settings_hook);
}
