use crate::api::File;
use crate::{
    imp::{
        core::*,
        element_handle::SetInputFilesArgs,
        locator::{
            CheckArgs, ClearArgs, ClickArgs, FillArgs, FilterOptions, HoverArgs,
            Locator as LocatorImpl, PressArgs, SelectOptionArgs, TypeArgs,
        },
        prelude::*,
        utils::{KeyboardModifier, MouseButton, Position},
    },
    Error,
};

/// Locators are the central piece of Playwright's auto-waiting and retry-ability.
/// In a nutshell, locators represent a way to find element(s) on the page at any moment.
/// Locators are created with the page.locator() method.
#[derive(Debug, Clone)]
pub struct Locator {
    inner: Weak<LocatorImpl>,
}

impl PartialEq for Locator {
    fn eq(&self, other: &Self) -> bool {
        let a = self.inner.upgrade();
        let b = other.inner.upgrade();
        a.and_then(|a| b.map(|b| (a, b)))
            .map(|(a, b)| a.guid() == b.guid())
            .unwrap_or_default()
    }
}

impl Locator {
    pub(crate) fn new(inner: Weak<LocatorImpl>) -> Self {
        Self { inner }
    }

    /// Returns the locator selector.
    pub fn selector(&self) -> Result<String, Error> {
        Ok(upgrade(&self.inner)?.selector().to_string())
    }

    // Action methods

    /// Click an element.
    pub fn click_builder(&self) -> LocatorClickBuilder {
        LocatorClickBuilder::new(self.inner.clone())
    }

    /// Double-click an element.
    pub fn dblclick_builder(&self) -> LocatorDblClickBuilder {
        LocatorDblClickBuilder::new(self.inner.clone())
    }

