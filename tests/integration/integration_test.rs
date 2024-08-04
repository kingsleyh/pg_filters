use crate::integration::client;

#[tokio::test]
async fn test_something_works() {
    let client = client().await;

    let rows = client
        .query("SELECT $1::TEXT", &[&"hello world"])
        .await.unwrap();

    // And then check that we got back the same string we sent over.
    let value: &str = rows[0].get(0);
    assert_eq!(value, "hello world");
}



#[tokio::test]
async fn test_something_works2() {
    let client = client().await;

    let rows = client
        .query("SELECT $1::TEXT", &[&"hello world"])
        .await.unwrap();

    // And then check that we got back the same string we sent over.
    let value: &str = rows[0].get(0);
    assert_eq!(value, "hello world");
}

