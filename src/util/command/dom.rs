use fantoccini::Client;
pub async fn click(page: &Client, selector: String) {
    let _ = match page
        .execute(
            &format!("document.querySelector('{selector}').click()"),
            vec![],
        )
        .await
    {
        Ok(d) => print!("{d:?}"),
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
        Ok(data) => println!("{data:?}"),
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