    /// Fill a form control.
    pub fn fill_builder<'a>(&self, value: &'a str) -> LocatorFillBuilder<'a> {
        LocatorFillBuilder::new(self.inner.clone(), value)
    }

    /// Hover over an element.
    pub fn hover_builder(&self) -> LocatorHoverBuilder {
        LocatorHoverBuilder::new(self.inner.clone())
    }

    /// Check a checkbox or radio button.
    pub fn check_builder(&self) -> LocatorCheckBuilder {
        LocatorCheckBuilder::new(self.inner.clone())
    }

    /// Uncheck a checkbox or radio button.
    pub fn uncheck_builder(&self) -> LocatorUncheckBuilder {
        LocatorUncheckBuilder::new(self.inner.clone())
    }

    /// Press a key.
    pub fn press_builder<'a>(&self, key: &'a str) -> LocatorPressBuilder<'a> {
        LocatorPressBuilder::new(self.inner.clone(), key)
    }

    /// Set files for file input upload.
    pub fn set_input_files_builder(&self, file: File) -> LocatorSetInputFilesBuilder {
        LocatorSetInputFilesBuilder::new(self.inner.clone(), file)
    }

    /// Focus on the element.
    pub async fn focus(&self, timeout: Option<f64>) -> Result<(), Error> {
        upgrade(&self.inner)?
            .focus(timeout)
            .await
            .map_err(|_| Error::ObjectNotFound)
    }

    /// Remove focus from the element.
    pub async fn blur(&self, timeout: Option<f64>) -> Result<(), Error> {
        upgrade(&self.inner)?
            .blur(timeout)
            .await
            .map_err(|_| Error::ObjectNotFound)
    }

    /// Clear the input field.
    pub fn clear_builder(&self) -> LocatorClearBuilder {
        LocatorClearBuilder::new(self.inner.clone())
    }

    /// Type text into the element.
    pub fn type_builder<'a>(&self, text: &'a str) -> LocatorTypeBuilder<'a> {
        LocatorTypeBuilder::new(self.inner.clone(), text)
    }

    /// Select option(s) from a `<select>` element.
    pub fn select_option_builder(&self) -> LocatorSelectOptionBuilder {
        LocatorSelectOptionBuilder::new(self.inner.clone())
    }

    // Query methods

    /// Get the text content of the element.
    pub async fn text_content(&self, timeout: Option<f64>) -> Result<Option<String>, Error> {
        upgrade(&self.inner)?
            .text_content(timeout)
            .await
            .map_err(|_e| Error::ObjectNotFound)
    }

    /// Get the inner text of the element.
    pub async fn inner_text(&self, timeout: Option<f64>) -> Result<String, Error> {
        upgrade(&self.inner)?
            .inner_text(timeout)
            .await
            .map_err(|_| Error::ObjectNotFound)
    }

    /// Get the inner HTML of the element.
    pub async fn inner_html(&self, timeout: Option<f64>) -> Result<String, Error> {
        upgrade(&self.inner)?
            .inner_html(timeout)
            .await
            .map_err(|_| Error::ObjectNotFound)
    }

    /// Get an attribute value.
    pub async fn get_attribute(
        &self,
        name: &str,
        timeout: Option<f64>,
    ) -> Result<Option<String>, Error> {
        upgrade(&self.inner)?
            .get_attribute(name, timeout)
            .await
            .map_err(|_| Error::ObjectNotFound)
    }

    /// Get the input value (for form controls).
    pub async fn input_value(&self, timeout: Option<f64>) -> Result<String, Error> {
        upgrade(&self.inner)?
            .input_value(timeout)
            .await
            .map_err(|_| Error::ObjectNotFound)
    }

    /// Get the count of matching elements.
    pub async fn count(&self) -> Result<usize, Error> {
        upgrade(&self.inner)?
            .count()
            .await
            .map_err(|_| Error::ObjectNotFound)
    }

    // State methods

    /// Check if the element is visible.
    pub async fn is_visible(&self, timeout: Option<f64>) -> Result<bool, Error> {
        upgrade(&self.inner)?
            .is_visible(timeout)
            .await
            .map_err(|_| Error::ObjectNotFound)
    }

    /// Check if the element is hidden.
    pub async fn is_hidden(&self, timeout: Option<f64>) -> Result<bool, Error> {
        upgrade(&self.inner)?
            .is_hidden(timeout)
            .await
            .map_err(|_| Error::ObjectNotFound)
    }

    /// Check if the element is enabled.
    pub async fn is_enabled(&self, timeout: Option<f64>) -> Result<bool, Error> {
        upgrade(&self.inner)?
            .is_enabled(timeout)
            .await
            .map_err(|_| Error::ObjectNotFound)
    }

    /// Check if the element is disabled.
    pub async fn is_disabled(&self, timeout: Option<f64>) -> Result<bool, Error> {
        upgrade(&self.inner)?
            .is_disabled(timeout)
            .await
            .map_err(|_| Error::ObjectNotFound)
    }

    /// Check if the element is checked.
    pub async fn is_checked(&self, timeout: Option<f64>) -> Result<bool, Error> {
        upgrade(&self.inner)?
            .is_checked(timeout)
            .await
            .map_err(|_| Error::ObjectNotFound)
    }

    /// Check if the element is editable.
    pub async fn is_editable(&self, timeout: Option<f64>) -> Result<bool, Error> {
        upgrade(&self.inner)?
            .is_editable(timeout)
            .await
            .map_err(|_| Error::ObjectNotFound)
    }

    // Chaining methods

    /// Select the first matching element.
    pub async fn first(&self) -> Result<Locator, Error> {
        upgrade(&self.inner)?
            .first()
            .await
            .map(Locator::new)
            .map_err(|_| Error::ObjectNotFound)
    }

    /// Select the last matching element.
    pub async fn last(&self) -> Result<Locator, Error> {
        upgrade(&self.inner)?
            .last()
            .await
            .map(Locator::new)
            .map_err(|_| Error::ObjectNotFound)
    }

    /// Select the nth matching element.
    pub async fn nth(&self, index: i32) -> Result<Locator, Error> {
        upgrade(&self.inner)?
            .nth(index)
            .await
            .map(Locator::new)
            .map_err(|_| Error::ObjectNotFound)
    }

    /// Filter the locator to match only elements that meet certain criteria.
    pub fn filter_builder(&self) -> LocatorFilterBuilder {
        LocatorFilterBuilder::new(self.inner.clone())
    }
}

