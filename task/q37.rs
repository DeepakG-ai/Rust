// ============================================================================
// ## Q37 — Alerts (revision of Q10–Q12: traits, default methods, dyn, generics)
//
// No signatures given on purpose. Design the functions yourself.
//
// Data and traits
// - An alert always has a severity (0 to 10) and a message.
// - Every alert can produce a display line like "[CRITICAL] disk almost full".
//   The word comes from severity: 0-3 INFO, 4-7 WARN, 8-10 CRITICAL.
//   This logic is written ONCE and shared.
// - Three kinds of alert: Disk, Cpu, Security.
//     - Disk and Cpu use the shared display line as it is.
//     - Security overrides it: always "[!!SECURITY!!] message", whatever the severity.
// - Every alert can be escalated: severity goes up by 2, capped at 10. It changes
//   the alert itself.
//
// Behaviour to build
// 1. Make an alert from text. "disk:9", "cpu:3", "security:10" each make the right
//    kind of alert. It returns the alert as ONE common kind of value, so different
//    kinds can sit in one list. Errors:
//      - unknown kind (say which)
//      - missing ':'
//      - severity not a number
//      - severity above 10
// 2. Make many. Turn a list of texts into a list of mixed alerts. The first failure
//    stops everything and is returned as the error.
// 3. Most severe. Given the list, return the most severe alert, or nothing if the
//    list is empty. It must not copy or take the list.
// 4. Escalate all. Escalate every alert in the list, in place.
// 5. Critical check for a single alert. It must work with ANY one concrete alert
//    type. It must be compiled per type (not looked up at run time), and it must
//    not take ownership. It returns true when severity is 8 or more.
// 6. Print all. Print every display line in the list.
//
// Main must show
// - a good list of at least 5 texts (all three kinds) -> print all
// - the most severe one
// - escalate all, print all again, and show that severity 9 stays capped at 10
// - a bad list, where the third text has an unknown kind -> print the error
// - the critical check called on a plain Disk alert that you build directly
//   (not from the list)
// - a comment answering: why can item 5 not be used on the alerts inside the list,
//   but item 6 can?
// ============================================================================

fn main() {}
