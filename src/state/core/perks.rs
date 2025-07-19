use std::collections::HashMap;

pub fn handle_perk_selection(
    perk_selection_keys: &mut HashMap<String, bool>,
    highlighted_perk: &mut Option<usize>,
    selected_perk: &mut Option<usize>,
    perk_eligibility: &mut bool,
    in_perk_selection: &mut bool,
) -> bool {
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
        if let Some(perk) = *highlighted_perk {
            *selected_perk = Some(perk);
            *perk_eligibility = false;
            *in_perk_selection = false;
            *highlighted_perk = None;
            perk_selection_keys.clear();
            return true;
        }
    }

    // Handle Escape key for default perk selection
    if perk_selection_keys.contains_key("Escape") {
        *selected_perk = Some(1);
        *perk_eligibility = false;
        *in_perk_selection = false;
        *highlighted_perk = None;
        perk_selection_keys.clear();
        return true;
    }

    false
}

pub fn apply_perk_effect(perk: usize, move_interval: &mut f32, food_score_value: &mut u32) {
    match perk {
        1 => {
            // Speed boost: reduce move interval by 20%
            *move_interval *= 0.8;
        }
        2 => {
            // Double score: increase food score value by 2x
            *food_score_value *= 2;
        }
        _ => {}
    }
}

pub fn check_perk_eligibility(
    score: u32,
    granted_perks: &mut Vec<u32>,
) -> Option<u32> {
    // Define specific score thresholds
    let thresholds = [1000, 3000, 6000, 10000];

    for &threshold in &thresholds {
        if score >= threshold && !granted_perks.contains(&threshold) {
            granted_perks.push(threshold);
            return Some(threshold);
        }
    }

    None
}