// Builder implementations

pub struct LocatorClickBuilder {
    inner: Weak<LocatorImpl>,
    args: ClickArgs,
}

impl LocatorClickBuilder {
    pub(crate) fn new(inner: Weak<LocatorImpl>) -> Self {
        Self {
            inner,
            args: ClickArgs::default(),
        }
    }

    pub async fn click(self) -> Result<(), Error> {
        let Self { inner, args } = self;
        upgrade(&inner)?
            .click(args)
            .await
            .map_err(|_| Error::ObjectNotFound)
    }

    setter! {
        /// Mouse button to click
        button: Option<MouseButton>,
        /// Number of times to click
        click_count: Option<i32>,
        /// Time to wait between mousedown and mouseup
        delay: Option<f64>,
        /// Position relative to the element's bounding box
        position: Option<Position>,
        /// Keyboard modifiers to press
        modifiers: Option<Vec<KeyboardModifier>>,
        /// Whether to bypass actionability checks
        force: Option<bool>,
        /// Whether to skip waiting after the action
        no_wait_after: Option<bool>,
        /// Maximum time to wait for the action
        timeout: Option<f64>
    }
}

pub struct LocatorFillBuilder<'a> {
    inner: Weak<LocatorImpl>,
    value: &'a str,
    args: FillArgs,
}

impl<'a> LocatorFillBuilder<'a> {
    pub(crate) fn new(inner: Weak<LocatorImpl>, value: &'a str) -> Self {
        Self {
            inner,
            value,
            args: FillArgs::default(),
        }
    }

    pub async fn fill(self) -> Result<(), Error> {
        let Self { inner, value, args } = self;
        upgrade(&inner)?
            .fill(value, args)
            .await
            .map_err(|_| Error::ObjectNotFound)
    }

    setter! {
        /// Whether to bypass actionability checks
        force: Option<bool>,
        /// Whether to skip waiting after the action
        no_wait_after: Option<bool>,
        /// Maximum time to wait for the action
        timeout: Option<f64>
    }
}

pub struct LocatorHoverBuilder {
    inner: Weak<LocatorImpl>,
    args: HoverArgs,
}

impl LocatorHoverBuilder {
    pub(crate) fn new(inner: Weak<LocatorImpl>) -> Self {
        Self {
            inner,
            args: HoverArgs::default(),
        }
    }

    pub async fn hover(self) -> Result<(), Error> {
        let Self { inner, args } = self;
        upgrade(&inner)?
            .hover(args)
            .await
            .map_err(|_| Error::ObjectNotFound)
    }

    setter! {
        /// Position relative to the element's bounding box
        position: Option<Position>,
        /// Keyboard modifiers to press
        modifiers: Option<Vec<KeyboardModifier>>,
        /// Whether to bypass actionability checks
        force: Option<bool>,
        /// Maximum time to wait for the action
        timeout: Option<f64>
    }
}

pub struct LocatorCheckBuilder {
    inner: Weak<LocatorImpl>,
    args: CheckArgs,
}

impl LocatorCheckBuilder {
    pub(crate) fn new(inner: Weak<LocatorImpl>) -> Self {
        Self {
            inner,
            args: CheckArgs::default(),
        }
    }

    pub async fn check(self) -> Result<(), Error> {
        let Self { inner, args } = self;
        upgrade(&inner)?
            .check(args)
            .await
            .map_err(|_| Error::ObjectNotFound)
    }

