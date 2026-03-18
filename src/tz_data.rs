use std::collections::HashMap;
use std::sync::LazyLock;

use chrono::{NaiveDate, TimeZone};
use chrono_tz::{OffsetName, Tz, TZ_VARIANTS};

// ---------------------------------------------------------------------------
// Resolved timezone result
// ---------------------------------------------------------------------------

pub struct ResolvedTz {
    pub tz: Tz,
    pub city: String,
    pub region: String,
}

// ---------------------------------------------------------------------------
// Abbreviation map: built once from TZ_VARIANTS via LazyLock
// ---------------------------------------------------------------------------

/// Zones we want to win when abbreviation collisions occur (e.g. CST, IST, EST).
/// Listed in priority order -- first match for a given abbreviation wins.
const PREFERRED_ZONES: &[Tz] = &[
    Tz::America__New_York,
    Tz::America__Chicago,
    Tz::America__Denver,
    Tz::America__Los_Angeles,
    Tz::America__Anchorage,
    Tz::Pacific__Honolulu,
    Tz::Europe__London,
    Tz::Europe__Berlin,
    Tz::Europe__Bucharest,
    Tz::Europe__Moscow,
    Tz::Asia__Kolkata,
    Tz::Asia__Shanghai,
    Tz::Asia__Tokyo,
    Tz::Asia__Seoul,
    Tz::Asia__Dubai,
    Tz::Australia__Sydney,
    Tz::Pacific__Auckland,
    Tz::Africa__Nairobi,
    Tz::Africa__Lagos,
    Tz::Africa__Johannesburg,
    Tz::America__Sao_Paulo,
    Tz::America__Argentina__Buenos_Aires,
];

fn build_abbreviation_map() -> HashMap<String, Tz> {
    let mut map: HashMap<String, Tz> = HashMap::new();

    let winter = NaiveDate::from_ymd_opt(2025, 1, 15)
        .unwrap()
        .and_hms_opt(12, 0, 0)
        .unwrap();
    let summer = NaiveDate::from_ymd_opt(2025, 7, 15)
        .unwrap()
        .and_hms_opt(12, 0, 0)
        .unwrap();

    // Insert preferred zones first so they win collisions
    for &tz in PREFERRED_ZONES {
        for nd in [winter, summer] {
            if let Some(dt) = tz.from_local_datetime(&nd).earliest() {
                if let Some(abbr) = dt.offset().abbreviation() {
                    let key = abbr.to_uppercase();
                    map.entry(key).or_insert(tz);
                }
            }
        }
    }

    // Then all remaining zones
    for &tz in &TZ_VARIANTS {
        for nd in [winter, summer] {
            if let Some(dt) = tz.from_local_datetime(&nd).earliest() {
                if let Some(abbr) = dt.offset().abbreviation() {
                    let key = abbr.to_uppercase();
                    map.entry(key).or_insert(tz);
                }
            }
        }
    }

    map.insert("UTC".to_string(), Tz::UTC);
    map
}

static ABBR_MAP: LazyLock<HashMap<String, Tz>> = LazyLock::new(build_abbreviation_map);

pub fn lookup_abbreviation(abbr: &str) -> Option<ResolvedTz> {
    let tz = ABBR_MAP.get(&abbr.to_uppercase()).copied()?;
    let iana = tz.name();
    let (city, region) = city_and_region(iana);
    Some(ResolvedTz { tz, city, region })
}

// ---------------------------------------------------------------------------
// City aliases: common names that don't appear in IANA zone paths
// ---------------------------------------------------------------------------

struct CityAlias {
    name: &'static str,
    iana_id: &'static str,
    display_city: &'static str,
    display_region: &'static str,
}

