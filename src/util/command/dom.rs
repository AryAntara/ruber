use fantoccini::{Client, Locator};
use std::fs;

async fn exec_js(page: &Client, script: &str, desc: &str) {
    match page.execute(script, vec![]).await {
        Ok(_) => println!("{}", desc),
        Err(e) => println!("Error during {}: {:?}", desc, e),
    };
}

pub async fn click(page: &Client, selector: String) {
    exec_js(
        page,
        &format!("document.querySelector('{selector}').click()"),
        &format!("Clicking {selector}"),
    )
    .await;
}

pub async fn fill(page: &Client, selector: String, value: String) {
    exec_js(
        page,
        &format!("document.querySelector('{selector}').value = '{value}'"),
        &format!("Filling {selector} with {value}"),
    )
    .await;
}

pub async fn clear(page: &Client, selector: String) {
    exec_js(
        page,
        &format!("document.querySelector('{selector}').value = ''"),
        &format!("Clearing {selector}"),
    )
    .await;
}

pub async fn hover(page: &Client, selector: String) {
    exec_js(
        page,
        &format!(
            "let el = document.querySelector('{selector}'); if(el) el.dispatchEvent(new MouseEvent('mouseover', {{ bubbles: true }}))"
        ),
        &format!("Hovering over {selector}"),
    )
    .await;
}

pub async fn scroll_to(page: &Client, selector: String) {
    exec_js(
        page,
        &format!("document.querySelector('{selector}').scrollIntoView({{ behavior: 'smooth' }})"),
        &format!("Scrolling to {selector}"),
    )
    .await;
}

pub async fn trigger_event(page: &Client, event_name: String, selector: String) {
    exec_js(
        page,
        &format!(
            "let el = document.querySelector('{selector}'); if(el) el.dispatchEvent(new Event('{event_name}', {{ bubbles: true }}))"
        ),
        &format!("Triggering {event_name} on {selector}"),
    )
    .await;
}

pub async fn select_first(page: &Client, selector: String) {
    exec_js(
        page,
        &format!(
            "let el = document.querySelector('{selector}'); if(el) {{ let opt = el.querySelector('option, .options'); if(opt) opt.click() || (el.value = opt.value); el.dispatchEvent(new Event('change', {{ bubbles: true }})) }}"
        ),
        &format!("Selecting first option for {selector}"),
    )
    .await;
}

pub async fn wait_for_element(page: &Client, selector: String) {
    match page.wait().for_element(Locator::Css(&selector)).await {
        Ok(_) => println!("✅ Element appeared: {selector}"),
        Err(e) => println!("❌ Element timed out: {selector} - {:?}", e),
    }
}

pub async fn screenshot(page: &Client, name: String) {
    match page.screenshot().await {
        Ok(png) => {
            let filename = if name.ends_with(".png") { name } else { format!("{}.png", name) };
            if let Err(e) = fs::write(&filename, png) {
                println!("❌ Failed to save screenshot: {:?}", e);
            } else {
                println!("📸 Screenshot saved as: {}", filename);
            }
        }
        Err(e) => println!("❌ Failed to take screenshot: {:?}", e),
    }
}

pub async fn assert_text(page: &Client, expected: String, selector: String) {
    let script = format!("return document.querySelector('{selector}')?.innerText || ''");
    match page.execute(&script, vec![]).await {
        Ok(val) => {
            let actual = val.as_str().unwrap_or("");
            if actual.contains(&expected) {
                println!("✅ Assertion passed: '{}' found in {}", expected, selector);
            } else {
                println!("❌ Assertion failed: expected '{}', found '{}' in {}", expected, actual, selector);
            }
        }
        Err(e) => println!("❌ Assertion error on {}: {:?}", selector, e),
    }
}

pub async fn simulate_keyinput(_page: &Client, _value: String) {
    // Placeholder for future implementation
}