    setter! {
        /// Position relative to the element's bounding box
        position: Option<Position>,
        /// Whether to bypass actionability checks
        force: Option<bool>,
        /// Whether to skip waiting after the action
        no_wait_after: Option<bool>,
        /// Maximum time to wait for the action
        timeout: Option<f64>
    }
}

pub struct LocatorUncheckBuilder {
    inner: Weak<LocatorImpl>,
    args: CheckArgs,
}

impl LocatorUncheckBuilder {
    pub(crate) fn new(inner: Weak<LocatorImpl>) -> Self {
        Self {
            inner,
            args: CheckArgs::default(),
        }
    }

    pub async fn uncheck(self) -> Result<(), Error> {
        let Self { inner, args } = self;
        upgrade(&inner)?
            .uncheck(args)
            .await
            .map_err(|_| Error::ObjectNotFound)
    }

    setter! {
        /// Position relative to the element's bounding box
        position: Option<Position>,
        /// Whether to bypass actionability checks
        force: Option<bool>,
        /// Whether to skip waiting after the action
        no_wait_after: Option<bool>,
        /// Maximum time to wait for the action
        timeout: Option<f64>
    }
}

pub struct LocatorPressBuilder<'a> {
    inner: Weak<LocatorImpl>,
    key: &'a str,
    args: PressArgs,
}

impl<'a> LocatorPressBuilder<'a> {
    pub(crate) fn new(inner: Weak<LocatorImpl>, key: &'a str) -> Self {
        Self {
            inner,
            key,
            args: PressArgs::default(),
        }
    }

    pub async fn press(self) -> Result<(), Error> {
        let Self { inner, key, args } = self;
        upgrade(&inner)?
            .press(key, args)
            .await
            .map_err(|_| Error::ObjectNotFound)
    }

    setter! {
        /// Time to wait between keydown and keyup
        delay: Option<f64>,
        /// Whether to skip waiting after the action
        no_wait_after: Option<bool>,
        /// Maximum time to wait for the action
        timeout: Option<f64>
    }
}

pub struct LocatorFilterBuilder {
    inner: Weak<LocatorImpl>,
    args: FilterOptions,
}

impl LocatorFilterBuilder {
    pub(crate) fn new(inner: Weak<LocatorImpl>) -> Self {
        Self {
            inner,
            args: FilterOptions::default(),
        }
    }

    pub async fn filter(self) -> Result<Locator, Error> {
        let Self { inner, args } = self;
        upgrade(&inner)?
            .filter(args)
            .await
            .map(Locator::new)
            .map_err(|_| Error::ObjectNotFound)
    }

    setter! {
        /// Filter to elements containing this text
        has_text: Option<String>,
        /// Filter to elements not containing this text
        has_not_text: Option<String>,
        /// Filter to elements matching this selector
        has: Option<String>,
        /// Filter to elements not matching this selector
        has_not: Option<String>
    }
}

pub struct LocatorSetInputFilesBuilder {
    inner: Weak<LocatorImpl>,
    args: SetInputFilesArgs,
}

impl LocatorSetInputFilesBuilder {
    pub(crate) fn new(inner: Weak<LocatorImpl>, file: File) -> Self {
        Self {
            inner,
            args: SetInputFilesArgs {
                files: vec![file],
                timeout: None,
                no_wait_after: None,
            },
        }
    }

    pub async fn set_input_files(self) -> Result<(), Error> {
        let Self { inner, args } = self;
        upgrade(&inner)?
            .set_input_files(args)
            .await
            .map_err(|_| Error::ObjectNotFound)
    }

    setter! {
        /// Maximum time to wait for the action
        timeout: Option<f64>,
        /// Whether to skip waiting after the action
        no_wait_after: Option<bool>
    }

    /// Add multiple files to the input
    pub fn files(mut self, files: Vec<File>) -> Self {
        self.args.files = files;
        self
    }
}

pub struct LocatorDblClickBuilder {
    inner: Weak<LocatorImpl>,
    args: ClickArgs,
}