static CITY_ALIASES: &[CityAlias] = &[
    // North America - cities sharing a zone with a different IANA city name
    CityAlias { name: "san jose",        iana_id: "America/Los_Angeles", display_city: "San Jose",       display_region: "United States, California" },
    CityAlias { name: "san francisco",   iana_id: "America/Los_Angeles", display_city: "San Francisco",  display_region: "United States, California" },
    CityAlias { name: "san diego",       iana_id: "America/Los_Angeles", display_city: "San Diego",      display_region: "United States, California" },
    CityAlias { name: "seattle",         iana_id: "America/Los_Angeles", display_city: "Seattle",        display_region: "United States, Washington" },
    CityAlias { name: "portland",        iana_id: "America/Los_Angeles", display_city: "Portland",       display_region: "United States, Oregon" },
    CityAlias { name: "las vegas",       iana_id: "America/Los_Angeles", display_city: "Las Vegas",      display_region: "United States, Nevada" },
    CityAlias { name: "dallas",          iana_id: "America/Chicago",     display_city: "Dallas",         display_region: "United States, Texas" },
    CityAlias { name: "houston",         iana_id: "America/Chicago",     display_city: "Houston",        display_region: "United States, Texas" },
    CityAlias { name: "austin",          iana_id: "America/Chicago",     display_city: "Austin",         display_region: "United States, Texas" },
    CityAlias { name: "minneapolis",     iana_id: "America/Chicago",     display_city: "Minneapolis",    display_region: "United States, Minnesota" },
    CityAlias { name: "atlanta",         iana_id: "America/New_York",    display_city: "Atlanta",        display_region: "United States, Georgia" },
    CityAlias { name: "miami",           iana_id: "America/New_York",    display_city: "Miami",          display_region: "United States, Florida" },
    CityAlias { name: "boston",           iana_id: "America/New_York",    display_city: "Boston",         display_region: "United States, Massachusetts" },
    CityAlias { name: "washington",      iana_id: "America/New_York",    display_city: "Washington",     display_region: "United States, D.C." },
    CityAlias { name: "philadelphia",    iana_id: "America/New_York",    display_city: "Philadelphia",   display_region: "United States, Pennsylvania" },
    CityAlias { name: "salt lake city",  iana_id: "America/Denver",      display_city: "Salt Lake City", display_region: "United States, Utah" },
    CityAlias { name: "ottawa",          iana_id: "America/Toronto",     display_city: "Ottawa",         display_region: "Canada, Ontario" },
    CityAlias { name: "montreal",        iana_id: "America/Toronto",     display_city: "Montreal",       display_region: "Canada, Quebec" },
    CityAlias { name: "calgary",         iana_id: "America/Edmonton",    display_city: "Calgary",        display_region: "Canada, Alberta" },
    CityAlias { name: "guadalajara",     iana_id: "America/Mexico_City", display_city: "Guadalajara",    display_region: "Mexico" },

    // South America
    CityAlias { name: "rio de janeiro",  iana_id: "America/Sao_Paulo",   display_city: "Rio de Janeiro", display_region: "Brazil" },
    CityAlias { name: "brasilia",        iana_id: "America/Sao_Paulo",   display_city: "Brasilia",       display_region: "Brazil" },
    CityAlias { name: "medellin",        iana_id: "America/Bogota",      display_city: "Medellin",       display_region: "Colombia" },
    CityAlias { name: "quito",           iana_id: "America/Guayaquil",   display_city: "Quito",          display_region: "Ecuador" },

    // Europe
    CityAlias { name: "munich",          iana_id: "Europe/Berlin",       display_city: "Munich",         display_region: "Germany" },
    CityAlias { name: "frankfurt",       iana_id: "Europe/Berlin",       display_city: "Frankfurt",      display_region: "Germany" },
    CityAlias { name: "hamburg",         iana_id: "Europe/Berlin",       display_city: "Hamburg",        display_region: "Germany" },
    CityAlias { name: "barcelona",       iana_id: "Europe/Madrid",       display_city: "Barcelona",      display_region: "Spain" },
    CityAlias { name: "milan",           iana_id: "Europe/Rome",         display_city: "Milan",          display_region: "Italy" },
    CityAlias { name: "manchester",      iana_id: "Europe/London",       display_city: "Manchester",     display_region: "United Kingdom" },
    CityAlias { name: "edinburgh",       iana_id: "Europe/London",       display_city: "Edinburgh",      display_region: "United Kingdom, Scotland" },
    CityAlias { name: "glasgow",         iana_id: "Europe/London",       display_city: "Glasgow",        display_region: "United Kingdom, Scotland" },
    CityAlias { name: "lyon",            iana_id: "Europe/Paris",        display_city: "Lyon",           display_region: "France" },
    CityAlias { name: "marseille",       iana_id: "Europe/Paris",        display_city: "Marseille",      display_region: "France" },
    CityAlias { name: "rotterdam",       iana_id: "Europe/Amsterdam",    display_city: "Rotterdam",      display_region: "Netherlands" },
    CityAlias { name: "geneva",          iana_id: "Europe/Zurich",       display_city: "Geneva",         display_region: "Switzerland" },
    CityAlias { name: "krakow",          iana_id: "Europe/Warsaw",       display_city: "Krakow",         display_region: "Poland" },
    CityAlias { name: "porto",           iana_id: "Europe/Lisbon",       display_city: "Porto",          display_region: "Portugal" },
    CityAlias { name: "kiev",            iana_id: "Europe/Kyiv",         display_city: "Kyiv",           display_region: "Ukraine" },
    CityAlias { name: "saint petersburg", iana_id: "Europe/Moscow",      display_city: "Saint Petersburg", display_region: "Russia" },
    CityAlias { name: "tirana",          iana_id: "Europe/Tirane",       display_city: "Tirana",         display_region: "Albania" },
    CityAlias { name: "cluj-napoca",     iana_id: "Europe/Bucharest",    display_city: "Cluj-Napoca",    display_region: "Romania" },
    CityAlias { name: "timisoara",       iana_id: "Europe/Bucharest",    display_city: "Timisoara",      display_region: "Romania" },
    CityAlias { name: "ankara",          iana_id: "Europe/Istanbul",     display_city: "Ankara",         display_region: "Turkey" },

    // Asia
    CityAlias { name: "mumbai",          iana_id: "Asia/Kolkata",        display_city: "Mumbai",         display_region: "India" },
    CityAlias { name: "delhi",           iana_id: "Asia/Kolkata",        display_city: "Delhi",          display_region: "India" },
    CityAlias { name: "new delhi",       iana_id: "Asia/Kolkata",        display_city: "New Delhi",      display_region: "India" },
    CityAlias { name: "bangalore",       iana_id: "Asia/Kolkata",        display_city: "Bangalore",      display_region: "India" },
    CityAlias { name: "bengaluru",       iana_id: "Asia/Kolkata",        display_city: "Bengaluru",      display_region: "India" },
    CityAlias { name: "chennai",         iana_id: "Asia/Kolkata",        display_city: "Chennai",        display_region: "India" },
    CityAlias { name: "hyderabad",       iana_id: "Asia/Kolkata",        display_city: "Hyderabad",      display_region: "India" },
    CityAlias { name: "pune",            iana_id: "Asia/Kolkata",        display_city: "Pune",           display_region: "India" },
    CityAlias { name: "ahmedabad",       iana_id: "Asia/Kolkata",        display_city: "Ahmedabad",      display_region: "India" },
    CityAlias { name: "beijing",         iana_id: "Asia/Shanghai",       display_city: "Beijing",        display_region: "China" },
    CityAlias { name: "shenzhen",        iana_id: "Asia/Shanghai",       display_city: "Shenzhen",       display_region: "China" },
    CityAlias { name: "guangzhou",       iana_id: "Asia/Shanghai",       display_city: "Guangzhou",      display_region: "China" },
    CityAlias { name: "chengdu",         iana_id: "Asia/Shanghai",       display_city: "Chengdu",        display_region: "China" },
    CityAlias { name: "osaka",           iana_id: "Asia/Tokyo",          display_city: "Osaka",          display_region: "Japan" },
    CityAlias { name: "busan",           iana_id: "Asia/Seoul",          display_city: "Busan",          display_region: "South Korea" },
    CityAlias { name: "abu dhabi",       iana_id: "Asia/Dubai",          display_city: "Abu Dhabi",      display_region: "United Arab Emirates" },
    CityAlias { name: "jeddah",          iana_id: "Asia/Riyadh",         display_city: "Jeddah",         display_region: "Saudi Arabia" },
    CityAlias { name: "tel aviv",        iana_id: "Asia/Jerusalem",      display_city: "Tel Aviv",       display_region: "Israel" },
    CityAlias { name: "lahore",          iana_id: "Asia/Karachi",        display_city: "Lahore",         display_region: "Pakistan" },
    CityAlias { name: "islamabad",       iana_id: "Asia/Karachi",        display_city: "Islamabad",      display_region: "Pakistan" },
    CityAlias { name: "hanoi",           iana_id: "Asia/Ho_Chi_Minh",    display_city: "Hanoi",          display_region: "Vietnam" },
    CityAlias { name: "bali",            iana_id: "Asia/Makassar",       display_city: "Bali",           display_region: "Indonesia" },

    // Africa
    CityAlias { name: "cape town",       iana_id: "Africa/Johannesburg", display_city: "Cape Town",      display_region: "South Africa" },
    CityAlias { name: "durban",          iana_id: "Africa/Johannesburg", display_city: "Durban",         display_region: "South Africa" },
    CityAlias { name: "pretoria",        iana_id: "Africa/Johannesburg", display_city: "Pretoria",       display_region: "South Africa" },
    CityAlias { name: "rabat",           iana_id: "Africa/Casablanca",   display_city: "Rabat",          display_region: "Morocco" },
    CityAlias { name: "alexandria",      iana_id: "Africa/Cairo",        display_city: "Alexandria",     display_region: "Egypt" },
    CityAlias { name: "abuja",           iana_id: "Africa/Lagos",        display_city: "Abuja",          display_region: "Nigeria" },

    // Oceania
    CityAlias { name: "canberra",        iana_id: "Australia/Sydney",    display_city: "Canberra",       display_region: "Australia, ACT" },
    CityAlias { name: "christchurch",    iana_id: "Pacific/Auckland",    display_city: "Christchurch",   display_region: "New Zealand" },
    CityAlias { name: "wellington",      iana_id: "Pacific/Auckland",    display_city: "Wellington",     display_region: "New Zealand" },
];

