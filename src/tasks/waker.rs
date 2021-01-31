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

use alloc::{sync::Arc, task::Wake};
use core::task::Waker;

use crossbeam_queue::SegQueue;

use super::TaskId;

pub struct TaskWaker {
    task_id: TaskId,
    task_queue: Arc<SegQueue<TaskId>>,
}

impl TaskWaker {
    pub fn new(task_id: TaskId, task_queue: Arc<SegQueue<TaskId>>) -> Waker {
        Waker::from(Arc::new(TaskWaker { task_id, task_queue }))
    }

    pub fn wake_task(&self) {
        self.task_queue.push(self.task_id);
    }
}

impl Wake for TaskWaker {
    fn wake(self: Arc<Self>) {
        self.wake_task();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        self.wake_task();
    }
}
