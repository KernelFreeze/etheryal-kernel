// Copyright 2021 Miguel Peláez <kernelfreeze@outlook.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use alloc::{collections::BTreeMap, sync::Arc};
use core::task::{Context, Poll, Waker};

use crossbeam_queue::SegQueue;
use futures::Future;

use super::{waker::TaskWaker, Task, TaskId};

pub struct TaskExecutor {
    tasks: BTreeMap<TaskId, Task>,
    task_queue: Arc<SegQueue<TaskId>>,
    waker_cache: BTreeMap<TaskId, Waker>,
}

impl TaskExecutor {
    pub fn new() -> Self {
        TaskExecutor {
            tasks: BTreeMap::new(),
            task_queue: Arc::new(SegQueue::new()),
            waker_cache: BTreeMap::new(),
        }
    }

    pub fn spawn(&mut self, future: impl Future<Output = ()> + 'static) {
        self.spawn_task(Task::new(future))
    }

    pub fn spawn_task(&mut self, task: Task) {
        let task_id = task.id;

        if self.tasks.insert(task.id, task).is_some() {
            panic!("task with same ID already in tasks");
        }
        self.task_queue.push(task_id);
    }

    pub fn run(&mut self) -> ! {
        loop {
            self.run_ready_tasks();
            self.sleep_if_idle();
        }
    }

    fn run_ready_tasks(&mut self) {
        let tasks = &mut self.tasks;
        let task_queue = &mut self.task_queue;
        let waker_cache = &mut self.waker_cache;

        while let Some(task_id) = task_queue.pop() {
            let task = match tasks.get_mut(&task_id) {
                Some(task) => task,
                None => continue,
            };

            let waker = waker_cache
                .entry(task_id)
                .or_insert_with(|| TaskWaker::new(task_id, task_queue.clone()));
            let mut context = Context::from_waker(waker);

            if let Poll::Ready(_) = task.poll(&mut context) {
                // task done -> remove it and its cached waker
                tasks.remove(&task_id);
                waker_cache.remove(&task_id);
            }
        }
    }

    fn sleep_if_idle(&self) {
        if self.task_queue.is_empty() {
            crate::platform::halt::halt_cpu();
        }
    }
}
