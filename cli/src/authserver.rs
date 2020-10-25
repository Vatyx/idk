use warp::Filter;
use tokio::sync::mpsc;

pub async fn start_server() -> String {

    let cors = warp::cors().allow_any_origin();

    let (signal_sender, mut signal_receiver) = mpsc::channel::<String>(1);
    let (data_sender, mut data_receiver) = mpsc::channel::<String>(1);

    let with_signal = warp::any().map(move || signal_sender.clone());
    let with_data = warp::any().map(move || data_sender.clone());

    let routes = warp::any().and(warp::path::param()).and(with_signal).and(with_data).and_then(move |name: String, tx: mpsc::Sender<String>, dx: mpsc::Sender<String>| async move { 
        tx.clone().send("".to_string()).await.ok();
        dx.clone().send(name).await.ok();
        Ok::<_, warp::reject::Rejection>("<html><script>window.location=\"https://app.idkcli.com\"</script></html>")
    }).with(cors);
    
        // dataSender.send(name);

    let (_addr, server) = warp::serve(routes).bind_with_graceful_shutdown(([127, 0, 0, 1], 8765), async move {
        signal_receiver.recv().await.unwrap();
    });

    async {
        tokio::task::spawn(server).await.ok();
        data_receiver.recv().await.unwrap()
    }.await
}
