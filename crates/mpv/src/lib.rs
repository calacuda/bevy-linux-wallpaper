use std::fmt::Display;
// use tokio::sync::mpsc::UnboundedSender;

// pub mod connection;
// pub mod stream_manager;

// pub fn spawn<F, E>(future: F)
// where
//     F: Future<Output = Result<(), E>> + Send + 'static,
//     E: Display,
// {
//     tokio::task::spawn(async {
//         if let Err(error) = future.await {
//             eprintln!("{}", error);
//         }
//     });
// }
//
// /// Sends a message over an unbounded receiver and returns true if the message was sent
// /// or false if the channel has been closed.
// pub fn send<T>(sender: &UnboundedSender<T>, message: T) -> bool {
//     match sender.send(message) {
//         Ok(_) => true,
//         Err(_) => false,
//     }
// }
