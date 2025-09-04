use crate::imp::{
    core::*,
    element_handle::SetInputFilesArgs,
    frame::Frame,
    prelude::*,
    utils::{KeyboardModifier, MouseButton, Position},
};
use serde_json::map::Map;

#[derive(Debug)]
pub(crate) struct Locator {
    // For server-side created locators (legacy)
    channel: Option<ChannelOwner>,
    // Core locator data (used by both client-side and server-side)
    selector: String,
    frame: Weak<Frame>,
}

impl Locator {
    // Legacy server-side constructor (for existing protocol support)
    pub(crate) fn try_new(ctx: &Context, channel: ChannelOwner) -> Result<Self, Error> {
        let Initializer {
            selector,
            frame: OnlyGuid { guid },
        } = serde_json::from_value(channel.initializer.clone())?;

        let frame = get_object!(ctx, &guid, Frame)?;

        Ok(Self {
            channel: Some(channel),
            selector,
            frame,
        })
    }

    // New client-side constructor (following TypeScript/Go pattern)
    pub(crate) fn new_client_side(frame: Weak<Frame>, selector: String) -> Self {
        Self {
            channel: None, // No server-side channel needed
            selector,
            frame,
        }
    }

    pub(crate) fn selector(&self) -> &str {
        &self.selector
    }

    pub(crate) fn frame(&self) -> Weak<Frame> {
        self.frame.clone()
    }

    // Action methods - Delegate to Frame methods (following TypeScript/Go pattern)
    pub(crate) async fn click(&self, args: ClickArgs) -> Result<(), Arc<Error>> {
        if let Some(frame) = self.frame.upgrade() {
            // Convert Locator ClickArgs to Frame ClickArgs by adding selector
            let mut frame_args = crate::imp::frame::ClickArgs::new(&self.selector);
            frame_args.modifiers = args.modifiers;
            frame_args.position = args.position;
            frame_args.delay = args.delay;
            frame_args.button = args.button;
            frame_args.click_count = args.click_count;
            frame_args.timeout = args.timeout;
            frame_args.force = args.force;
            frame_args.no_wait_after = args.no_wait_after;
            // trial defaults to None in constructor
            frame.click(frame_args).await
        } else {
            Err(Arc::new(crate::Error::ObjectNotFound))
        }
    }

    pub(crate) async fn dblclick(&self, args: ClickArgs) -> Result<(), Arc<Error>> {
        if let Some(frame) = self.frame.upgrade() {
            let mut frame_args = crate::imp::frame::ClickArgs::new(&self.selector);
            frame_args.modifiers = args.modifiers;
            frame_args.position = args.position;
            frame_args.delay = args.delay;
            frame_args.button = args.button;
            frame_args.click_count = args.click_count;
            frame_args.timeout = args.timeout;
            frame_args.force = args.force;
            frame_args.no_wait_after = args.no_wait_after;
            frame.dblclick(frame_args).await
        } else {
            Err(Arc::new(crate::Error::ObjectNotFound))
        }
    }

