use std::time::{SystemTime, UNIX_EPOCH};
use regex::Regex;

pub fn get_unix_timestamp() -> u64 {
    let now = SystemTime::now();
    let duration_since_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    duration_since_epoch.as_secs()
}

pub fn is_ascii(input: &str) -> bool {
    for c in input.chars() {
        if (c as u32) > 127 || (c as u32) < 32 {
            return false;
        }
    }
    true
}

pub fn is_valid_email(email: &str) -> bool {
    let re = Regex::new(r"^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$").unwrap();
    re.is_match(email)
}

/// - 7 - 50 characters
/// - 1 upper case letter
/// - 1 lower case letter
/// - 1 special from this list: !@#$%^&*()-_=+[]{}\|<>,./?
pub fn is_valid_password(password: &str) -> bool {
    if password.len() < 7 || 50 < password.len() {
        return false;
    }

    let has_lowercase = password.chars().any(|c| c.is_ascii_lowercase());
    let has_uppercase = password.chars().any(|c| c.is_ascii_uppercase());
    let has_no_space = ! password.contains(char::is_whitespace);
    let has_special = password.chars().any(|c| "!@#$%^&*()-_=+[]{}\\|<>,./?".contains(c));

    has_lowercase && has_uppercase && has_special && has_no_space
}

/// - 5 - 25 characters
/// - at least one letter
/// - no space allowed
pub fn is_valid_username(username: &str) -> bool {
    let len = username.len();
    if ! (5..=25).contains(&len) {
        return false;
    };

    let has_letter = username.chars().any(|c| c.is_ascii_lowercase() || c.is_ascii_uppercase());
    let has_no_space = ! username.contains(char::is_whitespace);

    has_letter && has_no_space
}

pub fn is_valid_country_code(country_code: &str) -> bool {
    matches!(country_code, "AFG"|"ALB"|"DZA"|"ASM"|"AND"|"AGO"|"AIA"|"ATA"|"ATG"|"ARG"|"ARM"|"ABW"|"AUS"|"AUT"|"AZE"|"BHS"|"BHR"|"BGD"|"BRB"|"BLR"|"BEL"|"BLZ"|"BEN"|"BMU"|"BTN"|"BOL"|"BES"|"BIH"|"BWA"|"BVT"|"BRA"|"IOT"|"BRN"|"BGR"|"BFA"|"BDI"|"KHM"|"CMR"|"CAN"|"CPV"|"CYM"|"CAF"|"TCD"|"CHL"|"CHN"|"CXR"|"CCK"|"COL"|"COM"|"COG"|"COD"|"COK"|"CRI"|"HRV"|"CUB"|"CUW"|"CYP"|"CZE"|"DNK"|"DJI"|"DMA"|"DOM"|"TLS"|"ECU"|"EGY"|"SLV"|"GNQ"|"ERI"|"EST"|"SWZ"|"ETH"|"FLK"|"FRO"|"FJI"|"FIN"|"FRA"|"GUF"|"PYF"|"ATF"|"GAB"|"GMB"|"GEO"|"DEU"|"GHA"|"GIB"|"GRC"|"GRL"|"GRD"|"GLP"|"GUM"|"GTM"|"GGY"|"GIN"|"GNB"|"GUY"|"HTI"|"HMD"|"VAT"|"HND"|"HKG"|"HUN"|"ISL"|"IND"|"IDN"|"IRN"|"IRQ"|"IRL"|"IMN"|"ISR"|"ITA"|"CIV"|"JAM"|"JPN"|"JEY"|"JOR"|"KAZ"|"KEN"|"KIR"|"PRK"|"KOR"|"KWT"|"KGZ"|"LAO"|"LVA"|"LBN"|"LSO"|"LBR"|"LBY"|"LIE"|"LTU"|"LUX"|"MAC"|"MDG"|"MWI"|"MYS"|"MDV"|"MLI"|"MLT"|"MHL"|"MTQ"|"MRT"|"MUS"|"MYT"|"MEX"|"FSM"|"MDA"|"MCO"|"MNG"|"MNE"|"MSR"|"MAR"|"MOZ"|"MMR"|"NAM"|"NRU"|"NPL"|"NLD"|"NCL"|"NZL"|"NIC"|"NER"|"NGA"|"NIU"|"NFK"|"MNP"|"NOR"|"OMN"|"PAK"|"PLW"|"PSE"|"PAN"|"PNG"|"PRY"|"PER"|"PHL"|"PCN"|"POL"|"PRT"|"PRI"|"QAT"|"MKD"|"ROU"|"RUS"|"RWA"|"REU"|"BLM"|"SHN"|"KNA"|"LCA"|"MAF"|"SPM"|"VCT"|"WSM"|"SMR"|"STP"|"SAU"|"SEN"|"SRB"|"SYC"|"SLE"|"SGP"|"SXM"|"SVK"|"SVN"|"SLB"|"SOM"|"ZAF"|"SGS"|"SSD"|"ESP"|"LKA"|"SDN"|"SUR"|"SJM"|"SWE"|"CHE"|"SYR"|"TWN"|"TJK"|"TZA"|"THA"|"TGO"|"TKL"|"TON"|"TTO"|"TUN"|"TUR"|"TKM"|"TCA"|"TUV"|"UGA"|"UKR"|"ARE"|"GBR"|"USA"|"UMI"|"URY"|"UZB"|"VUT"|"VEN"|"VNM"|"VIR"|"WLF"|"ESH"|"YEM"|"ZMB"|"ZWE")
}

/// - 3 - 25 character
pub fn is_valid_stack_name(stack_name: &str) -> bool {
    if stack_name.len() < 3 || 25 < stack_name.len() {
        return false;
    }

    true
}

pub fn parse_tags(tags: &str) -> String {
    tags.split(',').map(|part| part.trim()).collect::<Vec<&str>>().join(",")
}

/// max 10 of 20 char long tags
pub fn is_valid_tags(tags: &str) -> bool {
    let tags_iter = tags.split(',').map(|part| part.trim());
    let tags_len = tags_iter.count();
    if tags_len == 1 { return tags.len() <= 20; }
    else if tags_len > 10 { return false; }
    ! tags.split(',').map(|part| part.trim()).any(|tag| 20 < tag.len() )
}