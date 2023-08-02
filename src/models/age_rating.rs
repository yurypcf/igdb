use num_enum::TryFromPrimitive;
use serde::Deserialize;

pub type AgeRatingResult = Vec<AgeRating>;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct AgeRating {
    pub id: u64,
    pub category: Option<i32>,
    pub content_descriptions: Option<Vec<u64>>,
    pub rating: Option<i32>,
    pub rating_cover_url: Option<String>,
    pub synopsis: Option<String>,
    pub checksum: String,
}

impl AgeRating {
    pub fn category(&self) -> &'static str {
        Category::as_int(self.category).as_str_name()
    }

    pub fn rating(&self) -> &'static str {
        Rating::as_int(self.rating).as_str_name()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, TryFromPrimitive)]
#[repr(i32)]
pub enum Category {
    Null = 0,
    Esrb = 1,
    Pegi = 2,
    Cero = 3,
    Usk = 4,
    Grac = 5,
    ClassInd = 6,
    Acb = 7,
}

impl Category {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Category::Null => "CATEGORY_NULL",
            Category::Esrb => "ESRB",
            Category::Pegi => "PEGI",
            Category::Cero => "CERO",
            Category::Usk => "USK",
            Category::Grac => "GRAC",
            Category::ClassInd => "CLASS_IND",
            Category::Acb => "ACB",
        }
    }

    fn as_int(value: Option<i32>) -> Self {
        match value {
            Some(num) => Self::try_from(num).unwrap(),
            None => Category::Null,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, TryFromPrimitive)]
#[repr(i32)]
pub enum Rating {
    Null = 0,
    Three = 1,
    Seven = 2,
    Twelve = 3,
    Sixteen = 4,
    Eighteen = 5,
    Rp = 6,
    Ec = 7,
    E = 8,
    E10 = 9,
    T = 10,
    M = 11,
    Ao = 12,
    CeroA = 13,
    CeroB = 14,
    CeroC = 15,
    CeroD = 16,
    CeroZ = 17,
    Usk0 = 18,
    Usk6 = 19,
    Usk12 = 20,
    Usk16 = 21,
    Usk18 = 22,
    GracAll = 23,
    GracTwelve = 24,
    GracFifteen = 25,
    GracEighteen = 26,
    GracTesting = 27,
    ClassIndL = 28,
    ClassIndTen = 29,
    ClassIndTwelve = 30,
    ClassIndFourteen = 31,
    ClassIndSixteen = 32,
    ClassIndEighteen = 33,
    AcbG = 34,
    AcbPg = 35,
    AcbM = 36,
    AcbMa15 = 37,
    AcbR18 = 38,
    AcbRc = 39,
}

impl Rating {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Rating::Null => "RATING_NULL",
            Rating::Three => "THREE",
            Rating::Seven => "SEVEN",
            Rating::Twelve => "TWELVE",
            Rating::Sixteen => "SIXTEEN",
            Rating::Eighteen => "EIGHTEEN",
            Rating::Rp => "RP",
            Rating::Ec => "EC",
            Rating::E => "E",
            Rating::E10 => "E10",
            Rating::T => "T",
            Rating::M => "M",
            Rating::Ao => "AO",
            Rating::CeroA => "CERO_A",
            Rating::CeroB => "CERO_B",
            Rating::CeroC => "CERO_C",
            Rating::CeroD => "CERO_D",
            Rating::CeroZ => "CERO_Z",
            Rating::Usk0 => "USK_0",
            Rating::Usk6 => "USK_6",
            Rating::Usk12 => "USK_12",
            Rating::Usk16 => "USK_16",
            Rating::Usk18 => "USK_18",
            Rating::GracAll => "GRAC_ALL",
            Rating::GracTwelve => "GRAC_TWELVE",
            Rating::GracFifteen => "GRAC_FIFTEEN",
            Rating::GracEighteen => "GRAC_EIGHTEEN",
            Rating::GracTesting => "GRAC_TESTING",
            Rating::ClassIndL => "CLASS_IND_L",
            Rating::ClassIndTen => "CLASS_IND_TEN",
            Rating::ClassIndTwelve => "CLASS_IND_TWELVE",
            Rating::ClassIndFourteen => "CLASS_IND_FOURTEEN",
            Rating::ClassIndSixteen => "CLASS_IND_SIXTEEN",
            Rating::ClassIndEighteen => "CLASS_IND_EIGHTEEN",
            Rating::AcbG => "ACB_G",
            Rating::AcbPg => "ACB_PG",
            Rating::AcbM => "ACB_M",
            Rating::AcbMa15 => "ACB_MA15",
            Rating::AcbR18 => "ACB_R18",
            Rating::AcbRc => "ACB_RC",
        }
    }

    fn as_int(value: Option<i32>) -> Self {
        match value {
            Some(num) => Self::try_from(num).unwrap(),
            None => Rating::Null,
        }
    }
}
