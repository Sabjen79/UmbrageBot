use std::{future::Future, pin::Pin, sync::Arc, time::Duration};

use tokio::runtime::Handle;
use tokio::sync::Notify;

use tokio::sync::Mutex;
use tokio::time::Instant;

use crate::timer_manager;

// This is the best method I found for passing async closures as struct fields
// It looks bad but it works

pub type AsyncClosure<T> = Box<
    dyn FnMut() -> 
        Pin<Box<dyn Future<Output = T> + Send>>
        + Send 
        + Sync
>;

type Action = Arc<Mutex<AsyncClosure<()>>>;
type DurationHandler = Arc<Mutex<AsyncClosure<Duration>>>;

//============================================================================

pub struct TimerBuilder {
    name: String,
    action: Option<Action>,
    duration_handler: Option<DurationHandler>
}

impl TimerBuilder {
    pub(in crate::timer_manager)
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            action: None,
            duration_handler:None
        }
    }

    pub fn action<F, Fut>(&mut self, mut action: F) -> &mut Self
    where
        F: FnMut() -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.action = Some(Arc::new(Mutex::new(Box::new(move || Box::pin(action())))));

        return self;
    }

    pub fn duration_handler<F, Fut>(&mut self, mut action: F) -> &mut Self
    where
        F: FnMut() -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Duration> + Send + 'static,
    {
        self.duration_handler = Some(Arc::new(Mutex::new(Box::new(move || Box::pin(action())))));

        return self;
    }

    pub fn build_and_register(&self) -> Arc<Timer> {
        if self.action.is_none() {
            panic!("Cannot create timer without action");
        }

        if self.duration_handler.is_none()  {
            panic!("Cannot create timer without duration_handler");
        }

        let timer = Arc::new(Timer::new(
            self.action.as_ref().unwrap().clone(), 
            self.duration_handler.as_ref().unwrap().clone()
        ));

        tokio::task::block_in_place(|| {
            Handle::current().block_on(async {
                timer_manager::register_timer(self.name.as_str(), timer.clone()).await;
            });
        });
        
        return timer
    } 
}

//============================================================================

pub struct Timer
{
    action: Action,
    deadline: Arc<Mutex<Instant>>,
    duration_handler: DurationHandler,
    notify: Arc<Notify>,
}

impl Timer
{
    pub(in crate::timer_manager)
    fn new(action: Action, duration_handler: DurationHandler) -> Self {
        Self {
            action: action,
            deadline: Arc::new(Mutex::new(Instant::now())),
            duration_handler: duration_handler,
            notify: Arc::new(Notify::new())
        }
    }

    pub fn start(&self) {
        let notify = self.notify.clone();
        let action = self.action.clone();
        let duration_action = self.duration_handler.clone();
        let deadline = self.deadline.clone();

        tokio::spawn(async move {
            tokio::select! {
                _ = async {
                    loop {
                        let deadline = {
                            *deadline.lock().await = {
                                let mut duration_action = duration_action.lock().await;

                                let duration = (*duration_action)().await;

                                Instant::now() + duration
                            };

                            let d = *deadline.lock().await;

                            d.clone()
                        };
                        
                        tokio::time::sleep_until(deadline).await;

                        let action = action.clone();
                        tokio::spawn(async move {
                            let mut action = action.lock().await;
                            (action)().await;
                        });
                    }
                } => (),
                _ = notify.notified() => ()
            }
        });
    }

    /// Returns the remaining time until the timer runs
    pub async fn get_time_left(&self) -> u128 {
        let deadline = {
            let lock = self.deadline.lock().await;

            lock.clone()
        };

        deadline.duration_since(Instant::now()).as_millis()
    }

    pub fn run_early(&self) {
        self.cancel();

        let action = self.action.clone();

        tokio::spawn(async move {
            let mut action = action.lock().await;
            (action)().await;
        });

        self.start();
    }

    pub fn reset(&self) {
        self.cancel();

        self.start();
    }

    pub fn cancel(&self) {
        self.notify.notify_waiters();
    }
}