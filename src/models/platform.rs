use crate::utils::response_handler::timestamp_as_string;
use serde::Deserialize;

pub type PlatformResult = Vec<Platform>;

#[derive(Deserialize, Debug, PartialEq)]
pub struct Platform {
    pub id: usize,
    pub abbreviation: Option<String>,
    pub alternative_name: Option<String>,
    pub category: Option<u8>,
    pub checksum: Option<String>,
    pub created_at: Option<i64>,
    pub generation: Option<u8>,
    pub name: Option<String>,
    pub platform_family: Option<u8>,
    pub platform_logo: Option<u8>,
    pub slug: Option<String>,
    pub summary: Option<String>,
    pub updated_at: Option<i64>,
    pub url: Option<String>,
    pub versions: Option<Vec<usize>>,
    pub websites: Option<Vec<usize>>,
}

impl Platform {
    pub fn category(&self) -> String {
        match self.category {
            Some(c) => Category::new(c).translate(),
            _ => String::from("null"),
        }
    }

    pub fn created_at(&self) -> String {
        timestamp_as_string(self.created_at)
    }

    pub fn updated_at(&self) -> String {
        timestamp_as_string(self.updated_at)
    }
}

#[derive(Debug, Clone, Deserialize)]
enum Category {
    Console,
    Arcade,
    Platform,
    OperatingSystem,
    PortableConsole,
    Computer,
    Null,
}

impl Category {
    fn new(int: u8) -> Category {
        match int {
            1 => Category::Console,
            2 => Category::Arcade,
            3 => Category::Platform,
            4 => Category::OperatingSystem,
            5 => Category::PortableConsole,
            6 => Category::Computer,
            _ => Category::Null,
        }
    }

    pub fn translate(&self) -> String {
        match self {
            Category::Console => String::from("Console"),
            Category::Arcade => String::from("Arcade"),
            Category::Platform => String::from("Platform"),
            Category::OperatingSystem => String::from("OperatingSystem"),
            Category::PortableConsole => String::from("PortableConsole"),
            Category::Computer => String::from("Computer"),
            Category::Null => String::from("null"),
        }
    }
}
