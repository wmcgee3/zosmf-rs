pub mod feedback;
pub mod file_list;
pub mod file_read;
pub mod list;
pub mod purge;
pub mod status;
pub mod submit;

use serde::{Deserialize, Serialize};
use z_osmf_macros::Getters;

use crate::convert::TryFromResponse;
use crate::error::Error;
use crate::ZOsmf;

use self::feedback::{ClassJson, JobFeedback, JobFeedbackBuilder, RequestJson};
use self::file_list::{JobFileList, JobFileListBuilder};
use self::file_read::{JobFileID, JobFileRead, JobFileReadBuilder};
use self::list::{JobList, JobListBuilder};
use self::purge::JobPurgeBuilder;
use self::status::JobStatusBuilder;
use self::submit::{JclSource, JobSubmitBuilder};

/// # Jobs
impl ZOsmf {
    /// # Examples
    ///
    /// Cancel job TESTJOB2 with ID JOB0084:
    /// ```
    /// # use z_osmf::jobs::JobIdentifier;
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let identifier = JobIdentifier::NameId("TESTJOB2".into(), "JOB00084".into());
    ///
    /// let job_feedback = zosmf
    ///     .cancel_job(identifier)
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn cancel_job(
        &self,
        identifier: JobIdentifier,
    ) -> JobFeedbackBuilder<JobFeedback, RequestJson> {
        JobFeedbackBuilder::new(
            self.base_url.clone(),
            self.client.clone(),
            identifier,
            RequestJson::new("cancel"),
        )
    }

    /// # Examples
    ///
    /// Cancel and purge the output of job TESTJOBW with ID JOB0085:
    /// ```
    /// # use z_osmf::jobs::JobIdentifier;
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let identifier = JobIdentifier::NameId("TESTJOBW".into(), "JOB00085".into());
    ///
    /// let job_feedback = zosmf
    ///     .cancel_and_purge_job(identifier)
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn cancel_and_purge_job(&self, identifier: JobIdentifier) -> JobPurgeBuilder<JobFeedback> {
        JobPurgeBuilder::new(self.base_url.clone(), self.client.clone(), identifier)
    }

    /// # Examples
    ///
    /// Change the message class of job TESTJOBW with ID JOB0023:
    /// ```
    /// # use z_osmf::jobs::JobIdentifier;
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let identifier = JobIdentifier::NameId("TESTJOBW".into(), "JOB00023".into());
    ///
    /// let job_feedback = zosmf
    ///     .change_job_class(identifier, 'A')
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn change_job_class<C>(
        &self,
        identifier: JobIdentifier,
        class: C,
    ) -> JobFeedbackBuilder<JobFeedback, ClassJson>
    where
        C: Into<char>,
    {
        JobFeedbackBuilder::new(
            self.base_url.clone(),
            self.client.clone(),
            identifier,
            ClassJson::new(class),
        )
    }

    /// # Examples
    ///
    /// Hold job TESTJOBW with ID JOB0023:
    /// ```
    /// # use z_osmf::jobs::JobIdentifier;
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let identifier = JobIdentifier::NameId("TESTJOBW".into(), "JOB00023".into());
    ///
    /// let job_feedback = zosmf
    ///     .hold_job(identifier)
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn hold_job(
        &self,
        identifier: JobIdentifier,
    ) -> JobFeedbackBuilder<JobFeedback, RequestJson> {
        JobFeedbackBuilder::new(
            self.base_url.clone(),
            self.client.clone(),
            identifier,
            RequestJson::new("hold"),
        )
    }

    /// # Examples
    ///
    /// Obtain the status of the job BLSJPRMI, job ID STC00052:
    /// ```
    /// # use z_osmf::jobs::JobIdentifier;
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let identifier = JobIdentifier::NameId("BLSJPRMI".into(), "STC00052".into());
    ///
    /// let job_status = zosmf
    ///     .job_status(identifier)
    ///     .exec_data()
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn job_status(&self, identifier: JobIdentifier) -> JobStatusBuilder<JobData> {
        JobStatusBuilder::new(self.base_url.clone(), self.client.clone(), identifier)
    }

    /// # Examples
    ///
    /// List the spool files for job TESTJOB1 with ID JOB00023:
    /// ```
    /// # use z_osmf::jobs::JobIdentifier;
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let identifier = JobIdentifier::NameId("TESTJOB1".into(), "JOB00023".into());
    ///
    /// let job_files = zosmf
    ///     .list_job_files(identifier)
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_job_files(&self, identifier: JobIdentifier) -> JobFileListBuilder<JobFileList> {
        JobFileListBuilder::new(self.base_url.clone(), self.client.clone(), identifier)
    }

