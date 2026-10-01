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

#![allow(dead_code)]

// One trait. Required methods = what each type MUST provide.
// Default methods = logic written ONCE and shared by every type.
trait Alert {
    fn severity(&self) -> u8;
    fn message(&self) -> String;
    fn set_severity(&mut self, severity: u8);

    // default: shared word logic (0-3 INFO, 4-7 WARN, 8-10 CRITICAL)
    fn display_line(&self) -> String {
        let word = match self.severity() {
            0..=3 => "INFO",
            4..=7 => "WARN",
            _ => "CRITICAL",
        };
        format!("[{}] {}", word, self.message())
    }

    // default: +2, capped at 10. Changes the alert itself, so &mut self.
    fn escalate(&mut self) {
        let mut new_severity = self.severity() + 2;
        if new_severity > 10 {
            new_severity = 10;
        }
        self.set_severity(new_severity);
    }
}

struct DiskAlert {
    severity: u8,
    message: String,
}

struct CpuAlert {
    severity: u8,
    message: String,
}

struct SecurityAlert {
    severity: u8,
    message: String,
}

impl DiskAlert {
    fn new(severity: u8) -> Self {
        Self {
            severity,
            message: "disk almost full".to_string(),
        }
    }
}

impl CpuAlert {
    fn new(severity: u8) -> Self {
        Self {
            severity,
            message: "cpu overloaded".to_string(),
        }
    }
}

impl SecurityAlert {
    fn new(severity: u8) -> Self {
        Self {
            severity,
            message: "unauthorized access".to_string(),
        }
    }
}

// Disk and Cpu: only the required methods. display_line + escalate come from the trait.
impl Alert for DiskAlert {
    fn severity(&self) -> u8 {
        self.severity
    }
    fn message(&self) -> String {
        self.message.clone()
    }
    fn set_severity(&mut self, severity: u8) {
        self.severity = severity;
    }
}

impl Alert for CpuAlert {
    fn severity(&self) -> u8 {
        self.severity
    }
    fn message(&self) -> String {
        self.message.clone()
    }
    fn set_severity(&mut self, severity: u8) {
        self.severity = severity;
    }
}

// Security: same required methods, PLUS overrides the default display_line.
impl Alert for SecurityAlert {
    fn severity(&self) -> u8 {
        self.severity
    }
    fn message(&self) -> String {
        self.message.clone()
    }
    fn set_severity(&mut self, severity: u8) {
        self.severity = severity;
    }

    fn display_line(&self) -> String {
        format!("[!!SECURITY!!] {}", self.message())
    }
}

// 1. Make an alert from text. Returns Box<dyn Alert>: one common type, so
//    different kinds can sit in the same Vec.
fn parse_alert(text: &str) -> Result<Box<dyn Alert>, String> {
    let (kind, raw_severity) = match text.split_once(':') {
        Some(pair) => pair,
        None => return Err(format!("missing ':' in {}", text)),
    };

    // u32 (not u8) so "300" reports "above 10" instead of "not a number"
    let severity = match raw_severity.trim().parse::<u32>() {
        Ok(n) => n,
        Err(_) => return Err(format!("severity is not a number: {}", raw_severity.trim())),
    };

    if severity > 10 {
        return Err(format!("severity above 10: {}", severity));
    }
    let severity = severity as u8;

    match kind.trim() {
        "disk" => Ok(Box::new(DiskAlert::new(severity))),
        "cpu" => Ok(Box::new(CpuAlert::new(severity))),
        "security" => Ok(Box::new(SecurityAlert::new(severity))),
        other => Err(format!("unknown kind: {}", other)),
    }
}

// 2. Make many. The first failure leaves the function right away through `?`.
fn parse_all(texts: &[&str]) -> Result<Vec<Box<dyn Alert>>, String> {
    let mut alerts: Vec<Box<dyn Alert>> = Vec::new();

    for text in texts {
        let alert = parse_alert(text)?;
        alerts.push(alert);
    }

    Ok(alerts)
}

