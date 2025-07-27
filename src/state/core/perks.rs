use std::collections::HashMap;
use crate::state::constants::state;

pub enum Perk {
    NeedForSpeed,
    HungryWorm,
    PerkThree,
    PerkFour,
    PerkFive,
    PerkSix,
    PerkSeven,
    PerkEight
}

pub fn get_perks_for_threshold(threshold: u32) -> (Perk, Perk) {
    match threshold {
        state::SCORE_PERK_THRESHOLD_LEVEL_1 => (Perk::NeedForSpeed, Perk::HungryWorm),
        state::SCORE_PERK_THRESHOLD_LEVEL_2 => (Perk::PerkThree, Perk::PerkFour),
        state::SCORE_PERK_THRESHOLD_LEVEL_3 => (Perk::PerkFive, Perk::PerkSix),
        state::SCORE_PERK_THRESHOLD_LEVEL_4 => (Perk::PerkSeven, Perk::PerkEight),
        _ => (Perk::NeedForSpeed, Perk::HungryWorm), // Default fallback
    }
}

pub fn handle_perk_selection(
    perk_selection_keys: &mut HashMap<String, bool>,
    highlighted_perk: &mut Option<usize>,
    selected_perk: &mut Option<Perk>,
    perk_eligibility: &mut bool,
    in_perk_selection: &mut bool,
    current_threshold: u32,
) -> bool {
    let (perk1, perk2) = get_perks_for_threshold(current_threshold);

    // Handle A and D keys for perk navigation
    if perk_selection_keys.contains_key("KeyA") {
        *highlighted_perk = Some(1);
        perk_selection_keys.remove("KeyA");
    }
    if perk_selection_keys.contains_key("KeyD") {
        *highlighted_perk = Some(2);
        perk_selection_keys.remove("KeyD");
    }

    // Handle Space key for perk selection
    if perk_selection_keys.contains_key("Space") {
        if let Some(perk_index) = *highlighted_perk {
            let chosen_perk = match perk_index {
                1 => perk1,
                2 => perk2,
                _ => perk1, // Default fallback
            };
            *selected_perk = Some(chosen_perk);
            *perk_eligibility = false;
            *in_perk_selection = false;
            *highlighted_perk = None;
            perk_selection_keys.clear();
            return true;
        }
    }

    // Handle Escape key for default perk selection
    if perk_selection_keys.contains_key("Escape") {
        *selected_perk = Some(perk1); // Choose first perk as default
        *perk_eligibility = false;
        *in_perk_selection = false;
        *highlighted_perk = None;
        perk_selection_keys.clear();
        return true;
    }

    false
}

pub fn apply_perk_effect(perk: &Perk, move_interval: &mut f32, food_score_value: &mut u32) {
    match perk {
        Perk::NeedForSpeed => {
            // Speed boost: reduce move interval by 25%
            *move_interval *= 0.75;
        }
        Perk::HungryWorm => {
            // Double score: increase food score value by 2x
            *food_score_value *= 2;
        }
        Perk::PerkThree => {
            // TODO: Implement PerkThree effect
        }
        Perk::PerkFour => {
            // TODO: Implement PerkFour effect
        }
        Perk::PerkFive => {
            // TODO: Implement PerkFive effect
        }
        Perk::PerkSix => {
            // TODO: Implement PerkSix effect
        }
        Perk::PerkSeven => {
            // TODO: Implement PerkSeven effect
        }
        Perk::PerkEight => {
            // TODO: Implement PerkEight effect
        }
    }
}

pub fn check_perk_eligibility(
    score: u32,
    granted_perks: &mut Vec<u32>,
) -> Option<u32> {
    // Define specific score thresholds
    let thresholds = [state::SCORE_PERK_THRESHOLD_LEVEL_1,
        state::SCORE_PERK_THRESHOLD_LEVEL_2,
        state::SCORE_PERK_THRESHOLD_LEVEL_3,
        state::SCORE_PERK_THRESHOLD_LEVEL_4];

    for &threshold in &thresholds {
        if score >= threshold && !granted_perks.contains(&threshold) {
            granted_perks.push(threshold);
            return Some(threshold);
        }
    }

    None
}