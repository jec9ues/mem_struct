use serde::{Deserialize, Serialize};
use phf::phf_map;

#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum ModelNameMap {
    #[default]
    mp_rr_olympus_mu2,
    mp_rr_canyonlands_staging_mu1,

}

pub static MODEL_NAME_MAP: phf::Map<&'static str, ModelNameMap> = phf_map! {
    "maps/mp_rr_canyonlands_staging_mu1.bsp" => ModelNameMap::mp_rr_canyonlands_staging_mu1,
    "maps/mp_rr_olympus_mu2.bsp" => ModelNameMap::mp_rr_olympus_mu2,
};
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum ModelNamePlayer {
    #[default]
    bangalore,
    fuse,
    ash,
    madmaggie,
    ballistic,

    pathfinder,
    wraith,
    stim,
    revenant,
    nova,
    valkyrie,

    bloodhound,
    crypto,
    seer,
    vantage,

    gibraltar,
    lifeline,
    holo,
    loba,
    newcastle,
    conduit,

    caustic,
    wattson,
    rampart,
    catalyst,
}

pub static MODEL_NAME_PLAYER: phf::Map<&'static str, ModelNamePlayer> = phf_map! {
    "mdl/humans/class/medium/pilot_medium_bangalore.rmdl" => ModelNamePlayer::bangalore,
    "mdl/Humans/class/medium/pilot_medium_fuse.rmdl" => ModelNamePlayer::fuse,
    "mdl/techart/mshop/characters/legends/ash/ash_base_w.rmdl" => ModelNamePlayer::ash,
    "mdl/techart/mshop/characters/legends/madmaggie/madmaggie_base_w.rmdl" => ModelNamePlayer::madmaggie,
    "mdl/techart/mshop/characters/legends/ballistic/ballistic_base_w.rmdl" => ModelNamePlayer::ballistic,

    "mdl/humans/class/heavy/pilot_heavy_pathfinder.rmdl" => ModelNamePlayer::pathfinder,
    "mdl/humans/class/light/pilot_light_wraith.rmdl" => ModelNamePlayer::wraith,
    "mdl/humans/class/medium/pilot_medium_stim.rmdl" => ModelNamePlayer::stim,
    "mdl/humans/class/heavy/pilot_heavy_revenant_reborn.rmdl" => ModelNamePlayer::revenant,
    "mdl/humans/class/medium/pilot_medium_nova_01.rmdl" => ModelNamePlayer::nova,
    "mdl/humans/class/medium/pilot_medium_valkyrie.rmdl" => ModelNamePlayer::valkyrie,

    "mdl/humans/class/medium/pilot_medium_bloodhound_legendary_02.rmdl" => ModelNamePlayer::bloodhound,
    "mdl/humans/class/medium/pilot_medium_crypto.rmdl" => ModelNamePlayer::crypto,
    "mdl/techart/mshop/characters/legends/seer/seer_base_w.rmdl" => ModelNamePlayer::seer,
    "mdl/techart/mshop/characters/legends/vantage/vantage_base_w.rmdl" => ModelNamePlayer::vantage,

    "mdl/humans/class/heavy/pilot_heavy_gibraltar.rmdl" => ModelNamePlayer::gibraltar,
    "mdl/humans/class/light/pilot_light_lifeline.rmdl" => ModelNamePlayer::lifeline,
    "mdl/humans/class/medium/pilot_medium_holo.rmdl" => ModelNamePlayer::holo,
    "mdl/humans/class/medium/pilot_medium_loba.rmdl" => ModelNamePlayer::loba,
    "mdl/techart/mshop/characters/legends/newcastle/newcastle_base_w.rmdl" => ModelNamePlayer::newcastle,
    "mdl/techart/mshop/characters/legends/conduit/conduit_base_w.rmdl" => ModelNamePlayer::conduit,

    "mdl/humans/class/heavy/pilot_heavy_caustic.rmdl" => ModelNamePlayer::caustic,
    "mdl/humans/class/light/pilot_light_wattson_legendary_01.rmdl" => ModelNamePlayer::wattson,
    "mdl/humans/class/medium/pilot_medium_rampart.rmdl" => ModelNamePlayer::rampart,
    "mdl/techart/mshop/characters/legends/catalyst/catalyst_base_w.rmdl" => ModelNamePlayer::catalyst,
};