// ---------------------------------------------------------------------------
// Region display names for common IANA zones
// ---------------------------------------------------------------------------

static REGION_NAMES: &[(&str, &str)] = &[
    ("America/New_York",       "United States, New York"),
    ("America/Chicago",        "United States, Illinois"),
    ("America/Denver",         "United States, Colorado"),
    ("America/Los_Angeles",    "United States, California"),
    ("America/Phoenix",        "United States, Arizona"),
    ("America/Anchorage",      "United States, Alaska"),
    ("America/Detroit",        "United States, Michigan"),
    ("Pacific/Honolulu",       "United States, Hawaii"),
    ("America/Toronto",        "Canada, Ontario"),
    ("America/Vancouver",      "Canada, British Columbia"),
    ("America/Edmonton",       "Canada, Alberta"),
    ("America/Winnipeg",       "Canada, Manitoba"),
    ("America/Halifax",        "Canada, Nova Scotia"),
    ("America/St_Johns",       "Canada, Newfoundland"),
    ("America/Mexico_City",    "Mexico"),
    ("America/Sao_Paulo",      "Brazil"),
    ("America/Argentina/Buenos_Aires", "Argentina"),
    ("America/Santiago",       "Chile"),
    ("America/Bogota",         "Colombia"),
    ("America/Lima",           "Peru"),
    ("Europe/London",          "United Kingdom"),
    ("Europe/Paris",           "France"),
    ("Europe/Berlin",          "Germany"),
    ("Europe/Madrid",          "Spain"),
    ("Europe/Rome",            "Italy"),
    ("Europe/Amsterdam",       "Netherlands"),
    ("Europe/Brussels",        "Belgium"),
    ("Europe/Vienna",          "Austria"),
    ("Europe/Zurich",          "Switzerland"),
    ("Europe/Stockholm",       "Sweden"),
    ("Europe/Oslo",            "Norway"),
    ("Europe/Copenhagen",      "Denmark"),
    ("Europe/Helsinki",        "Finland"),
    ("Europe/Warsaw",          "Poland"),
    ("Europe/Prague",          "Czech Republic"),
    ("Europe/Budapest",        "Hungary"),
    ("Europe/Bucharest",       "Romania"),
    ("Europe/Athens",          "Greece"),
    ("Europe/Istanbul",        "Turkey"),
    ("Europe/Moscow",          "Russia"),
    ("Europe/Lisbon",          "Portugal"),
    ("Europe/Dublin",          "Ireland"),
    ("Europe/Kyiv",            "Ukraine"),
    ("Europe/Sofia",           "Bulgaria"),
    ("Europe/Belgrade",        "Serbia"),
    ("Europe/Zagreb",          "Croatia"),
    ("Europe/Minsk",           "Belarus"),
    ("Europe/Chisinau",        "Moldova"),
    ("Asia/Tokyo",             "Japan"),
    ("Asia/Shanghai",          "China"),
    ("Asia/Hong_Kong",         "China"),
    ("Asia/Singapore",         "Singapore"),
    ("Asia/Seoul",             "South Korea"),
    ("Asia/Taipei",            "Taiwan"),
    ("Asia/Bangkok",           "Thailand"),
    ("Asia/Kolkata",           "India"),
    ("Asia/Dubai",             "United Arab Emirates"),
    ("Asia/Riyadh",            "Saudi Arabia"),
    ("Asia/Qatar",             "Qatar"),
    ("Asia/Jerusalem",         "Israel"),
    ("Asia/Karachi",           "Pakistan"),
    ("Asia/Dhaka",             "Bangladesh"),
    ("Asia/Manila",            "Philippines"),
    ("Asia/Kuala_Lumpur",      "Malaysia"),
    ("Asia/Ho_Chi_Minh",       "Vietnam"),
    ("Asia/Jakarta",           "Indonesia"),
    ("Asia/Colombo",           "Sri Lanka"),
    ("Asia/Kathmandu",         "Nepal"),
    ("Asia/Tehran",            "Iran"),
    ("Asia/Almaty",            "Kazakhstan"),
    ("Asia/Tashkent",          "Uzbekistan"),
    ("Asia/Tbilisi",           "Georgia"),
    ("Asia/Baku",              "Azerbaijan"),
    ("Asia/Yerevan",           "Armenia"),
    ("Asia/Yangon",            "Myanmar"),
    ("Africa/Cairo",           "Egypt"),
    ("Africa/Lagos",           "Nigeria"),
    ("Africa/Johannesburg",    "South Africa"),
    ("Africa/Nairobi",         "Kenya"),
    ("Africa/Casablanca",      "Morocco"),
    ("Africa/Accra",           "Ghana"),
    ("Africa/Addis_Ababa",     "Ethiopia"),
    ("Australia/Sydney",       "Australia, New South Wales"),
    ("Australia/Melbourne",    "Australia, Victoria"),
    ("Australia/Brisbane",     "Australia, Queensland"),
    ("Australia/Perth",        "Australia, Western Australia"),
    ("Australia/Adelaide",     "Australia, South Australia"),
    ("Australia/Darwin",       "Australia, Northern Territory"),
    ("Australia/Hobart",       "Australia, Tasmania"),
    ("Pacific/Auckland",       "New Zealand"),
    ("Pacific/Fiji",           "Fiji"),
    ("Atlantic/Reykjavik",     "Iceland"),
];

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Extract city name and region from an IANA identifier.
/// Checks the REGION_NAMES table for a nice display name, otherwise falls back
/// to parsing the IANA path components.
pub fn city_and_region(iana_id: &str) -> (String, String) {
    if iana_id == "UTC" {
        return ("UTC".to_string(), "Coordinated Universal Time".to_string());
    }

    let city = iana_city_name(iana_id);

    let region = REGION_NAMES
        .iter()
        .find(|(id, _)| *id == iana_id)
        .map(|(_, r)| r.to_string())
        .unwrap_or_else(|| {
            iana_id
                .split('/')
                .next()
                .unwrap_or("")
                .to_string()
        });

    (city, region)
}

