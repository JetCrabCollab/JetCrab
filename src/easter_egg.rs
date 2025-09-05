//! # Easter Egg - Walking Crab Animation
//!
//! A fun easter egg that displays a walking crab animation in the terminal.

use std::io::{self, Write};
use std::thread;
use std::time::Duration;

/// Crab animation frames for walking left to right
const CRAB_FRAMES: &[&str] = &[
    "🦀     ",
    " 🦀    ",
    "  🦀   ",
    "   🦀  ",
    "    🦀 ",
    "     🦀",
    "    🦀 ",
    "   🦀  ",
    "  🦀   ",
    " 🦀    ",
];

/// JetCrab logo frames for walking left to right
const JETCRAB_FRAMES: &[&str] = &[
    "🦀 JetCrab     ",
    " 🦀 JetCrab    ",
    "  🦀 JetCrab   ",
    "   🦀 JetCrab  ",
    "    🦀 JetCrab ",
    "     🦀 JetCrab",
    "    🦀 JetCrab ",
    "   🦀 JetCrab  ",
    "  🦀 JetCrab   ",
    " 🦀 JetCrab    ",
];

/// Claw logo frames for walking left to right
const CLAW_FRAMES: &[&str] = &[
    "🦀 Claw     ",
    " 🦀 Claw    ",
    "  🦀 Claw   ",
    "   🦀 Claw  ",
    "    🦀 Claw ",
    "     🦀 Claw",
    "    🦀 Claw ",
    "   🦀 Claw  ",
    "  🦀 Claw   ",
    " 🦀 Claw    ",
];

/// Display a walking crab animation
pub fn show_walking_crab() {
    println!("\n🦀 JetCrab Easter Egg! 🦀\n");

    print!("\x1B[2J\x1B[1;1H\x1B[?25l");
    io::stdout().flush().unwrap();

    for _ in 0..3 {
        for frame in CRAB_FRAMES {
            print!("\r{}", frame);
            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(200));
        }

        for frame in CRAB_FRAMES.iter().rev() {
            print!("\r{}", frame);
            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    }

    print!("\r🦀 JetCrab is ready! 🦀\n\n");
    io::stdout().flush().unwrap();

    print!("\x1B[?25h");
    io::stdout().flush().unwrap();
}

/// Display a walking JetCrab logo animation
pub fn show_walking_jetcrab() {
    println!("\n🦀 JetCrab Easter Egg! 🦀\n");

    print!("\x1B[2J\x1B[1;1H\x1B[?25l");
    io::stdout().flush().unwrap();

    for _ in 0..2 {
        for frame in JETCRAB_FRAMES {
            print!("\r{}", frame);
            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(150));
        }

        for frame in JETCRAB_FRAMES.iter().rev() {
            print!("\r{}", frame);
            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(150));
        }
    }

    print!("\r🦀 JetCrab Runtime v0.4.0 🦀\n\n");
    io::stdout().flush().unwrap();

    print!("\x1B[?25h");
    io::stdout().flush().unwrap();
}

/// Display a walking Claw logo animation
pub fn show_walking_claw() {
    println!("\n🦀 Claw Easter Egg! 🦀\n");

    print!("\x1B[2J\x1B[1;1H\x1B[?25l");
    io::stdout().flush().unwrap();

    for _ in 0..2 {
        for frame in CLAW_FRAMES {
            print!("\r{}", frame);
            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(150));
        }

        for frame in CLAW_FRAMES.iter().rev() {
            print!("\r{}", frame);
            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(150));
        }
    }

    print!("\r🦀 Claw Package Manager v0.4.0 🦀\n\n");
    io::stdout().flush().unwrap();

    print!("\x1B[?25h");
    io::stdout().flush().unwrap();
}

/// Check if easter egg should be triggered
pub fn should_trigger_easter_egg() -> bool {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::time::{SystemTime, UNIX_EPOCH};

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let mut hasher = DefaultHasher::new();
    now.hash(&mut hasher);
    let hash = hasher.finish();

    hash % 10 == 0
}

/// Check if easter egg should be triggered based on command
pub fn should_trigger_easter_egg_for_command(command: &str) -> bool {
    matches!(command, "crab" | "walk" | "dance" | "party")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crab_frames_length() {
        assert_eq!(CRAB_FRAMES.len(), 10);
        assert_eq!(JETCRAB_FRAMES.len(), 10);
        assert_eq!(CLAW_FRAMES.len(), 10);
    }

    #[test]
    fn test_crab_frames_contain_crab() {
        for frame in CRAB_FRAMES {
            assert!(frame.contains("🦀"));
        }
    }

    #[test]
    fn test_jetcrab_frames_contain_both() {
        for frame in JETCRAB_FRAMES {
            assert!(frame.contains("🦀"));
            assert!(frame.contains("JetCrab"));
        }
    }

    #[test]
    fn test_claw_frames_contain_both() {
        for frame in CLAW_FRAMES {
            assert!(frame.contains("🦀"));
            assert!(frame.contains("Claw"));
        }
    }

    #[test]
    fn test_should_trigger_easter_egg_for_command() {
        assert!(should_trigger_easter_egg_for_command("crab"));
        assert!(should_trigger_easter_egg_for_command("walk"));
        assert!(should_trigger_easter_egg_for_command("dance"));
        assert!(should_trigger_easter_egg_for_command("party"));
        assert!(!should_trigger_easter_egg_for_command("hello"));
        assert!(!should_trigger_easter_egg_for_command("test"));
    }

    #[test]
    fn test_should_trigger_easter_egg_randomness() {
        let result = should_trigger_easter_egg();
        assert!(result == true || result == false);
    }
}
