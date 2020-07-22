use crossbeam_channel::{unbounded, Receiver, Sender};

lazy_static::lazy_static! {
    pub static ref EMIT_CHANNEL: (Sender<String>, Receiver<String>) = unbounded();
}

#[macro_export]
macro_rules! emit_crash {
    ($msg:expr) => ({
        crate::EMIT_CHANNEL.0.send($msg.to_owned()).expect("carsh sender is drop");
        panic!($msg)
    });
    // ($msg:expr,) => ({ $crate::panic!($msg) });
    ($fmt:expr, $($arg:tt)+) => ({
        let msg = format!($fmt, $($arg)+);
        crate::EMIT_CHANNEL.0.send((&msg).to_string()).expect("carsh sender is drop");
        panic!((&msg).to_string())
    });
}

pub fn listen_main_thread() {
    let panic_str = EMIT_CHANNEL.1.recv().unwrap();

    panic!(
        "main thread exit, because a child thread crashed \npanic message:\n{:?}",
        panic_str
    );
}

#[cfg(test)]
mod tests {
    use super::emit_crash;
    use super::listen_main_thread;

    #[test]
    fn test_msg() {
        std::thread::spawn(move || {
            emit_crash!("test carsh");
        });

        listen_main_thread();

        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_msg_format() {
        std::thread::spawn(move || {
            emit_crash!("test carsh {:?}", 111);
        });

        listen_main_thread();

        assert_eq!(2 + 2, 4);
    }
}
