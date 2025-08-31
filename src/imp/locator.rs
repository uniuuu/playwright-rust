use crate::imp::{
    core::*,
    element_handle::SetInputFilesArgs,
    frame::Frame,
    prelude::*,
    utils::{KeyboardModifier, MouseButton, Position},
};
use serde_json::{map::Map, value::Value};

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
            let element = frame.query_selector(&self.selector).await.map_err(Arc::from)?;
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
            let mut frame_args = crate::imp::frame::SetInputFilesArgs::new(&self.selector);
            frame_args.files = args.files;
            frame_args.timeout = args.timeout;
            frame_args.no_wait_after = args.no_wait_after;
            frame.set_input_files(frame_args).await.map_err(|e| e.into())
        } else {
            Err(Arc::new(crate::Error::ObjectNotFound))
        }
    }

    pub(crate) async fn focus(&self, timeout: Option<f64>) -> Result<(), Arc<Error>> {
        if let Some(frame) = self.frame.upgrade() {
            frame.focus(&self.selector, timeout).await.map_err(|e| e.into())
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
        let _ = send_message!(self, "clear", args);
        Ok(())
    }

    pub(crate) async fn r#type(&self, text: &str, args: TypeArgs) -> Result<(), Arc<Error>> {
        if let Some(frame) = self.frame.upgrade() {
            // Use ElementHandle-based approach via querySelector
            let element = frame.query_selector(&self.selector).await.map_err(Arc::from)?;
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
                options.extend(values.into_iter().map(crate::imp::element_handle::Opt::Value));
            }
            if let Some(labels) = args.labels {
                options.extend(labels.into_iter().map(crate::imp::element_handle::Opt::Label));
            }
            if let Some(indices) = args.indices {
                options.extend(indices.into_iter().map(|i| crate::imp::element_handle::Opt::Index(i as usize)));
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
            frame.text_content(&self.selector, timeout).await.map_err(Arc::from)
        } else {
            Err(Arc::new(crate::Error::ObjectNotFound))
        }
    }

    pub(crate) async fn inner_text(&self, timeout: Option<f64>) -> Result<String, Arc<Error>> {
        if let Some(frame) = self.frame.upgrade() {
            frame.inner_text(&self.selector, timeout).await.map_err(Arc::from)
        } else {
            Err(Arc::new(crate::Error::ObjectNotFound))
        }
    }

    pub(crate) async fn inner_html(&self, timeout: Option<f64>) -> Result<String, Arc<Error>> {
        if let Some(frame) = self.frame.upgrade() {
            frame.inner_html(&self.selector, timeout).await.map_err(Arc::from)
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
            frame.get_attribute(&self.selector, name, timeout).await.map_err(Arc::from)
        } else {
            Err(Arc::new(crate::Error::ObjectNotFound))
        }
    }

    pub(crate) async fn input_value(&self, timeout: Option<f64>) -> Result<String, Arc<Error>> {
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

    pub(crate) async fn count(&self) -> Result<usize, Arc<Error>> {
        let v = send_message!(self, "count", Map::new());
        let count = only_u64(&v)? as usize;
        Ok(count)
    }

    // State methods
    pub(crate) async fn is_visible(&self, timeout: Option<f64>) -> Result<bool, Arc<Error>> {
        if let Some(frame) = self.frame.upgrade() {
            frame.is_visible(&self.selector, timeout).await.map_err(Arc::from)
        } else {
            Err(Arc::new(crate::Error::ObjectNotFound))
        }
    }

    pub(crate) async fn is_hidden(&self, timeout: Option<f64>) -> Result<bool, Arc<Error>> {
        if let Some(frame) = self.frame.upgrade() {
            frame.is_hidden(&self.selector, timeout).await.map_err(Arc::from)
        } else {
            Err(Arc::new(crate::Error::ObjectNotFound))
        }
    }

    pub(crate) async fn is_enabled(&self, timeout: Option<f64>) -> Result<bool, Arc<Error>> {
        if let Some(frame) = self.frame.upgrade() {
            frame.is_enabled(&self.selector, timeout).await.map_err(Arc::from)
        } else {
            Err(Arc::new(crate::Error::ObjectNotFound))
        }
    }

    pub(crate) async fn is_disabled(&self, timeout: Option<f64>) -> Result<bool, Arc<Error>> {
        if let Some(frame) = self.frame.upgrade() {
            frame.is_disabled(&self.selector, timeout).await.map_err(Arc::from)
        } else {
            Err(Arc::new(crate::Error::ObjectNotFound))
        }
    }

    pub(crate) async fn is_checked(&self, timeout: Option<f64>) -> Result<bool, Arc<Error>> {
        if let Some(frame) = self.frame.upgrade() {
            frame.is_checked(&self.selector, timeout).await.map_err(Arc::from)
        } else {
            Err(Arc::new(crate::Error::ObjectNotFound))
        }
    }

    pub(crate) async fn is_editable(&self, timeout: Option<f64>) -> Result<bool, Arc<Error>> {
        if let Some(frame) = self.frame.upgrade() {
            frame.is_editable(&self.selector, timeout).await.map_err(Arc::from)
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
        self.channel.as_ref().expect("RemoteObject methods should only be called on server-side locators")
    }

    fn channel_mut(&mut self) -> &mut ChannelOwner {
        self.channel.as_mut().expect("RemoteObject methods should only be called on server-side locators")
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
