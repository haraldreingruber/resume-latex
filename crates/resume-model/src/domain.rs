//! Normalized domain model: validated, plain Unicode text, parsed dates and
//! rich text spans. All renderers consume these types.

use std::fmt;

use serde::{Deserialize, Serialize};

/// Separator used between a date range's start and end, and between a
/// project's type and entity.
pub const DASH_SEPARATOR: &str = " – ";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Resume {
    pub basics: Basics,
    pub work: Vec<Work>,
    pub projects: Vec<Project>,
    pub skills: Vec<SkillGroup>,
    pub education: Vec<Education>,
    pub languages: Vec<Language>,
}

impl Resume {
    pub fn skill_group(&self, id: &str) -> Option<&SkillGroup> {
        self.skills.iter().find(|group| group.id == id)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Basics {
    pub name: String,
    pub label: String,
    pub email: String,
    pub location: Location,
    pub profiles: Vec<Profile>,
    pub summary: RichText,
}

impl Basics {
    /// Case-insensitive lookup by network name, e.g. `"github"`.
    pub fn profile(&self, network: &str) -> Option<&Profile> {
        self.profiles
            .iter()
            .find(|profile| profile.network.eq_ignore_ascii_case(network))
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Location {
    pub city: String,
    pub region: Option<String>,
    /// Country name, resolved from the ISO country code.
    pub country: Option<String>,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parts: Vec<&str> = [
            Some(&self.city),
            self.region.as_ref(),
            self.country.as_ref(),
        ]
        .into_iter()
        .flatten()
        .map(String::as_str)
        .collect();
        f.write_str(&parts.join(", "))
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Profile {
    pub network: String,
    pub username: String,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Work {
    pub id: String,
    pub position: String,
    pub organization: String,
    pub location: Option<String>,
    pub dates: DateRange,
    /// Condensed one-paragraph version of the highlights.
    pub summary: Option<RichText>,
    pub highlights: Vec<RichText>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    /// E.g. "Master's Thesis".
    pub kind: Option<String>,
    /// Organization the project was done at.
    pub entity: Option<String>,
    pub title: String,
    pub description: Option<RichText>,
    pub location: Option<String>,
    pub dates: DateRange,
}

impl Project {
    /// "Master's Thesis – Austrian Institute of Technology"; falls back to the title.
    pub fn heading(&self) -> String {
        let parts: Vec<&str> = [self.kind.as_deref(), self.entity.as_deref()]
            .into_iter()
            .flatten()
            .collect();
        if parts.is_empty() {
            self.title.clone()
        } else {
            parts.join(DASH_SEPARATOR)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SkillGroup {
    pub id: String,
    pub name: String,
    pub keywords: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Education {
    pub id: String,
    /// E.g. "MSc".
    pub study_type: Option<String>,
    /// E.g. "Computer Science, Visual Computing".
    pub area: Option<String>,
    pub institution: String,
    pub location: Option<String>,
    pub dates: DateRange,
    pub courses: Vec<String>,
}

impl Education {
    /// "MSc Computer Science, Visual Computing"; falls back to the institution.
    pub fn title(&self) -> String {
        let parts: Vec<&str> = [self.study_type.as_deref(), self.area.as_deref()]
            .into_iter()
            .flatten()
            .collect();
        if parts.is_empty() {
            self.institution.clone()
        } else {
            parts.join(" ")
        }
    }

    /// "Image processing, real-time graphics." or `None` without courses.
    pub fn courses_sentence(&self) -> Option<String> {
        (!self.courses.is_empty()).then(|| format!("{}.", self.courses.join(", ")))
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Language {
    pub language: String,
    pub fluency: Option<String>,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.fluency {
            Some(fluency) => write!(f, "{} ({fluency})", self.language),
            None => f.write_str(&self.language),
        }
    }
}

/// A date with year or month precision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PartialDate {
    pub year: u16,
    pub month: Option<u8>,
}

impl fmt::Display for PartialDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.month {
            Some(month) => write!(f, "{month:02}/{}", self.year),
            None => write!(f, "{}", self.year),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DateRange {
    pub start: PartialDate,
    /// `None` means ongoing.
    pub end: Option<PartialDate>,
}

impl fmt::Display for DateRange {
    /// "11/2020 – Present", "2007 – 2011", or "2009" when start equals end.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.end {
            None => write!(f, "{}{DASH_SEPARATOR}Present", self.start),
            Some(end) if end == self.start => write!(f, "{}", self.start),
            Some(end) => write!(f, "{}{DASH_SEPARATOR}{end}", self.start),
        }
    }
}

/// Inline-formatted text: a sequence of styled spans.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RichText(pub Vec<Span>);

impl RichText {
    pub fn plain(&self) -> String {
        self.0.iter().map(|span| span.text.as_str()).collect()
    }

    pub fn spans(&self) -> &[Span] {
        &self.0
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Span {
    pub text: String,
    #[serde(default)]
    pub style: Style,
    pub link: Option<String>,
}

impl Span {
    pub fn plain(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            ..Self::default()
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Style {
    pub bold: bool,
    pub italic: bool,
    pub code: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(year: u16, month: Option<u8>) -> PartialDate {
        PartialDate { year, month }
    }

    #[test]
    fn date_range_display() {
        let ongoing = DateRange {
            start: date(2020, Some(11)),
            end: None,
        };
        assert_eq!(ongoing.to_string(), "11/2020 – Present");

        let years = DateRange {
            start: date(2007, None),
            end: Some(date(2011, None)),
        };
        assert_eq!(years.to_string(), "2007 – 2011");

        let single = DateRange {
            start: date(2009, None),
            end: Some(date(2009, None)),
        };
        assert_eq!(single.to_string(), "2009");

        let months = DateRange {
            start: date(2009, Some(2)),
            end: Some(date(2009, Some(7))),
        };
        assert_eq!(months.to_string(), "02/2009 – 07/2009");
    }

    #[test]
    fn location_display_skips_missing_parts() {
        let full = Location {
            city: "Scharnstein".into(),
            region: Some("Upper Austria".into()),
            country: Some("Austria".into()),
        };
        assert_eq!(full.to_string(), "Scharnstein, Upper Austria, Austria");

        let city_only = Location {
            city: "Vienna".into(),
            region: None,
            country: None,
        };
        assert_eq!(city_only.to_string(), "Vienna");
    }
}
