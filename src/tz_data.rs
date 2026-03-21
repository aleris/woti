use std::collections::HashMap;
use std::sync::LazyLock;

use chrono::{Datelike, NaiveDate, TimeZone, Utc};
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

    let year = Utc::now().year();
    let winter = NaiveDate::from_ymd_opt(year, 1, 15)
        .unwrap()
        .and_hms_opt(12, 0, 0)
        .unwrap();
    let summer = NaiveDate::from_ymd_opt(year, 7, 15)
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

static CITY_ALIASES: &[CityAlias] = &[
    // ── North America — overrides (city is in a different state than the IANA ref) ──
    CityAlias { name: "seattle",          iana_id: "America/Los_Angeles", display_city: "Seattle",        region_override: Some("United States, Washington") },
    CityAlias { name: "portland",         iana_id: "America/Los_Angeles", display_city: "Portland",       region_override: Some("United States, Oregon") },
    CityAlias { name: "las vegas",        iana_id: "America/Los_Angeles", display_city: "Las Vegas",      region_override: Some("United States, Nevada") },
    CityAlias { name: "dallas",           iana_id: "America/Chicago",     display_city: "Dallas",         region_override: Some("United States, Texas") },
    CityAlias { name: "houston",          iana_id: "America/Chicago",     display_city: "Houston",        region_override: Some("United States, Texas") },
    CityAlias { name: "austin",           iana_id: "America/Chicago",     display_city: "Austin",         region_override: Some("United States, Texas") },
    CityAlias { name: "minneapolis",      iana_id: "America/Chicago",     display_city: "Minneapolis",    region_override: Some("United States, Minnesota") },
    CityAlias { name: "atlanta",          iana_id: "America/New_York",    display_city: "Atlanta",        region_override: Some("United States, Georgia") },
    CityAlias { name: "miami",            iana_id: "America/New_York",    display_city: "Miami",          region_override: Some("United States, Florida") },
    CityAlias { name: "boston",            iana_id: "America/New_York",    display_city: "Boston",         region_override: Some("United States, Massachusetts") },
    CityAlias { name: "washington",       iana_id: "America/New_York",    display_city: "Washington",     region_override: Some("United States, D.C.") },
    CityAlias { name: "philadelphia",     iana_id: "America/New_York",    display_city: "Philadelphia",   region_override: Some("United States, Pennsylvania") },
    CityAlias { name: "salt lake city",   iana_id: "America/Denver",      display_city: "Salt Lake City", region_override: Some("United States, Utah") },
    CityAlias { name: "montreal",         iana_id: "America/Toronto",     display_city: "Montreal",       region_override: Some("Canada, Quebec") },

    // ── North America — region from REGION_NAMES ──
    CityAlias { name: "san jose",         iana_id: "America/Los_Angeles", display_city: "San Jose",       region_override: None },
    CityAlias { name: "san francisco",    iana_id: "America/Los_Angeles", display_city: "San Francisco",  region_override: None },
    CityAlias { name: "san diego",        iana_id: "America/Los_Angeles", display_city: "San Diego",      region_override: None },
    CityAlias { name: "ottawa",           iana_id: "America/Toronto",     display_city: "Ottawa",         region_override: None },
    CityAlias { name: "calgary",          iana_id: "America/Edmonton",    display_city: "Calgary",        region_override: None },
    CityAlias { name: "guadalajara",      iana_id: "America/Mexico_City", display_city: "Guadalajara",    region_override: None },

    // ── South America — region from REGION_NAMES ──
    CityAlias { name: "rio de janeiro",   iana_id: "America/Sao_Paulo",   display_city: "Rio de Janeiro", region_override: None },
    CityAlias { name: "brasilia",         iana_id: "America/Sao_Paulo",   display_city: "Brasilia",       region_override: None },
    CityAlias { name: "medellin",         iana_id: "America/Bogota",      display_city: "Medellin",       region_override: None },
    CityAlias { name: "quito",            iana_id: "America/Guayaquil",   display_city: "Quito",          region_override: None },

    // ── Europe — overrides (more specific than REGION_NAMES) ──
    CityAlias { name: "edinburgh",        iana_id: "Europe/London",       display_city: "Edinburgh",      region_override: Some("United Kingdom, Scotland") },
    CityAlias { name: "glasgow",          iana_id: "Europe/London",       display_city: "Glasgow",        region_override: Some("United Kingdom, Scotland") },

    // ── Europe — region from REGION_NAMES ──
    CityAlias { name: "munich",           iana_id: "Europe/Berlin",       display_city: "Munich",         region_override: None },
    CityAlias { name: "frankfurt",        iana_id: "Europe/Berlin",       display_city: "Frankfurt",      region_override: None },
    CityAlias { name: "hamburg",          iana_id: "Europe/Berlin",       display_city: "Hamburg",        region_override: None },
    CityAlias { name: "barcelona",        iana_id: "Europe/Madrid",       display_city: "Barcelona",      region_override: None },
    CityAlias { name: "milan",            iana_id: "Europe/Rome",         display_city: "Milan",          region_override: None },
    CityAlias { name: "manchester",       iana_id: "Europe/London",       display_city: "Manchester",     region_override: None },
    CityAlias { name: "lyon",             iana_id: "Europe/Paris",        display_city: "Lyon",           region_override: None },
    CityAlias { name: "marseille",        iana_id: "Europe/Paris",        display_city: "Marseille",      region_override: None },
    CityAlias { name: "rotterdam",        iana_id: "Europe/Amsterdam",    display_city: "Rotterdam",      region_override: None },
    CityAlias { name: "geneva",           iana_id: "Europe/Zurich",       display_city: "Geneva",         region_override: None },
    CityAlias { name: "krakow",           iana_id: "Europe/Warsaw",       display_city: "Krakow",         region_override: None },
    CityAlias { name: "porto",            iana_id: "Europe/Lisbon",       display_city: "Porto",          region_override: None },
    CityAlias { name: "kiev",             iana_id: "Europe/Kyiv",         display_city: "Kyiv",           region_override: None },
    CityAlias { name: "saint petersburg", iana_id: "Europe/Moscow",      display_city: "Saint Petersburg", region_override: None },
    CityAlias { name: "tirana",           iana_id: "Europe/Tirane",       display_city: "Tirana",         region_override: None },
    CityAlias { name: "brasov",           iana_id: "Europe/Bucharest",    display_city: "Brașov",         region_override: None },
    CityAlias { name: "constanta",        iana_id: "Europe/Bucharest",    display_city: "Constanța",      region_override: None },
    CityAlias { name: "craiova",          iana_id: "Europe/Bucharest",    display_city: "Craiova",        region_override: None },
    CityAlias { name: "iasi",             iana_id: "Europe/Bucharest",    display_city: "Iași",           region_override: None },
    CityAlias { name: "cluj-napoca",      iana_id: "Europe/Bucharest",    display_city: "Cluj-Napoca",    region_override: None },
    CityAlias { name: "timisoara",        iana_id: "Europe/Bucharest",    display_city: "Timișoara",      region_override: None },
    CityAlias { name: "ankara",           iana_id: "Europe/Istanbul",     display_city: "Ankara",         region_override: None },

    // ── Asia — region from REGION_NAMES ──
    CityAlias { name: "mumbai",           iana_id: "Asia/Kolkata",        display_city: "Mumbai",         region_override: None },
    CityAlias { name: "delhi",            iana_id: "Asia/Kolkata",        display_city: "Delhi",          region_override: None },
    CityAlias { name: "new delhi",        iana_id: "Asia/Kolkata",        display_city: "New Delhi",      region_override: None },
    CityAlias { name: "bangalore",        iana_id: "Asia/Kolkata",        display_city: "Bangalore",      region_override: None },
    CityAlias { name: "bengaluru",        iana_id: "Asia/Kolkata",        display_city: "Bengaluru",      region_override: None },
    CityAlias { name: "chennai",          iana_id: "Asia/Kolkata",        display_city: "Chennai",        region_override: None },
    CityAlias { name: "hyderabad",        iana_id: "Asia/Kolkata",        display_city: "Hyderabad",      region_override: None },
    CityAlias { name: "pune",             iana_id: "Asia/Kolkata",        display_city: "Pune",           region_override: None },
    CityAlias { name: "ahmedabad",        iana_id: "Asia/Kolkata",        display_city: "Ahmedabad",      region_override: None },
    CityAlias { name: "beijing",          iana_id: "Asia/Shanghai",       display_city: "Beijing",        region_override: None },
    CityAlias { name: "shenzhen",         iana_id: "Asia/Shanghai",       display_city: "Shenzhen",       region_override: None },
    CityAlias { name: "guangzhou",        iana_id: "Asia/Shanghai",       display_city: "Guangzhou",      region_override: None },
    CityAlias { name: "chengdu",          iana_id: "Asia/Shanghai",       display_city: "Chengdu",        region_override: None },
    CityAlias { name: "osaka",            iana_id: "Asia/Tokyo",          display_city: "Osaka",          region_override: None },
    CityAlias { name: "busan",            iana_id: "Asia/Seoul",          display_city: "Busan",          region_override: None },
    CityAlias { name: "abu dhabi",        iana_id: "Asia/Dubai",          display_city: "Abu Dhabi",      region_override: None },
    CityAlias { name: "jeddah",           iana_id: "Asia/Riyadh",         display_city: "Jeddah",         region_override: None },
    CityAlias { name: "tel aviv",         iana_id: "Asia/Jerusalem",      display_city: "Tel Aviv",       region_override: None },
    CityAlias { name: "lahore",           iana_id: "Asia/Karachi",        display_city: "Lahore",         region_override: None },
    CityAlias { name: "islamabad",        iana_id: "Asia/Karachi",        display_city: "Islamabad",      region_override: None },
    CityAlias { name: "hanoi",            iana_id: "Asia/Ho_Chi_Minh",    display_city: "Hanoi",          region_override: None },
    CityAlias { name: "bali",             iana_id: "Asia/Makassar",       display_city: "Bali",           region_override: None },

    // ── Africa — region from REGION_NAMES ──
    CityAlias { name: "cape town",        iana_id: "Africa/Johannesburg", display_city: "Cape Town",      region_override: None },
    CityAlias { name: "durban",           iana_id: "Africa/Johannesburg", display_city: "Durban",         region_override: None },
    CityAlias { name: "pretoria",         iana_id: "Africa/Johannesburg", display_city: "Pretoria",       region_override: None },
    CityAlias { name: "rabat",            iana_id: "Africa/Casablanca",   display_city: "Rabat",          region_override: None },
    CityAlias { name: "alexandria",       iana_id: "Africa/Cairo",        display_city: "Alexandria",     region_override: None },
    CityAlias { name: "abuja",            iana_id: "Africa/Lagos",        display_city: "Abuja",          region_override: None },

    // ── Oceania — overrides ──
    CityAlias { name: "canberra",         iana_id: "Australia/Sydney",    display_city: "Canberra",       region_override: Some("Australia, ACT") },

    // ── Oceania — region from REGION_NAMES ──
    CityAlias { name: "christchurch",     iana_id: "Pacific/Auckland",    display_city: "Christchurch",   region_override: None },
    CityAlias { name: "wellington",       iana_id: "Pacific/Auckland",    display_city: "Wellington",     region_override: None },
];

