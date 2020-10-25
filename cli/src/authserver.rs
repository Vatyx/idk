use std::collections::HashMap;

use tokio::sync::mpsc;
use warp::Filter;

const BODY: &str = "<!DOCTYPE html>
<html>
<head></head>
<body>
Successfully Logged In.
<script>
    window.location=\"https://app.idkcli.com\"
</script>
</body>
</html>";

pub async fn start_server() -> String {
    let cors = warp::cors().allow_any_origin();

    let (signal_sender, mut signal_receiver) = mpsc::channel::<String>(1);
    let (data_sender, mut data_receiver) = mpsc::channel::<String>(1);

    let with_signal = warp::any().map(move || signal_sender.clone());
    let with_data = warp::any().map(move || data_sender.clone());

    let routes = warp::any()
        .and(warp::path("auth"))
        .and(warp::query::<HashMap<String, String>>())
        .and(with_signal)
        .and(with_data)
        .and_then(
            move |p: HashMap<String, String>, tx: mpsc::Sender<String>, dx: mpsc::Sender<String>| async move {
                if let Some(key) = p.get("token") {
                    tx.clone().send("".to_string()).await.ok();
                    dx.clone().send(key.clone()).await.ok();
                }

                Ok::<_, warp::reject::Rejection>(warp::reply::html(
                    BODY.to_string(),
                ))
            },
        )
        .with(cors);

    let (_addr, server) =
        warp::serve(routes).bind_with_graceful_shutdown(([127, 0, 0, 1], 8765), async move {
            signal_receiver.recv().await;
        });

    async {
        tokio::task::spawn(server);
        data_receiver.recv().await.unwrap()
    }
    .await
}