    pub(crate) async fn fill(&self, value: &str, args: FillArgs) -> Result<(), Arc<Error>> {
        if let Some(frame) = self.frame.upgrade() {
            // SPECIAL HANDLING: Check if this is a complex selector nth-index marker
            if self.selector.starts_with("(") && self.selector.contains(")>>>nth-index-") {
                // Parse the complex selector: "(input, select, textarea)>>>nth-index-7"
                if let Some(close_paren) = self.selector.find(")>>>nth-index-") {
                    let base_selector = &self.selector[1..close_paren]; // Remove outer parentheses
                    let index_part = &self.selector[close_paren + 14..]; // After ")>>>nth-index-"
                    if let Ok(index) = index_part.parse::<usize>() {
                        // Use query_selector_all approach to get the specific element via frame
                        let elements = frame
                            .query_selector_all(base_selector)
                            .await
                            .map_err(Arc::from)?;

                        if let Some(element_weak) = elements.get(index) {
                            // Fill the specific element
                            if let Some(element) = element_weak.upgrade() {
                                let mut element_fill_args =
                                    crate::imp::element_handle::FillArgs::new(value);
                                element_fill_args.timeout = args.timeout;
                                element_fill_args.no_wait_after = args.no_wait_after;
                                return element.fill(element_fill_args).await.map_err(Arc::from);
                            }
                        } else {
                            // Index out of bounds
                            return Err(Arc::new(crate::Error::ObjectNotFound));
                        }
                    }
                }
            }

            // Regular selector handling
            let mut frame_args = crate::imp::frame::FillArgs::new(&self.selector, value);
            frame_args.timeout = args.timeout;
            frame_args.no_wait_after = args.no_wait_after;
            frame.fill(frame_args).await
        } else {
            Err(Arc::new(crate::Error::ObjectNotFound))
        }
    }

    pub(crate) async fn hover(&self, args: HoverArgs) -> Result<(), Arc<Error>> {
        if let Some(frame) = self.frame.upgrade() {
            let mut frame_args = crate::imp::frame::HoverArgs::new(&self.selector);
            frame_args.position = args.position;
            frame_args.modifiers = args.modifiers;
            frame_args.force = args.force;
            frame_args.timeout = args.timeout;
            frame.hover(frame_args).await
        } else {
            Err(Arc::new(crate::Error::ObjectNotFound))
        }
    }

    pub(crate) async fn check(&self, args: CheckArgs) -> Result<(), Arc<Error>> {
        if let Some(frame) = self.frame.upgrade() {
            let mut frame_args = crate::imp::frame::CheckArgs::new(&self.selector);
            frame_args.position = args.position;
            frame_args.force = args.force;
            frame_args.no_wait_after = args.no_wait_after;
            frame_args.timeout = args.timeout;
            frame.check(frame_args).await
        } else {
            Err(Arc::new(crate::Error::ObjectNotFound))
        }
    }

    pub(crate) async fn uncheck(&self, args: CheckArgs) -> Result<(), Arc<Error>> {
        if let Some(frame) = self.frame.upgrade() {
            let mut frame_args = crate::imp::frame::CheckArgs::new(&self.selector);
            frame_args.position = args.position;
            frame_args.force = args.force;
            frame_args.no_wait_after = args.no_wait_after;
            frame_args.timeout = args.timeout;
            frame.uncheck(frame_args).await
        } else {
            Err(Arc::new(crate::Error::ObjectNotFound))
        }
    }

    pub(crate) async fn press(&self, key: &str, args: PressArgs) -> Result<(), Arc<Error>> {
        if let Some(frame) = self.frame.upgrade() {
            // Use ElementHandle-based approach via querySelector since Frame's press method signature is unclear
            let element = frame
                .query_selector(&self.selector)
                .await
                .map_err(Arc::from)?;
            if let Some(element) = element {
                if let Some(element) = element.upgrade() {
                    let mut press_args = crate::imp::element_handle::PressArgs::new(key);
                    press_args.delay = args.delay;
                    press_args.timeout = args.timeout;
                    press_args.no_wait_after = args.no_wait_after;
                    element.press(press_args).await.map_err(Arc::from)
                } else {
                    Err(Arc::new(crate::Error::ObjectNotFound))
                }
            } else {
                Err(Arc::new(crate::Error::ObjectNotFound))
            }
        } else {
            Err(Arc::new(crate::Error::ObjectNotFound))
        }
    }

