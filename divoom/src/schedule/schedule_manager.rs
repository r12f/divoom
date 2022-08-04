use crate::dsl::{DivoomDslOperation, DivoomDslParser, DivoomDslRunner};
use crate::schedule::schedule_config::*;
use crate::{DivoomAPIResult, PixooClient};
use log::error;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tokio_cron_scheduler::{Job, JobScheduler};

#[cfg(feature = "animation-builder")]
use crate::DivoomAnimationTemplateManager;

pub struct DivoomScheduledJob {
    cron: String,
    operations: Vec<DivoomDslOperation>,
}

pub struct DivoomScheduleManager {
    device_address: String,
    jobs: Vec<Arc<DivoomScheduledJob>>,
    job_scheduler: JobScheduler,

    #[cfg(feature = "animation-builder")]
    template_manager: Arc<DivoomAnimationTemplateManager>,
}

impl DivoomScheduleManager {
    #[cfg(feature = "animation-builder")]
    pub fn from_config(
        device_address: String,
        schedules: Vec<DivoomScheduleConfigCronJob>,
        template_manager: Arc<DivoomAnimationTemplateManager>,
    ) -> DivoomAPIResult<Self> {
        let mut jobs: Vec<Arc<DivoomScheduledJob>> = Vec::new();

        for schedule in schedules {
            let parsed_operations: DivoomAPIResult<Vec<DivoomDslOperation>> = schedule
                .operations
                .iter()
                .map(|x| DivoomDslParser::parse(x))
                .collect();
            jobs.push(Arc::new(DivoomScheduledJob {
                cron: schedule.cron,
                operations: parsed_operations?,
            }));
        }

        Ok(DivoomScheduleManager {
            device_address,
            jobs,
            job_scheduler: JobScheduler::new().unwrap(),
            template_manager,
        })
    }

    #[cfg(feature = "animation-builder")]
    pub fn start(&mut self) {
        for job in &self.jobs {
            let cron = job.cron.clone();

            let device_address_for_closure = self.device_address.clone();
            let job_for_closure = job.clone();
            let template_manager_for_closure = self.template_manager.clone();

            let job_closure = move |_, _| -> Pin<Box<dyn Future<Output = ()> + Send>> {
                let device_address_for_async = device_address_for_closure.clone();
                let job_for_async = job_for_closure.clone();
                let template_manager_for_async = template_manager_for_closure.clone();
                Box::pin(async move {
                    let pixoo = match PixooClient::new(&device_address_for_async) {
                        Err(e) => {
                            error!(
                                "Failing to create device client: DeviceAddress = {}, Error = {:?}",
                                &device_address_for_async, e
                            );
                            return;
                        }
                        Ok(v) => v,
                    };

                    let mut dsl_runner = DivoomDslRunner::new(&pixoo, template_manager_for_async);
                    if let Err(e) = dsl_runner.batch_operations(&job_for_async.operations).await {
                        error!("Failing to batch operations: Error = {:?}", e);
                        return;
                    }

                    if let Err(e) = dsl_runner.execute().await {
                        error!("Failing to execute all operations: Error = {:?}", e);
                    }
                })
            };

            self.job_scheduler
                .add(Job::new_async(cron.as_ref(), job_closure).unwrap())
                .unwrap();
        }

        self.job_scheduler.start().unwrap();
    }

    #[cfg(not(feature = "animation-builder"))]
    pub fn from_config(
        device_address: String,
        schedules: Vec<DivoomScheduleConfigCronJob>,
    ) -> DivoomAPIResult<Self> {
        let mut jobs: Vec<Arc<DivoomScheduledJob>> = Vec::new();

        for schedule in schedules {
            let parsed_operations: DivoomAPIResult<Vec<DivoomDslOperation>> = schedule
                .operations
                .iter()
                .map(|x| DivoomDslParser::parse(x))
                .collect();
            jobs.push(Arc::new(DivoomScheduledJob {
                cron: schedule.cron,
                operations: parsed_operations?,
            }));
        }

        Ok(DivoomScheduleManager {
            device_address,
            jobs,
            job_scheduler: JobScheduler::new().unwrap(),
        })
    }

    #[cfg(not(feature = "animation-builder"))]
    pub fn start(&mut self) {
        for job in &self.jobs {
            let cron = job.cron.clone();

            let device_address_for_closure = self.device_address.clone();
            let job_for_closure = job.clone();

            let job_closure = move |_, _| -> Pin<Box<dyn Future<Output = ()> + Send>> {
                let device_address_for_async = device_address_for_closure.clone();
                let job_for_async = job_for_closure.clone();
                Box::pin(async move {
                    let pixoo = match PixooClient::new(&device_address_for_async) {
                        Err(e) => {
                            error!(
                                "Failing to create device client: DeviceAddress = {}, Error = {:?}",
                                &device_address_for_async, e
                            );
                            return;
                        }
                        Ok(v) => v,
                    };

                    let mut dsl_runner = DivoomDslRunner::new(&pixoo);
                    if let Err(e) = dsl_runner.batch_operations(&job_for_async.operations).await {
                        error!("Failing to batch operations: Error = {:?}", e);
                        return;
                    }

                    if let Err(e) = dsl_runner.execute().await {
                        error!("Failing to execute all operations: Error = {:?}", e);
                    }
                })
            };

            self.job_scheduler
                .add(Job::new_async(cron.as_ref(), job_closure).unwrap())
                .unwrap();
        }

        self.job_scheduler.start().unwrap();
    }
}
