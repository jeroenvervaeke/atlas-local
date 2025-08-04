use std::collections::HashMap;

use bollard::{
    Docker,
    query_parameters::{InspectContainerOptions, ListContainersOptionsBuilder},
};
use maplit::hashmap;

pub struct Client {
    docker: Docker,
}

impl Client {
    pub fn new(docker: Docker) -> Client {
        Client { docker }
    }

    pub async fn list_deployments(&self) -> Result<Vec<Deployment>, ListDeploymentError> {
        let list_container_options = ListContainersOptionsBuilder::default()
            .all(true)
            .filters(&hashmap! {
                "label" => vec!["mongodb-atlas-local=container"],
            })
            .build();

        let container_summaries = self
            .docker
            .list_containers(Some(list_container_options))
            .await?;

        let mut deployments = Vec::with_capacity(container_summaries.len());

        for container_summary in container_summaries {
            let container_id = container_summary
                .id
                .ok_or(ListDeploymentError::MissingContainerID)?;

            let deployment = self.get_deployment(container_id.as_ref()).await?;
            deployments.push(deployment);
        }

        Ok(deployments)
    }

    pub async fn get_deployment(
        &self,
        container_id: &str,
    ) -> Result<Deployment, GetDeploymentError> {
        let container_inspect_response = self
            .docker
            .inspect_container(container_id, None::<InspectContainerOptions>)
            .await?;

        let mut creation_source = None;

        if let Some(config) = &container_inspect_response.config {
            if let Some(env) = &config.env {
                let env_variables = env
                    .iter()
                    .filter_map(|e| e.split_once("="))
                    .collect::<HashMap<_, _>>();

                if let Some(tool) = env_variables.get("TOOL") {
                    if *tool == "ATLASCLI" {
                        creation_source = Some(CreationSource::AtlasCLI);
                    }
                }
            }
        }

        Ok(Deployment {
            container_id: container_id.to_string(),
            creation_source,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ListDeploymentError {
    #[error("Failed to list containers")]
    ListContainers(#[from] bollard::errors::Error),
    #[error("Container ID is missing")]
    MissingContainerID,
    #[error("Failed to get deployment details")]
    GetDeployment(#[from] GetDeploymentError),
}

#[derive(Debug, thiserror::Error)]
pub enum GetDeploymentError {
    #[error("Failed to inspect container")]
    ContainerInspect(#[from] bollard::errors::Error),
}

#[derive(Debug)]
pub struct Deployment {
    pub container_id: String,
    pub creation_source: Option<CreationSource>,
}

#[derive(Debug)]
pub enum CreationSource {
    AtlasCLI,
}
