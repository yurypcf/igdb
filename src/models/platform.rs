use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
pub struct Platform {
    pub id: usize,
    pub abbreviation: Option<String>,
    pub alternative_name: Option<String>,
    pub category: Option<u8>,
    pub checksum: Option<String>,
    pub created_at: Option<usize>,
    pub generation: Option<u8>,
    pub name: Option<String>,
    pub platform_family: Option<u8>,
    pub platform_logo: Option<u8>,
    pub slug: Option<String>,
    pub summary: Option<String>,
    pub updated_at: Option<usize>,
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