struct CityAlias {
    /// Lowercase search key (e.g. "san francisco").
    name: &'static str,
    /// IANA zone ID this alias maps to (e.g. "America/Los_Angeles").
    iana_id: &'static str,
    /// Pretty-printed city name shown in the UI.
    display_city: &'static str,
    /// If `Some`, overrides the region from `REGION_NAMES` — needed when the
    /// alias city is in a different state/province than the IANA reference city
    /// (e.g. Seattle is in Washington, not California). If `None`, region is
    /// resolved via `city_and_region(iana_id)`.
    region_override: Option<&'static str>,
}

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
    ("America/Guayaquil",      "Ecuador"),
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
    ("Europe/Tirane",          "Albania"),
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
    ("Asia/Makassar",          "Indonesia"),
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
        let region = alias
            .region_override
            .map(|r| r.to_string())
            .unwrap_or_else(|| city_and_region(alias.iana_id).1);
        return Some(ResolvedTz {
            tz,
            city: alias.display_city.to_string(),
            region,
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

// Source: IANA tzdata 2025b / zone1970.tab + supplementary Link zones
// for commonly-used aliases (e.g. Asia/Kuala_Lumpur, Africa/Accra).
// Sorted alphabetically by IANA zone name for binary search.
const ZONE_COUNTRY: &[(&str, &str)] = &[
    ("Africa/Abidjan", "CI"),
    ("Africa/Accra", "GH"),
    ("Africa/Addis_Ababa", "ET"),
    ("Africa/Algiers", "DZ"),
    ("Africa/Bissau", "GW"),
    ("Africa/Cairo", "EG"),
    ("Africa/Casablanca", "MA"),
    ("Africa/Ceuta", "ES"),
    ("Africa/El_Aaiun", "EH"),
    ("Africa/Johannesburg", "ZA"),
    ("Africa/Juba", "SS"),
    ("Africa/Khartoum", "SD"),
    ("Africa/Lagos", "NG"),
    ("Africa/Maputo", "MZ"),
    ("Africa/Monrovia", "LR"),
    ("Africa/Nairobi", "KE"),
    ("Africa/Ndjamena", "TD"),
    ("Africa/Sao_Tome", "ST"),
    ("Africa/Tripoli", "LY"),
    ("Africa/Tunis", "TN"),
    ("Africa/Windhoek", "NA"),
    ("America/Adak", "US"),
    ("America/Anchorage", "US"),
    ("America/Araguaina", "BR"),
    ("America/Argentina/Buenos_Aires", "AR"),
    ("America/Argentina/Catamarca", "AR"),
    ("America/Argentina/Cordoba", "AR"),
    ("America/Argentina/Jujuy", "AR"),
    ("America/Argentina/La_Rioja", "AR"),
    ("America/Argentina/Mendoza", "AR"),
    ("America/Argentina/Rio_Gallegos", "AR"),
    ("America/Argentina/Salta", "AR"),
    ("America/Argentina/San_Juan", "AR"),
    ("America/Argentina/San_Luis", "AR"),
    ("America/Argentina/Tucuman", "AR"),
    ("America/Argentina/Ushuaia", "AR"),
    ("America/Asuncion", "PY"),
    ("America/Bahia", "BR"),
    ("America/Bahia_Banderas", "MX"),
    ("America/Barbados", "BB"),
    ("America/Belem", "BR"),
    ("America/Belize", "BZ"),
    ("America/Boa_Vista", "BR"),
    ("America/Bogota", "CO"),
    ("America/Boise", "US"),
    ("America/Cambridge_Bay", "CA"),
    ("America/Campo_Grande", "BR"),
    ("America/Cancun", "MX"),
    ("America/Caracas", "VE"),
    ("America/Cayenne", "GF"),
    ("America/Chicago", "US"),
    ("America/Chihuahua", "MX"),
    ("America/Ciudad_Juarez", "MX"),
    ("America/Costa_Rica", "CR"),
    ("America/Coyhaique", "CL"),
    ("America/Cuiaba", "BR"),
    ("America/Danmarkshavn", "GL"),
    ("America/Dawson", "CA"),
    ("America/Dawson_Creek", "CA"),
    ("America/Denver", "US"),
    ("America/Detroit", "US"),
    ("America/Edmonton", "CA"),
    ("America/Eirunepe", "BR"),
    ("America/El_Salvador", "SV"),
    ("America/Fort_Nelson", "CA"),
    ("America/Fortaleza", "BR"),
    ("America/Glace_Bay", "CA"),
    ("America/Goose_Bay", "CA"),
    ("America/Grand_Turk", "TC"),
    ("America/Guatemala", "GT"),
    ("America/Guayaquil", "EC"),
    ("America/Guyana", "GY"),
    ("America/Halifax", "CA"),
    ("America/Havana", "CU"),
    ("America/Hermosillo", "MX"),
    ("America/Indiana/Indianapolis", "US"),
    ("America/Indiana/Knox", "US"),
    ("America/Indiana/Marengo", "US"),
    ("America/Indiana/Petersburg", "US"),
    ("America/Indiana/Tell_City", "US"),
    ("America/Indiana/Vevay", "US"),
    ("America/Indiana/Vincennes", "US"),
    ("America/Indiana/Winamac", "US"),
    ("America/Inuvik", "CA"),
    ("America/Iqaluit", "CA"),
    ("America/Jamaica", "JM"),
    ("America/Juneau", "US"),
    ("America/Kentucky/Louisville", "US"),
    ("America/Kentucky/Monticello", "US"),
    ("America/La_Paz", "BO"),
    ("America/Lima", "PE"),
    ("America/Los_Angeles", "US"),
    ("America/Maceio", "BR"),
    ("America/Managua", "NI"),
    ("America/Manaus", "BR"),
    ("America/Martinique", "MQ"),
    ("America/Matamoros", "MX"),
    ("America/Mazatlan", "MX"),
    ("America/Menominee", "US"),
    ("America/Merida", "MX"),
    ("America/Metlakatla", "US"),
    ("America/Mexico_City", "MX"),
    ("America/Miquelon", "PM"),
    ("America/Moncton", "CA"),
    ("America/Monterrey", "MX"),
    ("America/Montevideo", "UY"),
    ("America/New_York", "US"),
    ("America/Nome", "US"),
    ("America/Noronha", "BR"),
    ("America/North_Dakota/Beulah", "US"),
    ("America/North_Dakota/Center", "US"),
    ("America/North_Dakota/New_Salem", "US"),
    ("America/Nuuk", "GL"),
    ("America/Ojinaga", "MX"),
    ("America/Panama", "PA"),
    ("America/Paramaribo", "SR"),
    ("America/Phoenix", "US"),
    ("America/Port-au-Prince", "HT"),
    ("America/Porto_Velho", "BR"),
    ("America/Puerto_Rico", "PR"),
    ("America/Punta_Arenas", "CL"),
    ("America/Rankin_Inlet", "CA"),
    ("America/Recife", "BR"),
    ("America/Regina", "CA"),
    ("America/Resolute", "CA"),
    ("America/Rio_Branco", "BR"),
    ("America/Santarem", "BR"),
    ("America/Santiago", "CL"),
    ("America/Santo_Domingo", "DO"),
    ("America/Sao_Paulo", "BR"),
    ("America/Scoresbysund", "GL"),
    ("America/Sitka", "US"),
    ("America/St_Johns", "CA"),
    ("America/Swift_Current", "CA"),
    ("America/Tegucigalpa", "HN"),
    ("America/Thule", "GL"),
    ("America/Tijuana", "MX"),
    ("America/Toronto", "CA"),
    ("America/Vancouver", "CA"),
    ("America/Whitehorse", "CA"),
    ("America/Winnipeg", "CA"),
    ("America/Yakutat", "US"),
    ("Antarctica/Casey", "AQ"),
    ("Antarctica/Davis", "AQ"),
    ("Antarctica/Macquarie", "AU"),
    ("Antarctica/Mawson", "AQ"),
    ("Antarctica/Palmer", "AQ"),
    ("Antarctica/Rothera", "AQ"),
    ("Antarctica/Troll", "AQ"),
    ("Antarctica/Vostok", "AQ"),
    ("Asia/Almaty", "KZ"),
    ("Asia/Amman", "JO"),
    ("Asia/Anadyr", "RU"),
    ("Asia/Aqtau", "KZ"),
    ("Asia/Aqtobe", "KZ"),
    ("Asia/Ashgabat", "TM"),
    ("Asia/Atyrau", "KZ"),
    ("Asia/Baghdad", "IQ"),
    ("Asia/Baku", "AZ"),
    ("Asia/Bangkok", "TH"),
    ("Asia/Barnaul", "RU"),
    ("Asia/Beirut", "LB"),
    ("Asia/Bishkek", "KG"),
    ("Asia/Chita", "RU"),
    ("Asia/Colombo", "LK"),
    ("Asia/Damascus", "SY"),
    ("Asia/Dhaka", "BD"),
    ("Asia/Dili", "TL"),
    ("Asia/Dubai", "AE"),
    ("Asia/Dushanbe", "TJ"),
    ("Asia/Famagusta", "CY"),
    ("Asia/Gaza", "PS"),
    ("Asia/Hebron", "PS"),
    ("Asia/Ho_Chi_Minh", "VN"),
    ("Asia/Hong_Kong", "HK"),
    ("Asia/Hovd", "MN"),
    ("Asia/Irkutsk", "RU"),
    ("Asia/Jakarta", "ID"),
    ("Asia/Jayapura", "ID"),
    ("Asia/Jerusalem", "IL"),
    ("Asia/Kabul", "AF"),
    ("Asia/Kamchatka", "RU"),
    ("Asia/Karachi", "PK"),
    ("Asia/Kathmandu", "NP"),
    ("Asia/Khandyga", "RU"),
    ("Asia/Kolkata", "IN"),
    ("Asia/Krasnoyarsk", "RU"),
    ("Asia/Kuala_Lumpur", "MY"),
    ("Asia/Kuching", "MY"),
    ("Asia/Macau", "MO"),
    ("Asia/Magadan", "RU"),
    ("Asia/Makassar", "ID"),
    ("Asia/Manila", "PH"),
    ("Asia/Nicosia", "CY"),
    ("Asia/Novokuznetsk", "RU"),
    ("Asia/Novosibirsk", "RU"),
    ("Asia/Omsk", "RU"),
    ("Asia/Oral", "KZ"),
    ("Asia/Pontianak", "ID"),
    ("Asia/Pyongyang", "KP"),
    ("Asia/Qatar", "QA"),
    ("Asia/Qostanay", "KZ"),
    ("Asia/Qyzylorda", "KZ"),
    ("Asia/Riyadh", "SA"),
    ("Asia/Sakhalin", "RU"),
    ("Asia/Samarkand", "UZ"),
    ("Asia/Seoul", "KR"),
    ("Asia/Shanghai", "CN"),
    ("Asia/Singapore", "SG"),
    ("Asia/Srednekolymsk", "RU"),
    ("Asia/Taipei", "TW"),
    ("Asia/Tashkent", "UZ"),
    ("Asia/Tbilisi", "GE"),
    ("Asia/Tehran", "IR"),
    ("Asia/Thimphu", "BT"),
    ("Asia/Tokyo", "JP"),
    ("Asia/Tomsk", "RU"),
    ("Asia/Ulaanbaatar", "MN"),
    ("Asia/Urumqi", "CN"),
    ("Asia/Ust-Nera", "RU"),
    ("Asia/Vladivostok", "RU"),
    ("Asia/Yakutsk", "RU"),
    ("Asia/Yangon", "MM"),
    ("Asia/Yekaterinburg", "RU"),
    ("Asia/Yerevan", "AM"),
    ("Atlantic/Azores", "PT"),
    ("Atlantic/Bermuda", "BM"),
    ("Atlantic/Canary", "ES"),
    ("Atlantic/Cape_Verde", "CV"),
    ("Atlantic/Faroe", "FO"),
    ("Atlantic/Madeira", "PT"),
    ("Atlantic/Reykjavik", "IS"),
    ("Atlantic/South_Georgia", "GS"),
    ("Atlantic/Stanley", "FK"),
    ("Australia/Adelaide", "AU"),
    ("Australia/Brisbane", "AU"),
    ("Australia/Broken_Hill", "AU"),
    ("Australia/Darwin", "AU"),
    ("Australia/Eucla", "AU"),
    ("Australia/Hobart", "AU"),
    ("Australia/Lindeman", "AU"),
    ("Australia/Lord_Howe", "AU"),
    ("Australia/Melbourne", "AU"),
    ("Australia/Perth", "AU"),
    ("Australia/Sydney", "AU"),
    ("Europe/Amsterdam", "NL"),
    ("Europe/Andorra", "AD"),
    ("Europe/Astrakhan", "RU"),
    ("Europe/Athens", "GR"),
    ("Europe/Belgrade", "RS"),
    ("Europe/Berlin", "DE"),
    ("Europe/Brussels", "BE"),
    ("Europe/Bucharest", "RO"),
    ("Europe/Budapest", "HU"),
    ("Europe/Chisinau", "MD"),
    ("Europe/Copenhagen", "DK"),
    ("Europe/Dublin", "IE"),
    ("Europe/Gibraltar", "GI"),
    ("Europe/Helsinki", "FI"),
    ("Europe/Istanbul", "TR"),
    ("Europe/Kaliningrad", "RU"),
    ("Europe/Kirov", "RU"),
    ("Europe/Kyiv", "UA"),
    ("Europe/Lisbon", "PT"),
    ("Europe/London", "GB"),
    ("Europe/Madrid", "ES"),
    ("Europe/Malta", "MT"),
    ("Europe/Minsk", "BY"),
    ("Europe/Moscow", "RU"),
    ("Europe/Oslo", "NO"),
    ("Europe/Paris", "FR"),
    ("Europe/Prague", "CZ"),
    ("Europe/Riga", "LV"),
    ("Europe/Rome", "IT"),
    ("Europe/Samara", "RU"),
    ("Europe/Saratov", "RU"),
    ("Europe/Simferopol", "RU"),
    ("Europe/Sofia", "BG"),
    ("Europe/Stockholm", "SE"),
    ("Europe/Tallinn", "EE"),
    ("Europe/Tirane", "AL"),
    ("Europe/Ulyanovsk", "RU"),
    ("Europe/Vienna", "AT"),
    ("Europe/Vilnius", "LT"),
    ("Europe/Volgograd", "RU"),
    ("Europe/Warsaw", "PL"),
    ("Europe/Zagreb", "HR"),
    ("Europe/Zurich", "CH"),
    ("Indian/Chagos", "IO"),
    ("Indian/Maldives", "MV"),
    ("Indian/Mauritius", "MU"),
    ("Pacific/Apia", "WS"),
    ("Pacific/Auckland", "NZ"),
    ("Pacific/Bougainville", "PG"),
    ("Pacific/Chatham", "NZ"),
    ("Pacific/Easter", "CL"),
    ("Pacific/Efate", "VU"),
    ("Pacific/Fakaofo", "TK"),
    ("Pacific/Fiji", "FJ"),
    ("Pacific/Galapagos", "EC"),
    ("Pacific/Gambier", "PF"),
    ("Pacific/Guadalcanal", "SB"),
    ("Pacific/Guam", "GU"),
    ("Pacific/Honolulu", "US"),
    ("Pacific/Kanton", "KI"),
    ("Pacific/Kiritimati", "KI"),
    ("Pacific/Kosrae", "FM"),
    ("Pacific/Kwajalein", "MH"),
    ("Pacific/Marquesas", "PF"),
    ("Pacific/Nauru", "NR"),
    ("Pacific/Niue", "NU"),
    ("Pacific/Norfolk", "NF"),
    ("Pacific/Noumea", "NC"),
    ("Pacific/Pago_Pago", "AS"),
    ("Pacific/Palau", "PW"),
    ("Pacific/Pitcairn", "PN"),
    ("Pacific/Port_Moresby", "PG"),
    ("Pacific/Rarotonga", "CK"),
    ("Pacific/Tahiti", "PF"),
    ("Pacific/Tarawa", "KI"),
    ("Pacific/Tongatapu", "TO"),
];

// Source: Unicode CLDR v48 supplemental/timeData.json
// Territories where _preferred hour cycle is "h" (12-hour).
// Sorted alphabetically for binary search.
const TWELVE_HOUR_COUNTRIES: &[&str] = &[
    "AE", "AG", "AL", "AR", "AS", "AU", "BB", "BD", "BH", "BM",
    "BN", "BO", "BS", "BT", "CA", "CL", "CO", "CR", "CU", "CY",
    "DJ", "DM", "DO", "DZ", "EC", "EG", "EH", "ER", "ET", "FJ",
    "FM", "GD", "GH", "GM", "GR", "GT", "GU", "GY", "HK", "HN",
    "IN", "IQ", "JM", "JO", "KH", "KI", "KN", "KP", "KR", "KW",
    "KY", "LB", "LC", "LR", "LS", "LY", "MH", "MO", "MP", "MR",
    "MW", "MX", "MY", "NA", "NI", "NZ", "OM", "PA", "PE", "PG",
    "PH", "PK", "PR", "PS", "PW", "PY", "QA", "SA", "SB", "SD",
    "SG", "SL", "SO", "SS", "SV", "SY", "SZ", "TC", "TD", "TN",
    "TO", "TT", "TW", "UM", "US", "UY", "VC", "VE", "VG", "VI",
    "VU", "WS", "YE", "ZM",
];

pub fn uses_12h_clock(iana_id: &str) -> bool {
    ZONE_COUNTRY
        .binary_search_by_key(&iana_id, |(zone, _)| zone)
        .ok()
        .map(|i| TWELVE_HOUR_COUNTRIES.binary_search(&ZONE_COUNTRY[i].1).is_ok())
        .unwrap_or(false)
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

    // --- 12h/24h clock detection (CLDR-based) ---

    #[test]
    fn uses_12h_us_zone() {
        assert!(uses_12h_clock("America/Chicago"));
    }

    #[test]
    fn uses_24h_german_zone() {
        assert!(!uses_12h_clock("Europe/Berlin"));
    }

    #[test]
    fn uses_24h_gb_per_cldr() {
        assert!(!uses_12h_clock("Europe/London"));
    }

    #[test]
    fn uses_12h_australia() {
        assert!(uses_12h_clock("Australia/Sydney"));
    }

    #[test]
    fn uses_12h_south_korea() {
        assert!(uses_12h_clock("Asia/Seoul"));
    }

    #[test]
    fn uses_12h_indiana_via_zone_table() {
        assert!(uses_12h_clock("America/Indiana/Indianapolis"));
    }

    #[test]
    fn unknown_zone_defaults_to_24h() {
        assert!(!uses_12h_clock("Fake/Nowhere"));
    }

    #[test]
    fn uses_12h_multi_country_zone() {
        assert!(uses_12h_clock("Asia/Dubai"));
    }
}