    /// # Examples
    ///
    /// List jobs with exec-data by owner and prefix:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let job_list = zosmf
    ///     .list_jobs()
    ///     .owner("IBMUSER")
    ///     .prefix("TESTJOB*")
    ///     .exec_data()
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_jobs(&self) -> JobListBuilder<JobList<JobData>> {
        JobListBuilder::new(self.base_url.clone(), self.client.clone())
    }

    /// # Examples
    ///
    /// Read file 1 for job TESTJOBJ with ID JOB00023:
    /// ```
    /// # use z_osmf::jobs::file_read::JobFileID;
    /// # use z_osmf::jobs::JobIdentifier;
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let identifier = JobIdentifier::NameId("TESTJOBJ".into(), "JOB00023".into());
    /// let file_id = JobFileID::ID(1);
    ///
    /// let job_file = zosmf
    ///     .read_job_file(identifier, file_id)
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Read a range of records (the first 250) of file 8 for job TESTJOBJ with ID JOB00023:
    /// ```
    /// # use std::str::FromStr;
    /// # use z_osmf::jobs::file_read::{JobFileID, RecordRange};
    /// # use z_osmf::jobs::JobIdentifier;
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let identifier = JobIdentifier::NameId("TESTJOBJ".into(), "JOB00023".into());
    /// let file_id = JobFileID::ID(8);
    ///
    /// let job_file = zosmf
    ///     .read_job_file(identifier, file_id)
    ///     .record_range(RecordRange::from_str("0-249")?)
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Read the JCL for job TESTJOBJ with ID JOB00060:
    /// ```
    /// # use z_osmf::jobs::file_read::JobFileID;
    /// # use z_osmf::jobs::JobIdentifier;
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let identifier = JobIdentifier::NameId("TESTJOBJ".into(), "JOB00060".into());
    /// let file_id = JobFileID::JCL;
    ///
    /// let job_file = zosmf
    ///     .read_job_file(identifier, file_id)
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn read_job_file(
        &self,
        identifier: JobIdentifier,
        id: JobFileID,
    ) -> JobFileReadBuilder<JobFileRead<Box<str>>> {
        JobFileReadBuilder::new(self.base_url.clone(), self.client.clone(), identifier, id)
    }

    /// # Examples
    ///
    /// Release job TESTJOBW with ID JOB0023:
    /// ```
    /// # use z_osmf::jobs::JobIdentifier;
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let identifier = JobIdentifier::NameId("TESTJOBW".into(), "JOB00023".into());
    ///
    /// let job_feedback = zosmf
    ///     .release_job(identifier)
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn release_job(
        &self,
        identifier: JobIdentifier,
    ) -> JobFeedbackBuilder<JobFeedback, RequestJson> {
        JobFeedbackBuilder::new(
            self.base_url.clone(),
            self.client.clone(),
            identifier,
            RequestJson::new("release"),
        )
    }

    /// # Examples
    ///
    /// Submit a job from text:
    /// ```
    /// # use z_osmf::jobs::submit::{JclData, JclSource, RecordFormat};
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let jcl = r#"//TESTJOBX JOB (),MSGCLASS=H
    /// // EXEC PGM=IEFBR14
    /// "#;
    ///
    /// let job_data = zosmf
    ///     .submit_job(JclSource::Jcl(JclData::Text(jcl.into())))
    ///     .message_class('A')
    ///     .record_format(RecordFormat::Fixed)
    ///     .record_length(80)
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn submit_job(&self, jcl_source: JclSource) -> JobSubmitBuilder<JobData> {
        JobSubmitBuilder::new(self.base_url.clone(), self.client.clone(), jcl_source)
    }
}

pub struct AsynchronousResponse;

impl TryFromResponse for AsynchronousResponse {
    async fn try_from_response(_: reqwest::Response) -> Result<Self, Error> {
        Ok(AsynchronousResponse {})
    }
}

#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct JobData {
    #[serde(rename = "jobid")]
    id: Box<str>,
    #[serde(rename = "jobname")]
    name: Box<str>,
    subsystem: Option<Box<str>>,
    owner: Box<str>,
    #[getter(copy)]
    status: Option<Status>,
    #[getter(copy)]
    job_type: Option<JobType>,
    class: Box<str>,
    #[serde(rename = "retcode")]
    return_code: Option<Box<str>>,
    url: Box<str>,
    files_url: Box<str>,
    job_correlator: Option<Box<str>>,
    #[getter(copy)]
    phase: i32,
    phase_name: Box<str>,
    reason_not_running: Option<Box<str>>,
}

impl JobData {
    pub fn identifier(&self) -> JobIdentifier {
        JobIdentifier::NameId(self.name.to_string(), self.id.to_string())
    }
}

impl TryFromResponse for JobData {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, Error> {
        Ok(value.json().await?)
    }
}

