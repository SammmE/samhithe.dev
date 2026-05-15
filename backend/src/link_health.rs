use std::{sync::Arc, time::Duration};

use chrono::{Datelike, Days, Timelike, Utc, Weekday};
use reqwest::StatusCode;
use tokio::time::MissedTickBehavior;

use crate::{models::HealthStatus, state::AppState};

pub fn spawn_worker(state: Arc<AppState>) {
    tokio::spawn(async move {
        tokio::time::sleep(next_sunday_midnight_delay()).await;

        let mut interval =
            tokio::time::interval(Duration::from_secs(state.config.link_health_interval_secs));
        interval.set_missed_tick_behavior(MissedTickBehavior::Delay);

        loop {
            interval.tick().await;
            if let Err(err) = run_once(state.clone()).await {
                tracing::error!("link health worker failed: {err}");
            }
        }
    });
}

fn next_sunday_midnight_delay() -> Duration {
    let now = Utc::now();
    let today_midnight = now
        .date_naive()
        .and_hms_opt(0, 0, 0)
        .expect("midnight is a valid time");
    let days_until_sunday =
        (Weekday::Sun.num_days_from_monday() + 7 - now.weekday().num_days_from_monday()) % 7;
    let mut target = today_midnight
        .checked_add_days(Days::new(days_until_sunday.into()))
        .expect("date arithmetic should stay in range")
        .and_utc();

    if days_until_sunday == 0 && (now.hour() > 0 || now.minute() > 0 || now.second() > 0) {
        target = target
            .checked_add_days(Days::new(7))
            .expect("date arithmetic should stay in range");
    }

    (target - now)
        .to_std()
        .unwrap_or_else(|_| Duration::from_secs(0))
}

async fn run_once(state: Arc<AppState>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let projects = state.firestore.list_projects().await?;

    for project in projects {
        let mut broken = false;
        for link in [project.demo_link.as_deref(), project.repo_link.as_deref()]
            .into_iter()
            .flatten()
        {
            let response = state.http.head(link).send().await;
            match response {
                Ok(response)
                    if response.status() == StatusCode::NOT_FOUND
                        || response.status().is_server_error() =>
                {
                    tracing::warn!(project = %project.id, link, status = %response.status(), "project link is broken");
                    broken = true;
                }
                Ok(response) => {
                    tracing::debug!(project = %project.id, link, status = %response.status(), "project link checked");
                }
                Err(err) => {
                    tracing::warn!(project = %project.id, link, error = %err, "project link check failed");
                    broken = true;
                }
            }
        }

        let status = if broken {
            HealthStatus::Broken
        } else {
            HealthStatus::Healthy
        };
        if let Err(err) = state
            .firestore
            .update_project_health(&project, status)
            .await
        {
            tracing::error!(project = %project.id, error = %err, "failed to update project health");
        }
    }

    Ok(())
}
