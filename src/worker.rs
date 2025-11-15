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
    uniquenes: DashSet<T>,
    notify: Notify,
    active: AtomicBool,
}

impl<T: Hash + Eq + Clone> Queue<T> {
    pub(crate) fn new() -> Self {
        Self {
            tasks: SegQueue::new(),
            uniquenes: DashSet::new(),
            notify: Notify::new(),
            active: AtomicBool::new(true),
        }
    }

    pub(crate) fn push(&self, value: T) -> bool {
        if self.active.load(Ordering::SeqCst) && self.uniquenes.insert(value.clone()) {
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
                self.uniquenes.remove(&value);
                break Some(value);
            } else if self.active.load(Ordering::SeqCst) {
                self.notify.notified().await;
            } else {
                break None;
            }
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
