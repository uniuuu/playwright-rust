use crate::imp::{
    core::*,
    frame::Frame,
    prelude::*,
    utils::{KeyboardModifier, MouseButton, Position}
};
use serde_json::{map::Map, value::Value};

#[derive(Debug)]
pub(crate) struct Locator {
    channel: ChannelOwner,
    selector: String,
    frame: Weak<Frame>,
}

impl Locator {
    pub(crate) fn try_new(ctx: &Context, channel: ChannelOwner) -> Result<Self, Error> {
        let Initializer { selector, frame: OnlyGuid { guid } } = 
            serde_json::from_value(channel.initializer.clone())?;
        
        let frame = get_object!(ctx, &guid, Frame)?;
        
        Ok(Self {
            channel,
            selector,
            frame,
        })
    }

    pub(crate) fn new_with_selector(
        frame: Weak<Frame>,
        selector: String,
        ctx: Weak<Mutex<Context>>
    ) -> Result<Self, Error> {
        let guid = generate_guid();
        let typ = Str::validate("Locator".into()).unwrap();
        
        // Create minimal initializer for the locator
        let frame_arc = upgrade(&frame)?;
        let initializer = serde_json::json!({
            "selector": selector,
            "frame": {
                "guid": frame_arc.guid().as_str()
            }
        });
        
        let channel = ChannelOwner::new(
            ctx,
            RemoteWeak::Frame(frame.clone()),
            typ,
            guid,
            initializer,
        );
        
        Ok(Self {
            channel,
            selector: selector.clone(),
            frame,
        })
    }

    pub(crate) fn selector(&self) -> &str {
        &self.selector
    }

    pub(crate) fn frame(&self) -> Weak<Frame> {
        self.frame.clone()
    }

    // Action methods
    pub(crate) async fn click(&self, args: ClickArgs) -> Result<(), Arc<Error>> {
        let _ = send_message!(self, "click", args);
        Ok(())
    }

    pub(crate) async fn fill(&self, value: &str, args: FillArgs) -> Result<(), Arc<Error>> {
        #[skip_serializing_none]
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Args<'a> {
            value: &'a str,
            #[serde(flatten)]
            options: FillArgs,
        }
        let args = Args { value, options: args };
        let _ = send_message!(self, "fill", args);
        Ok(())
    }

    pub(crate) async fn hover(&self, args: HoverArgs) -> Result<(), Arc<Error>> {
        let _ = send_message!(self, "hover", args);
        Ok(())
    }

    pub(crate) async fn check(&self, args: CheckArgs) -> Result<(), Arc<Error>> {
        let _ = send_message!(self, "check", args);
        Ok(())
    }

    pub(crate) async fn uncheck(&self, args: CheckArgs) -> Result<(), Arc<Error>> {
        let _ = send_message!(self, "uncheck", args);
        Ok(())
    }

    pub(crate) async fn press(&self, key: &str, args: PressArgs) -> Result<(), Arc<Error>> {
        #[skip_serializing_none]
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Args<'a> {
            key: &'a str,
            #[serde(flatten)]
            options: PressArgs,
        }
        let args = Args { key, options: args };
        let _ = send_message!(self, "press", args);
        Ok(())
    }

    // Query methods
    pub(crate) async fn text_content(&self, timeout: Option<f64>) -> Result<Option<String>, Arc<Error>> {
        #[skip_serializing_none]
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Args {
            timeout: Option<f64>,
        }
        let args = Args { timeout };
        let v = send_message!(self, "textContent", args);
        let text = match first(&v) {
            Some(Value::String(s)) => Some(s.clone()),
            Some(Value::Null) | None => None,
            _ => return Err(Arc::new(Error::InvalidParams))
        };
        Ok(text)
    }

    pub(crate) async fn inner_text(&self, timeout: Option<f64>) -> Result<String, Arc<Error>> {
        #[skip_serializing_none]
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Args {
            timeout: Option<f64>,
        }
        let args = Args { timeout };
        let v = send_message!(self, "innerText", args);
        let text = only_str(&v)?;
        Ok(text.to_owned())
    }

    pub(crate) async fn get_attribute(&self, name: &str, timeout: Option<f64>) -> Result<Option<String>, Arc<Error>> {
        #[skip_serializing_none]
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Args<'a> {
            name: &'a str,
            timeout: Option<f64>,
        }
        let args = Args { name, timeout };
        let v = send_message!(self, "getAttribute", args);
        let attr = match first(&v) {
            Some(Value::String(s)) => Some(s.clone()),
            Some(Value::Null) | None => None,
            _ => return Err(Arc::new(Error::InvalidParams))
        };
        Ok(attr)
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
        #[skip_serializing_none]
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Args {
            timeout: Option<f64>,
        }
        let args = Args { timeout };
        let v = send_message!(self, "isVisible", args);
        let visible = only_bool(&v)?;
        Ok(visible)
    }

    pub(crate) async fn is_hidden(&self, timeout: Option<f64>) -> Result<bool, Arc<Error>> {
        #[skip_serializing_none]
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Args {
            timeout: Option<f64>,
        }
        let args = Args { timeout };
        let v = send_message!(self, "isHidden", args);
        let hidden = only_bool(&v)?;
        Ok(hidden)
    }

    pub(crate) async fn is_enabled(&self, timeout: Option<f64>) -> Result<bool, Arc<Error>> {
        #[skip_serializing_none]
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Args {
            timeout: Option<f64>,
        }
        let args = Args { timeout };
        let v = send_message!(self, "isEnabled", args);
        let enabled = only_bool(&v)?;
        Ok(enabled)
    }

    pub(crate) async fn is_disabled(&self, timeout: Option<f64>) -> Result<bool, Arc<Error>> {
        #[skip_serializing_none]
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Args {
            timeout: Option<f64>,
        }
        let args = Args { timeout };
        let v = send_message!(self, "isDisabled", args);
        let disabled = only_bool(&v)?;
        Ok(disabled)
    }

    pub(crate) async fn is_checked(&self, timeout: Option<f64>) -> Result<bool, Arc<Error>> {
        #[skip_serializing_none]
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Args {
            timeout: Option<f64>,
        }
        let args = Args { timeout };
        let v = send_message!(self, "isChecked", args);
        let checked = only_bool(&v)?;
        Ok(checked)
    }

    pub(crate) async fn is_editable(&self, timeout: Option<f64>) -> Result<bool, Arc<Error>> {
        #[skip_serializing_none]
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Args {
            timeout: Option<f64>,
        }
        let args = Args { timeout };
        let v = send_message!(self, "isEditable", args);
        let editable = only_bool(&v)?;
        Ok(editable)
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

impl RemoteObject for Locator {
    fn channel(&self) -> &ChannelOwner {
        &self.channel
    }

    fn channel_mut(&mut self) -> &mut ChannelOwner {
        &mut self.channel
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

// Helper function to generate a GUID (simplified for now)
fn generate_guid() -> Str<Guid> {
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(1);
    let id = COUNTER.fetch_add(1, Ordering::SeqCst);
    let guid_str = format!("locator-{}", id);
    Str::validate(guid_str).unwrap()
}