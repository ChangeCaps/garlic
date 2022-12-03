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
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn set(&mut self, name: impl Into<String> + AsRef<str>, value: impl Into<String>) {
        for tag in &mut self.tags {
            if tag.name == name.as_ref() {
                tag.value = value.into();
                return;
            }
        }

        self.tags.push(StyleTag {
            name: name.into(),
            value: value.into(),
        });
    }
}

impl IntoPropValue<Option<IString>> for Style {
    #[inline]
    fn into_prop_value(self) -> Option<IString> {
        if self.tags.is_empty() {
            return None;
        }

        let string: String = self
            .tags
            .into_iter()
            .map(|tag| format!("{}:{};", tag.name, tag.value))
            .collect();

        Some(IString::from(string))
    }
}
