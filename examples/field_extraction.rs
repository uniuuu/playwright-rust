use playwright::{Error, Playwright};

#[derive(Debug, Clone)]
pub struct FormField {
    pub name: String,
    pub id: String,
    pub class: String,
    pub placeholder: String,
    pub field_type: String,
    pub visible: bool,
    pub enabled: bool,
    pub editable: bool,
    pub current_value: String,
    pub tag_name: String,
    pub selector: String,
}

impl Default for FormField {
    fn default() -> Self {
        Self {
            name: String::new(),
            id: String::new(),
            class: String::new(),
            placeholder: String::new(),
            field_type: String::new(),
            visible: true,
            enabled: true,
            editable: true,
            current_value: String::new(),
            tag_name: String::new(),
            selector: String::new(),
        }
    }
}

pub struct FieldExtractor;

impl FieldExtractor {
    /// Extract form fields using native Playwright API calls
    /// This replaces the JavaScript DOM extraction approach
    pub async fn extract_fields_native(
        page: &playwright::api::Page,
    ) -> Result<Vec<FormField>, Error> {
        // Create locator for all form input elements
        let input_locator = page.locator("input, select, textarea").await?;
        let count = input_locator.count().await?;

        let mut fields = Vec::with_capacity(count);

        // Iterate through each element using nth() approach
        for i in 0..count {
            let field_locator = input_locator.nth(i as i32).await?;

            // Extract all attributes using get_attribute()
            let name = field_locator
                .get_attribute("name", None)
                .await?
                .unwrap_or_default();
            let id = field_locator
                .get_attribute("id", None)
                .await?
                .unwrap_or_default();
            let class = field_locator
                .get_attribute("class", None)
                .await?
                .unwrap_or_default();
            let placeholder = field_locator
                .get_attribute("placeholder", None)
                .await?
                .unwrap_or_default();
            let field_type = field_locator
                .get_attribute("type", None)
                .await?
                .unwrap_or_default();

            // Get element state using is_* methods
            let visible = field_locator.is_visible(None).await.unwrap_or(false);
            let enabled = field_locator.is_enabled(None).await.unwrap_or(false);
            let editable = field_locator.is_editable(None).await.unwrap_or(false);

            // Get current value using input_value()
            let current_value = field_locator.input_value(None).await.unwrap_or_default();

            // Get tag name (simplified for this example)
            let tag_name = "input".to_string();

            // Store the selector for reference
            let selector = field_locator.selector()?;

            fields.push(FormField {
                name,
                id,
                class,
                placeholder,
                field_type,
                visible,
                enabled,
                editable,
                current_value,
                tag_name,
                selector,
            });
        }

        Ok(fields)
    }

    /// Alternative approach using the new all() method
    pub async fn extract_fields_with_all(
        page: &playwright::api::Page,
    ) -> Result<Vec<FormField>, Error> {
        // Create locator for all form input elements
        let input_locator = page.locator("input, select, textarea").await?;
        let all_locators = input_locator.all().await?;

        let mut fields = Vec::with_capacity(all_locators.len());

        // Process each locator
        for field_locator in all_locators {
            let name = field_locator
                .get_attribute("name", None)
                .await?
                .unwrap_or_default();
            let id = field_locator
                .get_attribute("id", None)
                .await?
                .unwrap_or_default();
            let class = field_locator
                .get_attribute("class", None)
                .await?
                .unwrap_or_default();
            let placeholder = field_locator
                .get_attribute("placeholder", None)
                .await?
                .unwrap_or_default();
            let field_type = field_locator
                .get_attribute("type", None)
                .await?
                .unwrap_or_default();

            let visible = field_locator.is_visible(None).await.unwrap_or(false);
            let enabled = field_locator.is_enabled(None).await.unwrap_or(false);
            let editable = field_locator.is_editable(None).await.unwrap_or(false);
            let current_value = field_locator.input_value(None).await.unwrap_or_default();
            let selector = field_locator.selector()?;

            fields.push(FormField {
                name,
                id,
                class,
                placeholder,
                field_type,
                visible,
                enabled,
                editable,
                current_value,
                tag_name: "input".to_string(), // Simplified for this example
                selector,
            });
        }

        Ok(fields)
    }

