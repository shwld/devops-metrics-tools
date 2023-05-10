use cranenum::Cranenum;

use crate::{
    dependencies::write_new_config::interface::WriteConfigData,
    persistence::project_config::ProjectConfig,
    project_creating::{
        validate_developer_count::{self, schema::ValidateDeveloperCountError},
        validate_github_owner_repo::{self, schema::ValidateGitHubOwnerRepoError},
        validate_github_personal_token::{self, schema::ValidateGitHubPersonalTokenError},
        validate_working_days_per_week::{self, schema::ValidateWorkingDaysPerWeekError},
    },
};

use super::create_github_deployment_project_schema::GitHubDeploymentProjectCreated;

pub type GitHubDeploymentProjectCreatedDto = WriteConfigData;

#[derive(Cranenum)]
pub enum ToGitHubDeploymentProjectCreatedError {
    GitHubPersonalToken(ValidateGitHubPersonalTokenError),
    GitHubOwnerRepo(ValidateGitHubOwnerRepoError),
    DeveloperCount(ValidateDeveloperCountError),
    WorkingDaysPerWeek(ValidateWorkingDaysPerWeekError),
}

impl GitHubDeploymentProjectCreatedDto {
    pub fn to_git_hub_deployment_project_created(
        dto: &GitHubDeploymentProjectCreatedDto,
    ) -> Result<GitHubDeploymentProjectCreated, ToGitHubDeploymentProjectCreatedError> {
        let github_personal_token = validate_github_personal_token::workflow::perform(Some(
            dto.github_personal_token.to_string(),
        ))?;
        let github_owner_repo = validate_github_owner_repo::workflow::perform(format!(
            "{}/{}",
            dto.project_config.github_owner, dto.project_config.github_repo
        ))?;
        let developer_count = validate_developer_count::workflow::perform(
            dto.project_config.developer_count.to_string(),
        )?;
        let working_days_per_week = validate_working_days_per_week::workflow::perform(
            dto.project_config.working_days_per_week.to_string(),
        )?;
        Ok(GitHubDeploymentProjectCreated {
            project_name: dto.project_name.to_string(),
            github_personal_token,
            github_owner_repo,
            developer_count,
            working_days_per_week,
        })
    }

    pub fn from_git_hub_deployment_project_created(
        domain_obj: GitHubDeploymentProjectCreated,
    ) -> GitHubDeploymentProjectCreatedDto {
        let (owner, repo) = domain_obj.github_owner_repo.get_values();
        GitHubDeploymentProjectCreatedDto {
            project_name: domain_obj.project_name,
            github_personal_token: domain_obj.github_personal_token.to_string(),
            project_config: ProjectConfig {
                github_personal_token: None,
                heroku_api_token: None,
                heroku_app_name: None,
                deployment_source: "github_deployment".to_string(),
                github_owner: owner,
                github_repo: repo,
                developer_count: domain_obj.developer_count.to_u32(),
                working_days_per_week: domain_obj.working_days_per_week.to_f32(),
            },
        }
    }
}