pub fn is_overheat_disabled() -> bool {
    std::option_env!("MBRH_DISABLE_OVERHEAT").is_some()
}