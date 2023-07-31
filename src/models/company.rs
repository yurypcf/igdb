use crate::utils::response_handler::timestamp_as_string;
use serde::Deserialize;
use num_enum::TryFromPrimitive;

pub type CompanyResult = Vec<Company>;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Company {
    pub id: u64,
    pub change_date: Option<i64>,
    pub change_date_category: Option<i32>,
    pub changed_company_id: Option<u64>,
    pub country: Option<i32>,
    pub created_at: Option<i64>,
    pub description: Option<String>,
    pub developed: Option<Vec<u64>>,
    pub logo: Option<u64>,
    pub name: String,
    pub parent: Option<u64>,
    pub published: Option<Vec<u64>>,
    pub slug: String,
    pub start_date: Option<i64>,
    pub start_date_category: Option<i32>,
    pub updated_at: Option<i64>,
    pub url: String,
    pub websites: Option<Vec<u64>>,
    pub checksum: String,
}

impl Company {
  pub fn change_date_category(&self) -> &'static str {
    DateFormatChangeDateCategoryEnum::as_int(self.change_date_category).as_str_name()
  }

  pub fn start_date_category(&self) -> &'static str {
    DateFormatChangeDateCategoryEnum::as_int(self.start_date_category).as_str_name()
  }

  pub fn created_at(&self) -> String {
    timestamp_as_string(self.created_at)
  }

  pub fn updated_at(&self) -> String {
      timestamp_as_string(self.updated_at)
  }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, TryFromPrimitive)]
#[repr(i32)]
pub enum DateFormatChangeDateCategoryEnum {
    Yyyymmmmdd = 0,
    Yyyymmmm = 1,
    Yyyy = 2,
    Yyyyq1 = 3,
    Yyyyq2 = 4,
    Yyyyq3 = 5,
    Yyyyq4 = 6,
    Tbd = 7,
    Null,
}

impl DateFormatChangeDateCategoryEnum {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DateFormatChangeDateCategoryEnum::Yyyymmmmdd => "YYYYMMMMDD",
            DateFormatChangeDateCategoryEnum::Yyyymmmm => "YYYYMMMM",
            DateFormatChangeDateCategoryEnum::Yyyy => "YYYY",
            DateFormatChangeDateCategoryEnum::Yyyyq1 => "YYYYQ1",
            DateFormatChangeDateCategoryEnum::Yyyyq2 => "YYYYQ2",
            DateFormatChangeDateCategoryEnum::Yyyyq3 => "YYYYQ3",
            DateFormatChangeDateCategoryEnum::Yyyyq4 => "YYYYQ4",
            DateFormatChangeDateCategoryEnum::Tbd => "TBD",
            _ => "Null",
        }
    }

    fn as_int(value: Option<i32>) -> Self {
      match value {
        Some(num) => Self::try_from(num).unwrap(),
        None => DateFormatChangeDateCategoryEnum::Null,
      }
    }
}