impl LocatorDblClickBuilder {
    pub(crate) fn new(inner: Weak<LocatorImpl>) -> Self {
        Self {
            inner,
            args: ClickArgs::default(),
        }
    }

    pub async fn dblclick(self) -> Result<(), Error> {
        let Self { inner, args } = self;
        upgrade(&inner)?
            .dblclick(args)
            .await
            .map_err(|_| Error::ObjectNotFound)
    }

    setter! {
        /// Mouse button to click
        button: Option<MouseButton>,
        /// Number of times to click (for dblclick, usually 2)
        click_count: Option<i32>,
        /// Time to wait between mousedown and mouseup
        delay: Option<f64>,
        /// Position relative to the element's bounding box
        position: Option<Position>,
        /// Keyboard modifiers to press
        modifiers: Option<Vec<KeyboardModifier>>,
        /// Whether to bypass actionability checks
        force: Option<bool>,
        /// Whether to skip waiting after the action
        no_wait_after: Option<bool>,
        /// Maximum time to wait for the action
        timeout: Option<f64>
    }
}

pub struct LocatorClearBuilder {
    inner: Weak<LocatorImpl>,
    args: ClearArgs,
}

impl LocatorClearBuilder {
    pub(crate) fn new(inner: Weak<LocatorImpl>) -> Self {
        Self {
            inner,
            args: ClearArgs::default(),
        }
    }

    pub async fn clear(self) -> Result<(), Error> {
        let Self { inner, args } = self;
        upgrade(&inner)?
            .clear(args)
            .await
            .map_err(|_| Error::ObjectNotFound)
    }

    setter! {
        /// Whether to bypass actionability checks
        force: Option<bool>,
        /// Whether to skip waiting after the action
        no_wait_after: Option<bool>,
        /// Maximum time to wait for the action
        timeout: Option<f64>
    }
}

pub struct LocatorTypeBuilder<'a> {
    inner: Weak<LocatorImpl>,
    text: &'a str,
    args: TypeArgs,
}

impl<'a> LocatorTypeBuilder<'a> {
    pub(crate) fn new(inner: Weak<LocatorImpl>, text: &'a str) -> Self {
        Self {
            inner,
            text,
            args: TypeArgs::default(),
        }
    }

    pub async fn r#type(self) -> Result<(), Error> {
        let Self { inner, text, args } = self;
        upgrade(&inner)?
            .r#type(text, args)
            .await
            .map_err(|_| Error::ObjectNotFound)
    }

    setter! {
        /// Time to wait between keystrokes
        delay: Option<f64>,
        /// Whether to skip waiting after the action
        no_wait_after: Option<bool>,
        /// Maximum time to wait for the action
        timeout: Option<f64>
    }
}

pub struct LocatorSelectOptionBuilder {
    inner: Weak<LocatorImpl>,
    args: SelectOptionArgs,
}

impl LocatorSelectOptionBuilder {
    pub(crate) fn new(inner: Weak<LocatorImpl>) -> Self {
        Self {
            inner,
            args: SelectOptionArgs::default(),
        }
    }

    pub async fn select_option(self) -> Result<Vec<String>, Error> {
        let Self { inner, args } = self;
        upgrade(&inner)?
            .select_option(args)
            .await
            .map_err(|_| Error::ObjectNotFound)
    }

    /// Select options by their values
    pub fn values(mut self, values: Vec<String>) -> Self {
        self.args.values = Some(values);
        self
    }

    /// Select options by their labels
    pub fn labels(mut self, labels: Vec<String>) -> Self {
        self.args.labels = Some(labels);
        self
    }

    /// Select options by their indices
    pub fn indices(mut self, indices: Vec<i32>) -> Self {
        self.args.indices = Some(indices);
        self
    }

    setter! {
        /// Whether to bypass actionability checks
        force: Option<bool>,
        /// Whether to skip waiting after the action
        no_wait_after: Option<bool>,
        /// Maximum time to wait for the action
        timeout: Option<f64>
    }
}