// 3. Most severe. Borrows the list (&), returns a borrow into it, or None if empty.
//    On a tie, the first one wins.
fn most_severe(alerts: &[Box<dyn Alert>]) -> Option<&Box<dyn Alert>> {
    let mut best: Option<&Box<dyn Alert>> = None;

    for alert in alerts {
        match best {
            None => best = Some(alert),
            Some(current) => {
                if alert.severity() > current.severity() {
                    best = Some(alert);
                }
            }
        }
    }

    best
}

// 4. Escalate all. &mut: every alert inside the list is changed in place.
fn escalate_all(alerts: &mut [Box<dyn Alert>]) {
    for alert in alerts.iter_mut() {
        alert.escalate();
    }
}

// 5. Critical check. Generic <T: Alert>: the compiler makes one copy per concrete type.
//    &T: only borrows, caller keeps the alert.
fn is_critical<T: Alert>(alert: &T) -> bool {
    alert.severity() >= 8
}

// 6. Print all. Dynamic dispatch: each Box<dyn Alert> picks its own display_line at run time.
fn print_all(alerts: &[Box<dyn Alert>]) {
    for alert in alerts {
        println!("{}", alert.display_line());
    }
}

fn main() {
    // good list: all three kinds
    let texts = ["disk:9", "cpu:3", "security:6", "cpu:8", "disk:4"];
    let mut alerts = parse_all(&texts).expect("valid alert list");

    println!("--- all alerts ---");
    print_all(&alerts);

    println!("--- most severe ---");
    match most_severe(&alerts) {
        Some(alert) => println!("{} (severity {})", alert.display_line(), alert.severity()),
        None => println!("no alerts"),
    }

    println!("--- escalate all ---");
    println!("first alert severity before: {}", alerts[0].severity());
    escalate_all(&mut alerts);
    println!("first alert severity after:  {}  (9 + 2 = 11, capped at 10)", alerts[0].severity());
    print_all(&alerts);

    // empty list -> None
    let empty: Vec<Box<dyn Alert>> = Vec::new();
    match most_severe(&empty) {
        Some(_) => println!("unexpected"),
        None => println!("empty list: no most severe alert"),
    }

    // bad list: third text has an unknown kind
    println!("--- bad list ---");
    let bad_texts = ["disk:9", "cpu:3", "gpu:5", "cpu:1"];
    match parse_all(&bad_texts) {
        Ok(_) => println!("unexpected success"),
        Err(e) => println!("Err: {}", e),
    }

    // the other error cases one by one
    for text in ["disk9", "disk:abc", "disk:11"] {
        match parse_alert(text) {
            Ok(_) => println!("unexpected success for {}", text),
            Err(e) => println!("Err: {}", e),
        }
    }

    // critical check on a plain Disk alert built directly (concrete type known)
    println!("--- critical check ---");
    let disk = DiskAlert::new(9);
    println!("disk:9 critical? {}", is_critical(&disk));
    let cpu = CpuAlert::new(3);
    println!("cpu:3 critical?  {}", is_critical(&cpu));
    println!("disk still usable: {}", disk.display_line()); // borrowed, not moved

    // Q: why can is_critical (item 5) not be used on the alerts inside the list,
    //    but print_all (item 6) can?
    //
    // A: is_critical<T: Alert> needs ONE concrete type T known at compile time, so the
    //    compiler can build a separate copy for DiskAlert, CpuAlert, etc.
    //    Inside the list, every element is a Box<dyn Alert>. The real type (Disk? Cpu?)
    //    is erased and only known at run time, so there is no T to compile a copy for.
    //    is_critical(item) fails: `Box<dyn Alert>` is not itself an Alert, and `dyn Alert`
    //    has no fixed size, which a plain T requires.
    //    print_all doesn't pick a type at compile time. It calls alert.display_line()
    //    through the Box's vtable, and the right method for each element is chosen at
    //    run time (dynamic dispatch).
}