#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct JobExecData {
    #[serde(rename = "jobid")]
    id: Box<str>,
    #[serde(rename = "jobname")]
    name: Box<str>,
    subsystem: Option<Box<str>>,
    owner: Box<str>,
    #[getter(copy)]
    status: Option<Status>,
    #[getter(copy)]
    job_type: Option<JobType>,
    class: Box<str>,
    #[serde(rename = "retcode")]
    return_code: Option<Box<str>>,
    url: Box<str>,
    files_url: Box<str>,
    job_correlator: Option<Box<str>>,
    #[getter(copy)]
    phase: i32,
    phase_name: Box<str>,
    #[serde(default)]
    exec_system: Option<Box<str>>,
    #[serde(default)]
    exec_member: Option<Box<str>>,
    #[serde(default)]
    exec_submitted: Option<Box<str>>,
    #[serde(default)]
    exec_ended: Option<Box<str>>,
    reason_not_running: Option<Box<str>>,
}

impl JobExecData {
    pub fn identifier(&self) -> JobIdentifier {
        JobIdentifier::NameId(self.name.to_string(), self.id.to_string())
    }
}

impl TryFromResponse for JobExecData {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, Error> {
        Ok(value.json().await?)
    }
}

#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct JobExecStepData {
    #[serde(rename = "jobid")]
    id: Box<str>,
    #[serde(rename = "jobname")]
    name: Box<str>,
    subsystem: Option<Box<str>>,
    owner: Box<str>,
    #[getter(copy)]
    status: Option<Status>,
    #[getter(copy)]
    job_type: Option<JobType>,
    class: Box<str>,
    #[serde(rename = "retcode")]
    return_code: Option<Box<str>>,
    url: Box<str>,
    files_url: Box<str>,
    job_correlator: Option<Box<str>>,
    #[getter(copy)]
    phase: i32,
    phase_name: Box<str>,
    step_data: Box<[StepData]>,
    #[serde(default)]
    exec_system: Option<Box<str>>,
    #[serde(default)]
    exec_member: Option<Box<str>>,
    #[serde(default)]
    exec_submitted: Option<Box<str>>,
    #[serde(default)]
    exec_ended: Option<Box<str>>,
    reason_not_running: Option<Box<str>>,
}

impl JobExecStepData {
    pub fn identifier(&self) -> JobIdentifier {
        JobIdentifier::NameId(self.name.to_string(), self.id.to_string())
    }
}

impl TryFromResponse for JobExecStepData {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, Error> {
        Ok(value.json().await?)
    }
}

#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct JobStepData {
    #[serde(rename = "jobid")]
    id: Box<str>,
    #[serde(rename = "jobname")]
    name: Box<str>,
    subsystem: Option<Box<str>>,
    owner: Box<str>,
    #[getter(copy)]
    status: Option<Status>,
    #[getter(copy)]
    job_type: Option<JobType>,
    class: Box<str>,
    #[serde(rename = "retcode")]
    return_code: Option<Box<str>>,
    url: Box<str>,
    files_url: Box<str>,
    job_correlator: Option<Box<str>>,
    #[getter(copy)]
    phase: i32,
    phase_name: Box<str>,
    step_data: Box<[StepData]>,
    reason_not_running: Option<Box<str>>,
}

impl JobStepData {
    pub fn identifier(&self) -> JobIdentifier {
        JobIdentifier::NameId(self.name.to_string(), self.id.to_string())
    }
}

impl TryFromResponse for JobStepData {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, Error> {
        Ok(value.json().await?)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum JobIdentifier {
    Correlator(String),
    NameId(String, String),
}

impl std::fmt::Display for JobIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let items = match self {
            JobIdentifier::Correlator(correlator) => vec![correlator.as_ref()],
            JobIdentifier::NameId(name, id) => vec![name.as_ref(), id.as_ref()],
        };

        write!(f, "{}", items.join("/"))
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum JobType {
    #[serde(rename = "JOB")]
    Job,
    #[serde(rename = "STC")]
    StartedTask,
    #[serde(rename = "TSU")]
    TsoUser,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Status {
    Active,
    Input,
    Output,
}

#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct StepData {
    #[getter(copy)]
    active: bool,
    #[serde(default, rename = "smfid")]
    smf_id: Option<Box<str>>,
    #[getter(copy)]
    step_number: i32,
    #[serde(default)]
    selected_time: Option<Box<str>>,
    #[serde(default)]
    owner: Option<Box<str>>,
    program_name: Box<str>,
    step_name: Box<str>,
    #[serde(default)]
    path_name: Option<Box<str>>,
    #[getter(copy)]
    #[serde(default)]
    substep_number: Option<i32>,
    #[serde(default)]
    end_time: Option<Box<str>>,
    proc_step_name: Box<str>,
    #[serde(default, rename = "completion")]
    completion_code: Option<Box<str>>,
    #[serde(default)]
    abend_reason_code: Option<Box<str>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_job_identifier() {
        assert_eq!(
            format!("{}", JobIdentifier::Correlator("ABCD1234".into())),
            "ABCD1234"
        );
    }
}
