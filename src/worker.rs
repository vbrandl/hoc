use crate::{cache::HocParams, hoc::hoc, http::AppState};

use std::{
    hash::Hash,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

use crossbeam_queue::SegQueue;
use dashmap::DashSet;
use tokio::sync::Notify;
use tracing::{error, trace};

pub(crate) struct Queue<T> {
    tasks: SegQueue<T>,
    uniqueness: DashSet<T>,
    notify: Notify,
    active: AtomicBool,
}

impl<T: Hash + Eq + Clone> Queue<T> {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn push(&self, value: T) -> bool {
        if self.active.load(Ordering::SeqCst) && self.uniqueness.insert(value.clone()) {
            self.tasks.push(value);
            self.notify.notify_one();
            true
        } else {
            false
        }
    }

    async fn pop(&self) -> Option<T> {
        loop {
            if let Some(value) = self.tasks.pop() {
                self.uniqueness.remove(&value);
                break Some(value);
            } else if self.active.load(Ordering::SeqCst) {
                self.notify.notified().await;
            } else {
                break None;
            }
        }
    }

    #[cfg(test)]
    fn close(&self) {
        self.active.store(false, Ordering::SeqCst);
        // wake up all waiting workers
        self.notify.notify_waiters();
    }
}

impl<T: Hash + Eq + Clone> Default for Queue<T> {
    fn default() -> Self {
        Self {
            tasks: SegQueue::new(),
            uniqueness: DashSet::new(),
            notify: Notify::new(),
            active: AtomicBool::new(true),
        }
    }
}

pub(crate) async fn worker(state: Arc<AppState>, queue: Arc<Queue<HocParams>>) {
    while let Some(task) = queue.pop().await {
        trace!(?task, "handling hoc calculation");

        if let Err(err) = hoc(&task, &state).await {
            error!(?task, %err, "error calculating hoc");
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::Queue;

    #[tokio::test]
    async fn empty_queue() {
        let queue = Arc::new(Queue::<i32>::new());
        let handle = {
            let queue = queue.clone();
            tokio::spawn(async move { queue.pop().await })
        };
        queue.close();
        let result = handle.await.unwrap();
        assert_eq!(result, None);
    }

    #[tokio::test]
    async fn push_single() {
        let queue = Arc::new(Queue::new());
        queue.push(1);
        let result = {
            let queue = queue.clone();
            tokio::spawn(async move { queue.pop().await })
        };
        queue.close();
        assert_eq!(result.await.unwrap(), Some(1));
    }

    #[tokio::test]
    async fn push_multiple_different() {
        let queue = Arc::new(Queue::new());

        queue.push(1);
        let queued = queue.push(2);
        assert!(queued);

        let result = {
            let queue = queue.clone();
            tokio::spawn(async move { queue.pop().await })
        };
        assert_eq!(result.await.unwrap(), Some(1));

        let result = {
            let queue = queue.clone();
            tokio::spawn(async move { queue.pop().await })
        };
        queue.close();
        assert_eq!(result.await.unwrap(), Some(2));
    }

    #[tokio::test]
    async fn push_multiple_duplicate() {
        let queue = Arc::new(Queue::new());

        queue.push(1);
        let queued = queue.push(1);
        assert!(!queued);

        let result = {
            let queue = queue.clone();
            tokio::spawn(async move { queue.pop().await })
        };
        assert_eq!(result.await.unwrap(), Some(1));

        let result = {
            let queue = queue.clone();
            tokio::spawn(async move { queue.pop().await })
        };
        queue.close();
        assert_eq!(result.await.unwrap(), None);
    }
}
