mod https_cert_task;

use actix::prelude::*;
use cron::Schedule;
use std::str::FromStr;
use chrono::Utc;
use std::sync::Arc;
use std::error::Error;

pub use self::https_cert_task::HttpsCertTask;

pub trait Task {
    fn run(&self);
}

pub struct Scheduler {
    tasks: Vec<(Arc<Schedule>, Arc<Box<dyn Task>>)>,
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
        let scheule = Schedule::from_str(cron_expression)?;

        self.tasks.push((Arc::new(scheule), Arc::new(task)));

        Ok(())
    }

    fn start_task(&self, ctx: &mut Context<Self>, schedule: Arc<Schedule>, task: Arc<Box<dyn Task>>) {
        if let Some(when) = schedule.upcoming(Utc).next() {
            let duration = when - Utc::now();
            let duration = duration.to_std().unwrap();
            let schedule = schedule.clone();
            let task = task.clone();
            ctx.run_later(duration, move |this: &mut Self, ctx: &mut Context<Self>| {
                task.run();
                this.start_task(ctx, schedule, task.clone());
            });
        }
    }
}
