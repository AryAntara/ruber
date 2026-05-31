use fantoccini::Client;
use serde_json::Value;
use std::fs;
use std::time::Duration;
use tokio::time::sleep;

const RECORDER_JS: &str = r#"
(function() {
    if (window._ruber_initialized) return;
    window._ruber_events = [];
    window._ruber_initialized = true;

    const getSelector = (el) => {
        if (!el || el.nodeType !== Node.ELEMENT_NODE) return 'body';
        if (el.id) return `#${el.id}`;
        
        const attrs = ['name', 'data-testid', 'placeholder', 'aria-label'];
        for (let attr of attrs) {
            let val = el.getAttribute(attr);
            if (val) return `[${attr}="${val}"]`;
        }

        let selector = el.tagName.toLowerCase();
        if (el.className && typeof el.className === 'string') {
            const cls = el.className.split(/\s+/).filter(c => c && !c.includes(':')).join('.');
            if (cls) selector += `.${cls}`;
        }
        
        if (document.querySelectorAll(selector).length > 1 && el.parentElement) {
            const idx = Array.from(el.parentElement.children).indexOf(el) + 1;
            return `${getSelector(el.parentElement)} > ${el.tagName.toLowerCase()}:nth-child(${idx})`;
        }
        return selector;
    };

    const record = (type, el, extra = {}) => {
        window._ruber_events.push({ type, selector: getSelector(el), ...extra });
    };

    document.addEventListener('click', (e) => {
        const el = e.target.closest('a, button, input[type="button"], input[type="submit"]') || e.target;
        record('click', el);
    }, true);

    document.addEventListener('change', (e) => {
        if (['INPUT', 'TEXTAREA', 'SELECT'].includes(e.target.tagName)) {
            record('fill', e.target, { value: e.target.value });
        }
    }, true);
})();
"#;

pub async fn start_recording(client: Client, output_file: String) {
    let mut recorded = Vec::new();
    let mut last_url = String::new();

    println!("🔴 Recording to: {}", output_file);

    loop {
        let current_url = match client.current_url().await {
            Ok(url) => url.to_string(),
            Err(_) => break,
        };

        if current_url != last_url {
            recorded.push(format!("go to {}", current_url));
            last_url = current_url;
            sleep(Duration::from_millis(500)).await;
            let _ = client.execute(RECORDER_JS, vec![]).await;
        } else {
            let _ = client.execute(RECORDER_JS, vec![]).await;
        }

        if let Ok(Value::Array(evs)) = client.execute("const e = window._ruber_events || []; window._ruber_events = []; return e;", vec![]).await {
            for ev in evs {
                let sel = ev["selector"].as_str().unwrap_or("");
                match ev["type"].as_str().unwrap_or("") {
                    "click" => recorded.push(format!("click element '{}'", sel)),
                    "fill" => recorded.push(format!("fill element '{}' with '{}'", sel, ev["value"].as_str().unwrap_or(""))),
                    _ => {}
                }
                println!("  ✨ {}", recorded.last().unwrap());
            }
        }

        if !recorded.is_empty() {
            let _ = fs::write(&output_file, recorded.join("\n") + "\nhang up");
        }
        sleep(Duration::from_millis(300)).await;
    }
    println!("🏁 Recording stopped.");
}
