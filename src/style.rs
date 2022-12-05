use std::ops::{Add, AddAssign};

use implicit_clone::unsync::IString;
use smallvec::SmallVec;
use yew::html::IntoPropValue;

#[derive(Clone, Debug, Default)]
pub struct StyleTag {
    pub name: String,
    pub value: String,
}

impl StyleTag {
    #[inline]
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Style {
    pub tags: SmallVec<[StyleTag; 8]>,
}

impl Style {
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn parse(&mut self, style: impl AsRef<str>) {
        let mut style = style.as_ref().split(';');

        while let Some(tag) = style.next() {
            let mut tag = tag.split(':');
            let name = tag.next().unwrap_or_default().trim();
            let value = tag.next().unwrap_or_default().trim();

            if !name.is_empty() && !value.is_empty() {
                self.set(name, value);
            }
        }
    }

    #[inline]
    pub fn set(
        &mut self,
        name: impl Into<String> + AsRef<str>,
        value: impl Into<String>,
    ) -> &mut Self {
        for tag in &mut self.tags {
            if tag.name == name.as_ref() {
                tag.value = value.into();
                return self;
            }
        }

        self.tags.push(StyleTag {
            name: name.into(),
            value: value.into(),
        });

        self
    }

    #[inline]
    pub fn with(mut self, name: impl Into<String> + AsRef<str>, value: impl Into<String>) -> Self {
        self.set(name, value);
        self
    }

    #[inline]
    pub fn to_string(&self) -> String {
        let mut style = String::new();

        for tag in &self.tags {
            style.push_str(&tag.name);
            style.push(':');
            style.push_str(&tag.value);
            style.push(';');
        }

        style
    }
}

impl IntoPropValue<Option<IString>> for Style {
    fn into_prop_value(self) -> Option<IString> {
        if self.tags.is_empty() {
            return None;
        }

        Some(IString::from(self.to_string()))
    }
}

impl IntoPropValue<String> for Style {
    fn into_prop_value(self) -> String {
        self.to_string()
    }
}

impl Add<Style> for Style {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self {
        self.tags.extend(rhs.tags);
        self
    }
}

impl AddAssign<Style> for Style {
    fn add_assign(&mut self, other: Style) {
        for tag in other.tags {
            self.set(tag.name, tag.value);
        }
    }
}
