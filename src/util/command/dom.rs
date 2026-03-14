use fantoccini::Client;
pub async fn click(page: &Client, selector: String) {
    let _ = match page
        .execute(
            &format!("document.querySelector('{selector}').click()"),
            vec![],
        )
        .await
    {
        Ok(_) => println!("Clicking {selector}"),
        Err(e) => println!("{e:?}"),
    };
}

pub async fn fill(page: &Client, selector: String, value: String) {
    let _ = match page
        .execute(
            &format!("document.querySelector('{selector}').value = '{value}'"),
            vec![],
        )
        .await
    {
        Ok(_) => println!("Filling {selector} with {value}"),
        Err(err) => println!("{err:?}"),
    };
}

pub async fn trigger_event(page: &Client, event_name: String, selector: String) {
    let _ = match page
        .execute(
            &format!(
                "
let element = document.querySelector('{selector}');
element.dispatchEvent(new Event('{event_name}'));
"
            ),
            vec![],
        )
        .await
    {
        Ok(data) => println!("{data:?}"),
        Err(err) => println!("{err:?}"),
    };
}

pub async fn select_first(page: &Client, selector: String) {
    let _ = match page
        .execute(
            &format!(
                "
let element = document.querySelector('{selector}');
let opts = element.querySelectorAll('.options');
opts[0].click();
"
            ),
            vec![],
        )
        .await
    {
        Ok(data) => println!("Selecting the first option for {selector}"),
        Err(err) => println!("{err:?}"),
    };
}

pub async fn simulate_keyinput(page: &Client, value: String) {}