    pub(crate) async fn set_input_files(&self, args: SetInputFilesArgs) -> Result<(), Arc<Error>> {
        if let Some(frame) = self.frame.upgrade() {
            // SPECIAL HANDLING: Check if this is a complex selector nth-index marker
            if self.selector.starts_with("(") && self.selector.contains(")>>>nth-index-") {
                // Parse the complex selector: "(input, select, textarea)>>>nth-index-4"
                if let Some(close_paren) = self.selector.find(")>>>nth-index-") {
                    let base_selector = &self.selector[1..close_paren]; // Remove outer parentheses
                    let index_part = &self.selector[close_paren + 14..]; // After ")>>>nth-index-"
                    if let Ok(index) = index_part.parse::<usize>() {
                        // Use query_selector_all approach to get the specific element via frame
                        let elements = frame
                            .query_selector_all(base_selector)
                            .await
                            .map_err(Arc::from)?;

                        if let Some(element_weak) = elements.get(index) {
                            // Set input files on the specific element
                            if let Some(element) = element_weak.upgrade() {
                                return element.set_input_files(args).await.map_err(Arc::from);
                            }
                        }
                        // If element not found or index out of bounds, fall through to ObjectNotFound
                        return Err(Arc::new(crate::Error::ObjectNotFound));
                    }
                }
            }

            // Regular selector handling
            let mut frame_args = crate::imp::frame::SetInputFilesArgs::new(&self.selector);
            frame_args.files = args.files;
            frame_args.timeout = args.timeout;
            frame_args.no_wait_after = args.no_wait_after;
            frame.set_input_files(frame_args).await.map_err(|e| e)
        } else {
            Err(Arc::new(crate::Error::ObjectNotFound))
        }
    }

    pub(crate) async fn focus(&self, timeout: Option<f64>) -> Result<(), Arc<Error>> {
        if let Some(frame) = self.frame.upgrade() {
            frame.focus(&self.selector, timeout).await.map_err(|e| e)
        } else {
            Err(Arc::new(crate::Error::ObjectNotFound))
        }
    }

    pub(crate) async fn blur(&self, timeout: Option<f64>) -> Result<(), Arc<Error>> {
        #[skip_serializing_none]
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Args {
            timeout: Option<f64>,
        }
        let args = Args { timeout };
        let _ = send_message!(self, "blur", args);
        Ok(())
    }

    pub(crate) async fn clear(&self, args: ClearArgs) -> Result<(), Arc<Error>> {
        // Handle both server-side and client-side locators
        if self.channel.is_some() {
            // Server-side locator: use protocol message
            let _ = send_message!(self, "clear", args);
            Ok(())
        } else {
            // Client-side locator: follow TypeScript/Go pattern - clear = fill("")
            // This matches official Playwright implementations:
            // TypeScript: async clear() { await this.fill('', options); }
            // Go: func Clear() { return Fill(""); }
            let fill_args = FillArgs {
                force: args.force,
                no_wait_after: args.no_wait_after,
                timeout: args.timeout,
            };
            self.fill("", fill_args).await
        }
    }