    /// Advanced field extraction with filtering and validation
    pub async fn extract_filtered_fields(
        page: &playwright::api::Page,
    ) -> Result<Vec<FormField>, Error> {
        // Get only visible, enabled input fields
        let input_locator = page
            .locator("input:visible, select:visible, textarea:visible")
            .await?;
        let visible_fields = input_locator.all().await?;

        let mut fields = Vec::new();

        for field_locator in visible_fields {
            // Only process if enabled and editable
            let enabled = field_locator.is_enabled(None).await.unwrap_or(false);
            let editable = field_locator.is_editable(None).await.unwrap_or(false);

            if !enabled || !editable {
                continue; // Skip disabled or non-editable fields
            }

            // Extract field information
            let name = field_locator
                .get_attribute("name", None)
                .await?
                .unwrap_or_default();
            let id = field_locator
                .get_attribute("id", None)
                .await?
                .unwrap_or_default();
            let placeholder = field_locator
                .get_attribute("placeholder", None)
                .await?
                .unwrap_or_default();
            let field_type = field_locator
                .get_attribute("type", None)
                .await?
                .unwrap_or("text".to_string());
            let current_value = field_locator.input_value(None).await.unwrap_or_default();
            let selector = field_locator.selector()?;

            // Skip fields that already have values (optional)
            if !current_value.is_empty() {
                continue; // Skip pre-filled fields
            }

            fields.push(FormField {
                name,
                id,
                class: String::new(),
                placeholder,
                field_type,
                visible: true,
                enabled: true,
                editable: true,
                current_value,
                tag_name: "input".to_string(),
                selector,
            });
        }

        Ok(fields)
    }
}

/// Main function for running the example
#[tokio::main]
async fn main() -> Result<(), Error> {
    demonstrate_field_extraction().await
}

/// Example usage function
pub async fn demonstrate_field_extraction() -> Result<(), Error> {
    let playwright = Playwright::initialize().await?;
    playwright.prepare()?;

    let chromium = playwright.chromium();
    let browser = chromium.launcher().headless(true).launch().await?;
    let context = browser.context_builder().build().await?;
    let page = context.new_page().await?;

    // Navigate to a test page
    let html = r#"
    <html>
    <body>
        <form>
            <input name="username" id="user" placeholder="Enter username" type="text" />
            <input name="password" id="pass" placeholder="Enter password" type="password" />
            <input name="email" id="email" placeholder="Enter email" type="email" value="pre-filled@example.com" />
            <select name="country" id="country">
                <option value="us">United States</option>
                <option value="uk">United Kingdom</option>
            </select>
            <textarea name="comments" placeholder="Your comments"></textarea>
            <input name="hidden" type="hidden" value="hidden_value" />
        </form>
    </body>
    </html>
    "#;

    page.goto_builder(&format!("data:text/html,{}", html))
        .goto()
        .await?;

    // Method 1: Using nth() iteration
    println!("=== Method 1: Using nth() iteration ===");
    let fields1 = FieldExtractor::extract_fields_native(&page).await?;
    for field in &fields1 {
        println!(
            "Field: {} ({}), visible: {}, enabled: {}, value: '{}'",
            field.name, field.field_type, field.visible, field.enabled, field.current_value
        );
    }

    // Method 2: Using all() method
    println!("\n=== Method 2: Using all() method ===");
    let fields2 = FieldExtractor::extract_fields_with_all(&page).await?;
    for field in &fields2 {
        println!(
            "Field: {} ({}), selector: {}",
            field.name, field.field_type, field.selector
        );
    }

    // Method 3: Filtered extraction
    println!("\n=== Method 3: Filtered extraction ===");
    let fields3 = FieldExtractor::extract_filtered_fields(&page).await?;
    for field in &fields3 {
        println!(
            "Fillable field: {} ({}), placeholder: '{}'",
            field.name, field.field_type, field.placeholder
        );
    }

    println!("\n✅ Field extraction demonstration complete!");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_field_extraction_methods() {
        // This test demonstrates the API usage patterns
        // In a real scenario, you would set up a browser and test page
        println!("Field extraction methods are ready for testing");

        // The methods are available for integration testing:
        // - FieldExtractor::extract_fields_native()
        // - FieldExtractor::extract_fields_with_all()
        // - FieldExtractor::extract_filtered_fields()
    }
}