/// Look up a city name and return the resolved timezone.
/// First checks aliases, then scans TZ_VARIANTS for a matching IANA city name.
pub fn lookup_city(name: &str) -> Option<ResolvedTz> {
    let lower = name.to_lowercase();

    // Check aliases first
    if let Some(alias) = CITY_ALIASES.iter().find(|a| a.name == lower) {
        let tz: Tz = alias.iana_id.parse().ok()?;
        return Some(ResolvedTz {
            tz,
            city: alias.display_city.to_string(),
            region: alias.display_region.to_string(),
        });
    }

    // Scan TZ_VARIANTS for a matching city component
    for &tz in &TZ_VARIANTS {
        let iana = tz.name();
        let iana_city = iana_city_name(iana).to_lowercase();
        if iana_city == lower {
            let (city, region) = city_and_region(iana);
            return Some(ResolvedTz { tz, city, region });
        }
    }

    None
}

fn iana_city_name(iana_id: &str) -> String {
    iana_id
        .split('/')
        .last()
        .unwrap_or(iana_id)
        .replace('_', " ")
}

const TWELVE_HOUR_REGIONS: &[&str] = &[
    "United States",
    "Canada",
    "United Kingdom",
    "Ireland",
    "Australia",
    "New Zealand",
    "India",
    "Pakistan",
    "Bangladesh",
    "Sri Lanka",
    "Nepal",
    "Philippines",
    "Malaysia",
    "Colombia",
    "Mexico",
    "Egypt",
    "Saudi Arabia",
];