    pub(crate) async fn r#type(&self, text: &str, args: TypeArgs) -> Result<(), Arc<Error>> {
        if let Some(frame) = self.frame.upgrade() {
            // Use ElementHandle-based approach via querySelector
            let element = frame
                .query_selector(&self.selector)
                .await
                .map_err(Arc::from)?;
            if let Some(element) = element {
                if let Some(element) = element.upgrade() {
                    let mut type_args = crate::imp::element_handle::TypeArgs::new(text);
                    type_args.delay = args.delay;
                    type_args.timeout = args.timeout;
                    type_args.no_wait_after = args.no_wait_after;
                    element.r#type(type_args).await.map_err(Arc::from)
                } else {
                    Err(Arc::new(crate::Error::ObjectNotFound))
                }
            } else {
                Err(Arc::new(crate::Error::ObjectNotFound))
            }
        } else {
            Err(Arc::new(crate::Error::ObjectNotFound))
        }
    }

    pub(crate) async fn select_option(
        &self,
        args: SelectOptionArgs,
    ) -> Result<Vec<String>, Arc<Error>> {
        if let Some(frame) = self.frame.upgrade() {
            let mut frame_args = crate::imp::frame::SelectOptionArgs::new(&self.selector);

            // Convert Locator args to Frame args
            let mut options = Vec::new();
            if let Some(values) = args.values {
                options.extend(
                    values
                        .into_iter()
                        .map(crate::imp::element_handle::Opt::Value),
                );
            }
            if let Some(labels) = args.labels {
                options.extend(
                    labels
                        .into_iter()
                        .map(crate::imp::element_handle::Opt::Label),
                );
            }
            if let Some(indices) = args.indices {
                options.extend(
                    indices
                        .into_iter()
                        .map(|i| crate::imp::element_handle::Opt::Index(i as usize)),
                );
            }

            if !options.is_empty() {
                frame_args.options = Some(options);
            }

            frame_args.timeout = args.timeout;
            frame_args.no_wait_after = args.no_wait_after;
            frame.select_option(frame_args).await.map_err(Arc::from)
        } else {
            Err(Arc::new(crate::Error::ObjectNotFound))
        }
    }

    // Query methods
    pub(crate) async fn text_content(
        &self,
        timeout: Option<f64>,
    ) -> Result<Option<String>, Arc<Error>> {
        if let Some(frame) = self.frame.upgrade() {
            // SPECIAL HANDLING: Check for problematic XPath patterns that cause hanging
            if self.selector.starts_with("xpath=") && self.is_complex_xpath() {
                // Convert complex XPath to JavaScript evaluation to avoid driver hanging
                return self
                    .handle_complex_xpath_text_content(&frame, timeout)
                    .await;
            }

            // SPECIAL HANDLING: Check if this is a complex selector nth-index marker
            if self.selector.starts_with("(") && self.selector.contains(")>>>nth-index-") {
                // Parse the complex selector: "(input, select, textarea)>>>nth-index-4"
                if let Some(close_paren) = self.selector.find(")>>>nth-index-") {
                    let base_selector = &self.selector[1..close_paren]; // Remove outer parentheses
                    let index_part = &self.selector[close_paren + 14..]; // After ")>>>nth-index-"
                    if let Ok(index) = index_part.parse::<usize>() {
                        // Use query_selector_all approach to get the specific element via frame
                        let elements = frame
                            .query_selector_all(base_selector)
                            .await
                            .map_err(Arc::from)?;

                        if let Some(element_weak) = elements.get(index) {
                            // Get text content from the specific element
                            if let Some(element) = element_weak.upgrade() {
                                return element.text_content().await.map_err(Arc::from);
                            }
                        }
                        // If element not found or index out of bounds, return None
                        return Ok(None);
                    }
                }
            }

            // SPECIAL HANDLING: Check if this is a simple nth-of-type selector created by nth() method
            if self.selector.contains(":nth-of-type(") {
                // Parse selectors like "label:nth-of-type(2)" created by nth() method
                if let Some(nth_pos) = self.selector.find(":nth-of-type(") {
                    let base_selector = &self.selector[..nth_pos]; // e.g., "label"
                    let nth_part = &self.selector[nth_pos + 13..]; // After ":nth-of-type("
                    if let Some(close_paren) = nth_part.find(')') {
                        let index_str = &nth_part[..close_paren];
                        if let Ok(css_index) = index_str.parse::<usize>() {
                            // CSS nth-of-type is 1-based, convert to 0-based for array indexing
                            let array_index = css_index.saturating_sub(1);

                            // Use query_selector_all to get all elements, then select by index
                            let elements = frame
                                .query_selector_all(base_selector)
                                .await
                                .map_err(Arc::from)?;

                            if let Some(element_weak) = elements.get(array_index) {
                                // Get text content from the specific element
                                if let Some(element) = element_weak.upgrade() {
                                    return element.text_content().await.map_err(Arc::from);
                                }
                            }
                            // If element not found or index out of bounds, return None
                            return Ok(None);
                        }
                    }
                }
            }

            // Regular selector handling
            frame
                .text_content(&self.selector, timeout)
                .await
                .map_err(Arc::from)
        } else {
            Err(Arc::new(crate::Error::ObjectNotFound))
        }
    }

    fn is_complex_xpath(&self) -> bool {
        // Detect XPath patterns that are known to cause hanging in the driver
        self.selector.contains("|") || // Union operators like "input | select | textarea"
        self.selector.contains("ancestor::") || // Ancestor traversal
        self.selector.contains("descendant::") || // Descendant traversal  
        self.selector.contains("following::") || // Following sibling traversal
        self.selector.contains("preceding::") // Preceding sibling traversal
    }

    async fn handle_complex_xpath_text_content(
        &self,
        frame: &Frame,
        _timeout: Option<f64>,
    ) -> Result<Option<String>, Arc<Error>> {
        // Extract the XPath expression (remove "xpath=" prefix)
        let xpath_expr = &self.selector[6..]; // Remove "xpath=" prefix

        // Use JavaScript evaluation to handle complex XPath safely
        // This avoids the hanging issue in the Playwright driver
        let js_code = format!(
            r#"
            (function() {{
                try {{
                    const result = document.evaluate(
                        '{}',
                        document,
                        null,
                        XPathResult.FIRST_ORDERED_NODE_TYPE,
                        null
                    );
                    const node = result.singleNodeValue;
                    return node ? node.textContent : null;
                }} catch (error) {{
                    console.error('XPath evaluation error:', error);
                    return null;
                }}
            }})()
            "#,
            xpath_expr.replace("'", "\\'") // Escape single quotes
        );

        match frame
            .evaluate::<(), serde_json::Value>(&js_code, None::<()>)
            .await
        {
            Ok(result) => {
                // Handle the JavaScript result
                if let Some(s) = result.as_str() {
                    Ok(Some(s.to_string()))
                } else if result.is_null() {
                    Ok(None)
                } else {
                    // Convert other types to string
                    Ok(Some(result.to_string()))
                }
            }
            Err(_e) => {
                // If JavaScript evaluation fails, fall back to regular XPath handling
                // This may still hang, but it's a last resort
                frame
                    .text_content(&self.selector, None) // Use None for timeout to avoid double timeout
                    .await
                    .map_err(Arc::from)
            }
        }
    }

    pub(crate) async fn inner_text(&self, timeout: Option<f64>) -> Result<String, Arc<Error>> {
        if let Some(frame) = self.frame.upgrade() {
            frame
                .inner_text(&self.selector, timeout)
                .await
                .map_err(Arc::from)
        } else {
            Err(Arc::new(crate::Error::ObjectNotFound))
        }
    }

    pub(crate) async fn inner_html(&self, timeout: Option<f64>) -> Result<String, Arc<Error>> {
        if let Some(frame) = self.frame.upgrade() {
            frame
                .inner_html(&self.selector, timeout)
                .await
                .map_err(Arc::from)
        } else {
            Err(Arc::new(crate::Error::ObjectNotFound))
        }
    }

    pub(crate) async fn get_attribute(
        &self,
        name: &str,
        timeout: Option<f64>,
    ) -> Result<Option<String>, Arc<Error>> {
        if let Some(frame) = self.frame.upgrade() {
            // SPECIAL HANDLING: Check if this is a complex selector nth-index marker
            if self.selector.starts_with("(") && self.selector.contains(")>>>nth-index-") {
                // Parse the complex selector: "(input, select, textarea)>>>nth-index-7"
                if let Some(close_paren) = self.selector.find(")>>>nth-index-") {
                    let base_selector = &self.selector[1..close_paren]; // Remove outer parentheses
                    let index_part = &self.selector[close_paren + 14..]; // After ")>>>nth-index-"
                    if let Ok(index) = index_part.parse::<usize>() {
                        // Use query_selector_all approach to get the specific element via frame
                        let elements = frame
                            .query_selector_all(base_selector)
                            .await
                            .map_err(Arc::from)?;

                        if let Some(element_weak) = elements.get(index) {
                            // Get attribute from the specific element
                            if let Some(element) = element_weak.upgrade() {
                                return element.get_attribute(name).await.map_err(Arc::from);
                            }
                        } else {
                            // Index out of bounds
                            return Ok(None);
                        }
                    }
                }
            }

            // Regular selector handling
            frame
                .get_attribute(&self.selector, name, timeout)
                .await
                .map_err(Arc::from)
        } else {
            Err(Arc::new(crate::Error::ObjectNotFound))
        }
    }

    pub(crate) async fn input_value(&self, timeout: Option<f64>) -> Result<String, Arc<Error>> {
        // Check if this is a client-side locator (like get_attribute does)
        if let Some(frame) = self.frame.upgrade() {
            // SPECIAL HANDLING: Check if this is a complex selector nth-index marker
            if self.selector.starts_with("(") && self.selector.contains(")>>>nth-index-") {
                // Parse the complex selector: "(input, select, textarea)>>>nth-index-7"
                if let Some(close_paren) = self.selector.find(")>>>nth-index-") {
                    let base_selector = &self.selector[1..close_paren]; // Remove outer parentheses
                    let index_part = &self.selector[close_paren + 14..]; // After ")>>>nth-index-"
                    if let Ok(index) = index_part.parse::<usize>() {
                        // Get input value from the specific element using JavaScript evaluation
                        let js_code = format!(
                            "(() => {{
                                const elements = document.querySelectorAll('{}');
                                const element = elements[{}];
                                return element ? (element.value || '') : '';
                            }})()",
                            base_selector.replace("'", "\\'"),
                            index
                        );
                        return frame
                            .evaluate::<(), String>(&js_code, None)
                            .await
                            .map_err(Arc::from);
                    }
                }
            }

            // Use frame.evaluate with JavaScript to get input value
            let js_code = format!(
                "(() => {{
                    const element = document.querySelector('{}');
                    return element ? (element.value || '') : '';
                }})()",
                self.selector.replace("'", "\\'")
            );
            frame
                .evaluate::<(), String>(&js_code, None)
                .await
                .map_err(Arc::from)
        } else {
            // Fallback for server-side locators (existing implementation)
            #[skip_serializing_none]
            #[derive(Serialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
                timeout: Option<f64>,
            }
            let args = Args { timeout };
            let v = send_message!(self, "inputValue", args);
            let value = only_str(&v)?;
            Ok(value.to_owned())
        }
    }

    pub(crate) async fn count(&self) -> Result<usize, Arc<Error>> {
        // Handle both server-side and client-side locators
        if self.channel.is_some() {
            // Server-side locator: use protocol message
            let v = send_message!(self, "count", Map::new());
            let count = only_u64(&v)? as usize;
            Ok(count)
        } else {
            // Client-side locator: delegate to frame
            if let Some(frame) = self.frame.upgrade() {
                let elements = frame
                    .query_selector_all(&self.selector)
                    .await
                    .map_err(Arc::from)?;
                Ok(elements.len())
            } else {
                Err(Arc::new(crate::Error::ObjectNotFound))
            }
        }
    }

    // State methods
    pub(crate) async fn is_visible(&self, timeout: Option<f64>) -> Result<bool, Arc<Error>> {
        if let Some(frame) = self.frame.upgrade() {
            frame
                .is_visible(&self.selector, timeout)
                .await
                .map_err(Arc::from)
        } else {
            Err(Arc::new(crate::Error::ObjectNotFound))
        }
    }

    pub(crate) async fn is_hidden(&self, timeout: Option<f64>) -> Result<bool, Arc<Error>> {
        if let Some(frame) = self.frame.upgrade() {
            frame
                .is_hidden(&self.selector, timeout)
                .await
                .map_err(Arc::from)
        } else {
            Err(Arc::new(crate::Error::ObjectNotFound))
        }
    }

    pub(crate) async fn is_enabled(&self, timeout: Option<f64>) -> Result<bool, Arc<Error>> {
        if let Some(frame) = self.frame.upgrade() {
            frame
                .is_enabled(&self.selector, timeout)
                .await
                .map_err(Arc::from)
        } else {
            Err(Arc::new(crate::Error::ObjectNotFound))
        }
    }

    pub(crate) async fn is_disabled(&self, timeout: Option<f64>) -> Result<bool, Arc<Error>> {
        if let Some(frame) = self.frame.upgrade() {
            frame
                .is_disabled(&self.selector, timeout)
                .await
                .map_err(Arc::from)
        } else {
            Err(Arc::new(crate::Error::ObjectNotFound))
        }
    }

    pub(crate) async fn is_checked(&self, timeout: Option<f64>) -> Result<bool, Arc<Error>> {
        if let Some(frame) = self.frame.upgrade() {
            frame
                .is_checked(&self.selector, timeout)
                .await
                .map_err(Arc::from)
        } else {
            Err(Arc::new(crate::Error::ObjectNotFound))
        }
    }

    pub(crate) async fn is_editable(&self, timeout: Option<f64>) -> Result<bool, Arc<Error>> {
        if let Some(frame) = self.frame.upgrade() {
            frame
                .is_editable(&self.selector, timeout)
                .await
                .map_err(Arc::from)
        } else {
            Err(Arc::new(crate::Error::ObjectNotFound))
        }
    }

    // Chaining methods
    pub(crate) async fn first(&self) -> Result<Weak<Locator>, Arc<Error>> {
        let v = send_message!(self, "first", Map::new());
        let guid = only_guid(&v)?;
        let locator = get_object!(self.context()?.lock().unwrap(), guid, Locator)?;
        Ok(locator)
    }

    pub(crate) async fn last(&self) -> Result<Weak<Locator>, Arc<Error>> {
        let v = send_message!(self, "last", Map::new());
        let guid = only_guid(&v)?;
        let locator = get_object!(self.context()?.lock().unwrap(), guid, Locator)?;
        Ok(locator)
    }

    pub(crate) async fn nth(&self, index: i32) -> Result<Weak<Locator>, Arc<Error>> {
        // Handle both server-side and client-side locators
        if self.channel.is_some() {
            // Server-side locator: use protocol message
            #[derive(Serialize)]
            #[serde(rename_all = "camelCase")]
            struct Args {
                index: i32,
            }
            let args = Args { index };
            let v = send_message!(self, "nth", args);
            let guid = only_guid(&v)?;
            let locator = get_object!(self.context()?.lock().unwrap(), guid, Locator)?;
            Ok(locator)
        } else {
            // Client-side locator: create new locator with nth selector
            // CRITICAL: Use self.frame directly instead of self.frame.upgrade()
            // This preserves the same frame reference as the parent locator

            // SPECIAL HANDLING FOR COMPLEX SELECTORS (contains comma)
            if self.selector.contains(',') {
                // Complex selectors like "input, select, textarea" cannot use CSS :nth-of-type()
                // because it creates invalid selectors like "input, select, textarea:nth-of-type(8)"
                // which selects ALL inputs + ALL selects + 8th textarea (causing homogenized results)
                //
                // SOLUTION: Create a unique selector for the specific element at this index
                // We'll delegate to get_attribute() to use querySelector approach
                let unique_selector = format!("({})>>>nth-index-{}", self.selector, index);
                let locator = Locator::new_client_side(self.frame.clone(), unique_selector);
                let locator_arc = Arc::new(locator);
                let locator_weak = Arc::downgrade(&locator_arc);

                // Keep the locator alive (same pattern as frame.locator())
                std::mem::forget(locator_arc.clone());

                Ok(locator_weak)
            } else {
                // Simple selectors can use CSS nth-of-type safely
                // Use CSS nth-of-type instead of nth engine for compatibility with older drivers
                // CSS nth is 1-based, so add 1 to the 0-based index
                let nth_selector = format!("{}:nth-of-type({})", self.selector, index + 1);
                let locator = Locator::new_client_side(self.frame.clone(), nth_selector);
                let locator_arc = Arc::new(locator);
                let locator_weak = Arc::downgrade(&locator_arc);

                // Keep the locator alive (same pattern as frame.locator())
                std::mem::forget(locator_arc.clone());

                Ok(locator_weak)
            }
        }
    }

    pub(crate) async fn filter(&self, options: FilterOptions) -> Result<Weak<Locator>, Arc<Error>> {
        let v = send_message!(self, "filter", options);
        let guid = only_guid(&v)?;
        let locator = get_object!(self.context()?.lock().unwrap(), guid, Locator)?;
        Ok(locator)
    }
}

