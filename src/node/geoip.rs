use core::fmt::Display;
use std::net::IpAddr;
use std::sync::Arc;

use maxminddb::Reader;
use maxminddb::geoip2;
use memmap2::Mmap;
use tracing::error;
use tracing::info;

/// A [`Reader`] to GeoIP ASN and City databases.
pub struct GeoIpReader {
    asn_reader: Option<Arc<Reader<Mmap>>>,
    city_reader: Option<Arc<Reader<Mmap>>>,
}

impl GeoIpReader {
    pub fn new(asn_db_path: &str, city_db_path: &str) -> Result<Self, maxminddb::MaxMindDbError> {
        let asn_reader = match unsafe { Reader::open_mmap(asn_db_path) } {
            Ok(reader) => {
                info!("Loaded GeoLite2 ASN DB");
                Some(Arc::new(reader))
            }
            Err(e) => {
                error!("Failed to load GeoLite2 ASN DB: {e}");
                None
            }
        };

        let city_reader = match unsafe { Reader::open_mmap(city_db_path) } {
            Ok(reader) => {
                info!("Loaded GeoLite2 City DB");
                Some(Arc::new(reader))
            }
            Err(e) => {
                error!("Failed to load GeoLite2 City DB: {e}");
                None
            }
        };

        if asn_reader.is_none() && city_reader.is_none() {
            return Err(maxminddb::MaxMindDbError::invalid_database(
                "Failed to load any GeoIP databases",
            ));
        }

        Ok(Self {
            asn_reader,
            city_reader,
        })
    }

    pub fn lookup_asn(&self, ip: IpAddr) -> Option<AsnInfo> {
        let reader = self.asn_reader.as_ref()?;
        let lookup_result = reader.lookup(ip).ok()?;
        let asn = lookup_result.decode::<geoip2::Asn>().ok()??;

        Some(AsnInfo {
            number: asn.autonomous_system_number?,
            organization: asn.autonomous_system_organization?.to_string(),
        })
    }

    pub fn lookup_city(&self, ip: IpAddr) -> Option<CityInfo> {
        let reader = self.city_reader.as_ref()?;
        let lookup_result = reader.lookup(ip).ok()?;
        let city = lookup_result.decode::<geoip2::City>().ok()??;

        Some(CityInfo {
            city: city.city.names.english.map(|s| s.to_string()),
            country: city.country.names.english.map(|s| s.to_string()),
            country_code: city.country.iso_code.map(|s| s.to_string()),
        })
    }

    pub fn lookup_all(&self, ip: IpAddr) -> GeoInfo {
        GeoInfo {
            asn: self.lookup_asn(ip),
            city: self.lookup_city(ip),
        }
    }
}

impl Clone for GeoIpReader {
    fn clone(&self) -> Self {
        Self {
            asn_reader: self.asn_reader.clone(),
            city_reader: self.city_reader.clone(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct AsnInfo {
    pub number: u32,
    pub organization: String,
}

#[derive(Clone, Debug)]
pub struct CityInfo {
    pub city: Option<String>,
    pub country: Option<String>,
    pub country_code: Option<String>,
}

#[derive(Clone, Debug)]
pub struct GeoInfo {
    pub asn: Option<AsnInfo>,
    pub city: Option<CityInfo>,
}

impl Display for AsnInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AS{} - {}", self.number, self.organization)
    }
}

impl Display for CityInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.city, &self.country) {
            (Some(city), Some(country)) => write!(f, "{}, {}", city, country),
            (None, Some(country)) => write!(f, "{}", country),
            (Some(city), None) => write!(f, "{}", city),
            (None, None) => write!(f, "Unknown"),
        }
    }
}

impl Display for GeoInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.asn, &self.city) {
            (Some(asn), Some(city)) => write!(f, "{}\n{}", asn, city),
            (Some(asn), None) => write!(f, "{}", asn),
            (None, Some(city)) => write!(f, "{}", city),
            (None, None) => write!(f, "No information available"),
        }
    }
}
