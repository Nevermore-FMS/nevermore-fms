/// This function evaluates if the given target id string is contained within
/// the given scope string. Some examples:
///
/// Scope: fms.field.driverstations
/// Target ID: fms.field.driverstations.Red1
/// Returns: True
///
/// Scope: fms.field.driverstations.Red2
/// Target ID: fms.field.driverstations.Blue1
/// Returns: False
///
/// Scope: fms.field
/// Target ID: fms.field.driverstations.Blue3
/// Returns: True
///
/// Scope: fms.field
/// Target ID: fms.field.game_elements.collector
/// Returns: True
pub fn is_target_in_scope(scope: &str, target_id: &str) -> bool {
    let mut scope_secs = scope.split(".");
    let mut target_secs = target_id.split(".");

    loop {
        let next_scope_sec = scope_secs.next();
        let next_target_sec = target_secs.next();

        if next_scope_sec.is_none() && next_target_sec.is_none() {
            // Evaluation of both scope and target ended at same level.
            // They must be the same identifier
            return true;
        }

        if next_scope_sec.is_none() && next_target_sec.is_some() {
            // Evaluation of scope ended at a higher level than target.
            // Scope must contain target
            return true;
        }

        if next_scope_sec.is_some() && next_target_sec.is_none() {
            // Evaluation of target ended at a higher level than scope.
            // Scope does not contain target
            return false;
        }

        if next_scope_sec.unwrap() != next_target_sec.unwrap() {
            // Scope and target diverge
            // Scope does not contain target
            return false;
        }
    }
}