pub fn uses_12h_clock(iana_id: &str) -> bool {
    if let Some(region) = REGION_NAMES.iter().find(|(id, _)| *id == iana_id).map(|(_, r)| *r) {
        return TWELVE_HOUR_REGIONS
            .iter()
            .any(|&prefix| region.starts_with(prefix));
    }

    iana_id.starts_with("America/Indiana/")
        || iana_id.starts_with("America/Kentucky/")
        || iana_id.starts_with("America/North_Dakota/")
        || iana_id.starts_with("Australia/")
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Abbreviation resolution (from spec: "Add timezone by abbreviation") ---

    #[test]
    fn abbreviation_pst_resolves_to_los_angeles() {
        let r = lookup_abbreviation("PST").expect("PST should resolve");
        assert_eq!(r.tz.name(), "America/Los_Angeles");
    }

    #[test]
    fn abbreviation_case_insensitive() {
        let r = lookup_abbreviation("pst").expect("pst should resolve");
        assert_eq!(r.tz.name(), "America/Los_Angeles");
    }

    #[test]
    fn abbreviation_eet_resolves_to_bucharest() {
        let r = lookup_abbreviation("EET").expect("EET should resolve");
        assert_eq!(r.tz.name(), "Europe/Bucharest");
    }

    #[test]
    fn abbreviation_cst_resolves_to_chicago() {
        let r = lookup_abbreviation("CST").expect("CST should resolve");
        assert_eq!(r.tz.name(), "America/Chicago");
    }

    #[test]
    fn abbreviation_ist_resolves_to_kolkata() {
        let r = lookup_abbreviation("IST").expect("IST should resolve");
        assert_eq!(r.tz.name(), "Asia/Kolkata");
    }

    #[test]
    fn abbreviation_jst_resolves_to_tokyo() {
        let r = lookup_abbreviation("JST").expect("JST should resolve");
        assert_eq!(r.tz.name(), "Asia/Tokyo");
    }

    #[test]
    fn abbreviation_utc_resolves() {
        let r = lookup_abbreviation("UTC").expect("UTC should resolve");
        assert_eq!(r.tz.name(), "UTC");
    }

    #[test]
    fn abbreviation_unknown_returns_none() {
        assert!(lookup_abbreviation("XYZ").is_none());
    }

    // --- City name resolution (from spec: "Add timezone by city name") ---

    #[test]
    fn city_bucharest_resolves_via_iana_scan() {
        let r = lookup_city("Bucharest").expect("Bucharest should resolve");
        assert_eq!(r.tz.name(), "Europe/Bucharest");
        assert_eq!(r.region, "Romania");
    }

    #[test]
    fn city_san_jose_resolves_via_alias() {
        let r = lookup_city("San Jose").expect("San Jose should resolve");
        assert_eq!(r.tz.name(), "America/Los_Angeles");
        assert_eq!(r.city, "San Jose");
        assert_eq!(r.region, "United States, California");
    }

    #[test]
    fn city_mumbai_resolves_via_alias() {
        let r = lookup_city("Mumbai").expect("Mumbai should resolve");
        assert_eq!(r.tz.name(), "Asia/Kolkata");
        assert_eq!(r.city, "Mumbai");
        assert_eq!(r.region, "India");
    }

    #[test]
    fn city_beijing_resolves_via_alias() {
        let r = lookup_city("Beijing").expect("Beijing should resolve");
        assert_eq!(r.tz.name(), "Asia/Shanghai");
        assert_eq!(r.city, "Beijing");
    }

    #[test]
    fn city_tokyo_resolves_via_iana_scan() {
        let r = lookup_city("Tokyo").expect("Tokyo should resolve");
        assert_eq!(r.tz.name(), "Asia/Tokyo");
        assert_eq!(r.region, "Japan");
    }

    #[test]
    fn city_lookup_case_insensitive() {
        let r = lookup_city("bucharest").expect("lowercase should work");
        assert_eq!(r.tz.name(), "Europe/Bucharest");
    }

    #[test]
    fn city_unknown_returns_none() {
        assert!(lookup_city("Atlantis").is_none());
    }

    // --- city_and_region display ---

    #[test]
    fn city_and_region_utc() {
        let (city, region) = city_and_region("UTC");
        assert_eq!(city, "UTC");
        assert_eq!(region, "Coordinated Universal Time");
    }

    #[test]
    fn city_and_region_with_override() {
        let (city, region) = city_and_region("America/Los_Angeles");
        assert_eq!(city, "Los Angeles");
        assert_eq!(region, "United States, California");
    }

    #[test]
    fn city_and_region_fallback() {
        let (city, region) = city_and_region("Asia/Dili");
        assert_eq!(city, "Dili");
        assert_eq!(region, "Asia");
    }

    // --- Abbreviation map covers daylight variants ---

    #[test]
    fn abbreviation_pdt_resolves_to_los_angeles() {
        let r = lookup_abbreviation("PDT").expect("PDT should resolve");
        assert_eq!(r.tz.name(), "America/Los_Angeles");
    }

    #[test]
    fn abbreviation_edt_resolves_to_new_york() {
        let r = lookup_abbreviation("EDT").expect("EDT should resolve");
        assert_eq!(r.tz.name(), "America/New_York");
    }

    #[test]
    fn abbreviation_bst_resolves_to_london() {
        let r = lookup_abbreviation("BST").expect("BST should resolve");
        assert_eq!(r.tz.name(), "Europe/London");
    }
}
