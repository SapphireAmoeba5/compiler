/// Wrapper around println that gets stripped on Release builds
#[macro_export]
macro_rules! debug_println {
    ($($args:expr), *) => (if cfg!(debug_assertions) {
        println!("[DEBUG] {}", format!($($args), *));
    })
}

/// Wrapper around println
#[macro_export]
macro_rules! error_println {
    ($($args:expr), *) => {
        println!("[ERROR] {}", format!($($args), *));
    }
}

/// Wrapper around println
#[macro_export]
macro_rules! info_println {
    ($($args:expr), *) => {
        println!("[INFO] {}", format!($($args), *));
    }
}

/// Wrapper around println that only prints if a condition is true
#[macro_export]
macro_rules! info_println_if {
    ($doif: expr, $($args:expr), *) => {
        if $doif {
            println!("[INFO] {}", format!($($args), *));
        }
    }
}
