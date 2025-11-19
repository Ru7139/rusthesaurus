#[rustfmt::skip]
#[ignore] // use it manually
#[tokio::test]
async fn fetch_html_from_servo() {
    let servo_url = "https://servo.org/";
    let reqwest_client = reqwest::Client::new(); // 自带Arc，用clone即可

    if let Ok(html_file) = reqwest_client.get(servo_url).send().await.unwrap().text().await {
        tokio::fs::write("src/test_of_those/servo.html", html_file).await.unwrap(); }
    else {
        panic!("Failed to download html from https://servo.org/")
    }
}
