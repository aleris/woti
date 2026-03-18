use chrono_tz::Tz;

use crate::config::TimezoneEntry;
use crate::tz_data;

pub fn resolve(input: &str) -> Result<TimezoneEntry, String> {
    // 1. Try direct IANA match (contains '/')
    if input.contains('/') {
        return match input.parse::<Tz>() {
            Ok(tz) => {
                let (city, region) = tz_data::city_and_region(tz.name());
                Ok(TimezoneEntry {
                    iana_id: tz.name().to_string(),
                    city,
                    region,
                    is_default: false,
                })
            }
            Err(_) => Err(format!("Unknown IANA timezone: {input}")),
        };
    }

    // 2. Try abbreviation lookup (dynamically built from chrono-tz)
    if let Some(r) = tz_data::lookup_abbreviation(input) {
        return Ok(TimezoneEntry {
            iana_id: r.tz.name().to_string(),
            city: r.city,
            region: r.region,
            is_default: false,
        });
    }

    // 3. Try city name lookup (aliases + TZ_VARIANTS scan)
    if let Some(r) = tz_data::lookup_city(input) {
        return Ok(TimezoneEntry {
            iana_id: r.tz.name().to_string(),
            city: r.city,
            region: r.region,
            is_default: false,
        });
    }

    Err(format!(
        "Could not resolve '{input}'. Try a timezone abbreviation (PST), \
         city name (Tokyo), or IANA identifier (America/New_York)."
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Spec: "Add timezone by abbreviation" ---

    #[test]
    fn resolve_common_abbreviation_pst() {
        let entry = resolve("PST").expect("PST should resolve");
        assert_eq!(entry.iana_id, "America/Los_Angeles");
        assert!(!entry.is_default);
    }

    #[test]
    fn resolve_ambiguous_abbreviation_cst() {
        let entry = resolve("CST").expect("CST should resolve");
        assert_eq!(entry.iana_id, "America/Chicago");
    }

    #[test]
    fn resolve_unrecognized_abbreviation() {
        let err = resolve("XYZ").unwrap_err();
        assert!(err.contains("Could not resolve"));
    }

    // --- Spec: "Add timezone by city name" ---

    #[test]
    fn resolve_city_bucharest() {
        let entry = resolve("Bucharest").expect("Bucharest should resolve");
        assert_eq!(entry.iana_id, "Europe/Bucharest");
        assert_eq!(entry.region, "Romania");
    }

    #[test]
    fn resolve_city_san_jose() {
        let entry = resolve("San Jose").expect("San Jose should resolve");
        assert_eq!(entry.iana_id, "America/Los_Angeles");
        assert_eq!(entry.city, "San Jose");
        assert_eq!(entry.region, "United States, California");
    }

    #[test]
    fn resolve_unknown_city() {
        let err = resolve("Atlantis").unwrap_err();
        assert!(err.contains("Could not resolve"));
    }

    // --- Spec: "Add timezone by IANA identifier" ---

    #[test]
    fn resolve_valid_iana() {
        let entry = resolve("America/New_York").expect("valid IANA should resolve");
        assert_eq!(entry.iana_id, "America/New_York");
        assert_eq!(entry.city, "New York");
        assert_eq!(entry.region, "United States, New York");
    }

    #[test]
    fn resolve_invalid_iana() {
        let err = resolve("Invalid/Nowhere").unwrap_err();
        assert!(err.contains("Unknown IANA timezone"));
    }

    // --- Additional resolution coverage ---

    #[test]
    fn resolve_iana_uses_region_override() {
        let entry = resolve("Europe/Bucharest").expect("should resolve");
        assert_eq!(entry.region, "Romania");
    }

    #[test]
    fn resolve_city_alias_mumbai() {
        let entry = resolve("Mumbai").expect("Mumbai should resolve");
        assert_eq!(entry.iana_id, "Asia/Kolkata");
        assert_eq!(entry.city, "Mumbai");
    }

    #[test]
    fn resolve_iana_city_via_tz_variants() {
        let entry = resolve("Zurich").expect("Zurich should resolve");
        assert_eq!(entry.iana_id, "Europe/Zurich");
        assert_eq!(entry.region, "Switzerland");
    }
}
