use actix::prelude::*;
use cron::Schedule as CronSchedule;
use std::str::FromStr;
use chrono::Utc;
use std::sync::Arc;
use std::error::Error;

pub mod https_cert_task;

pub enum TaskResult {
    Continue,
    Stop,
}

pub trait Task {
    fn run(&self) -> TaskResult;
}

pub struct Scheduler {
    tasks: Vec<(Arc<CronSchedule>, Arc<Box<dyn Task>>)>,
}

impl Actor for Scheduler {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        for (schedule, task) in self.tasks.iter() {
            let schedule = schedule.clone();
            let task = task.clone();
            self.start_task(ctx, schedule, task);
        }
    }
}

impl Scheduler {
    pub fn new() -> Self {
        Self { tasks: vec![] }
    }

    pub fn add_task(&mut self, cron_expression: &str, task: Box<dyn Task>) -> Result<(), Box<dyn Error>> {
        let schedule = CronSchedule::from_str(cron_expression)?;

        self.tasks.push((Arc::new(schedule), Arc::new(task)));

        Ok(())
    }

    fn start_task(&self, ctx: &mut Context<Self>, schedule: Arc<CronSchedule>, task: Arc<Box<dyn Task>>) {
        if let Some(when) = schedule.upcoming(Utc).next() {
            let duration = when - Utc::now();
            let duration = duration.to_std().unwrap();
            let schedule = schedule.clone();
            let task = task.clone();
            ctx.run_later(duration, move |this: &mut Self, ctx: &mut Context<Self>| {
                match task.run() {
                    TaskResult::Continue => this.start_task(ctx, schedule, task.clone()),
                    TaskResult::Stop => ()
                }
            });
        }
    }
}
