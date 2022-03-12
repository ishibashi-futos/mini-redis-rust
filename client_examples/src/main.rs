use mini_redis::{client, Result};
use tokio::sync::mpsc;
use tokio::sync::oneshot;
extern crate client_examples;
use client_examples::Command::*;

#[tokio::main]
pub async fn main() -> Result<()> {
    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();
    let manager = tokio::spawn(async move {
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();

        while let Some(cmd) = rx.recv().await {
            match cmd {
                Get { key, response } => {
                    let res = client.get(&key).await;
                    let _ = response.send(res);
                },
                Set { key, value, response } => {
                    let res = client.set(&key, value).await;
                    let _ = response.send(res);
                }
            }
        }
    });

    let t1 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd  = Get {
            key: String::from("hello"),
            response: resp_tx,
        };
        tx.send(cmd).await.unwrap();

        let res = resp_rx.await;
        println!("GOT = {:?}", res);
    });

    let t2 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd  = Set {
            key: String::from("foo"),
            value: "bar".into(),
            response: resp_tx,
        };
        tx2.send(cmd).await.unwrap();
        let res = resp_rx.await;
        println!("GOT = {:?}", res);
    });

    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();

    Ok(())
}