// Only implement RemoteObject for server-side locators
impl RemoteObject for Locator {
    fn channel(&self) -> &ChannelOwner {
        self.channel
            .as_ref()
            .expect("RemoteObject methods should only be called on server-side locators")
    }

    fn channel_mut(&mut self) -> &mut ChannelOwner {
        self.channel
            .as_mut()
            .expect("RemoteObject methods should only be called on server-side locators")
    }
}

// Helper types for locator operations
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Initializer {
    selector: String,
    frame: OnlyGuid,
}

#[skip_serializing_none]
#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ClickArgs {
    pub(crate) button: Option<MouseButton>,
    pub(crate) click_count: Option<i32>,
    pub(crate) delay: Option<f64>,
    pub(crate) position: Option<Position>,
    pub(crate) modifiers: Option<Vec<KeyboardModifier>>,
    pub(crate) force: Option<bool>,
    pub(crate) no_wait_after: Option<bool>,
    pub(crate) timeout: Option<f64>,
}

#[skip_serializing_none]
#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct FillArgs {
    pub(crate) force: Option<bool>,
    pub(crate) no_wait_after: Option<bool>,
    pub(crate) timeout: Option<f64>,
}

#[skip_serializing_none]
#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct HoverArgs {
    pub(crate) position: Option<Position>,
    pub(crate) modifiers: Option<Vec<KeyboardModifier>>,
    pub(crate) force: Option<bool>,
    pub(crate) timeout: Option<f64>,
}

