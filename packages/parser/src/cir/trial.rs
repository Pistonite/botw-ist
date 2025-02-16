use teleparse::Span;

use crate::error::{Error, ErrorReport};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Trial {
    Eventide,
    SwordNext,
    Sword1,
    Sword2,
    Sword3,
    RefightThunder,
    RefightWater,
    RefightFire,
    RefightWind,
}

pub fn parse_trial(trial_name: &str, span: &Span) -> Result<Trial, ErrorReport> {
    let space_removed = trial_name.to_ascii_lowercase().replace('_', "-");
    match space_removed.as_str() {
        "eventide" => Ok(Trial::Eventide),
        "tots" | "trial-of-the-sword" => Ok(Trial::SwordNext),
        "beginning-trial" => Ok(Trial::Sword1),
        "middle-trial" => Ok(Trial::Sword2),
        "final-trial" => Ok(Trial::Sword3),
        "thunderblight-refight" => Ok(Trial::RefightThunder),
        "waterblight-refight" => Ok(Trial::RefightWater),
        "fireblight-refight" => Ok(Trial::RefightFire),
        "windblight-refight" => Ok(Trial::RefightWind),
        _ => Err(ErrorReport::spanned(
            span,
            Error::InvalidTrial(trial_name.to_string()),
        )),
    }
}