#[skip_serializing_none]
#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CheckArgs {
    pub(crate) position: Option<Position>,
    pub(crate) force: Option<bool>,
    pub(crate) no_wait_after: Option<bool>,
    pub(crate) timeout: Option<f64>,
}

#[skip_serializing_none]
#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PressArgs {
    pub(crate) delay: Option<f64>,
    pub(crate) no_wait_after: Option<bool>,
    pub(crate) timeout: Option<f64>,
}

#[skip_serializing_none]
#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct FilterOptions {
    pub(crate) has_text: Option<String>,
    pub(crate) has_not_text: Option<String>,
    pub(crate) has: Option<String>,
    pub(crate) has_not: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ClearArgs {
    pub(crate) force: Option<bool>,
    pub(crate) no_wait_after: Option<bool>,
    pub(crate) timeout: Option<f64>,
}

#[skip_serializing_none]
#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct TypeArgs {
    pub(crate) delay: Option<f64>,
    pub(crate) no_wait_after: Option<bool>,
    pub(crate) timeout: Option<f64>,
}

#[skip_serializing_none]
#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SelectOptionArgs {
    pub(crate) values: Option<Vec<String>>,
    pub(crate) labels: Option<Vec<String>>,
    pub(crate) indices: Option<Vec<i32>>,
    pub(crate) force: Option<bool>,
    pub(crate) no_wait_after: Option<bool>,
    pub(crate) timeout: Option<f64>,
}
