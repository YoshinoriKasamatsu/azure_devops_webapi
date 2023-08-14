extern crate chrono;

use std::error::Error;
use crate::{request::{request_post, request_get}};
use serde::Deserialize;
use chrono::{DateTime, Utc};
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct WorkItem {
    pub fields: Fields,
    pub id: u64,
    pub rev: u64,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Fields {
    #[serde(rename = "Microsoft.VSTS.Common.Priority")]
    pub priority: Option<i64>,
    #[serde(rename = "Microsoft.VSTS.Common.StateChangeDate")]
    pub state_change_date: Option<DateTime<Utc>>,
    #[serde(rename = "Microsoft.VSTS.Common.ValueArea")]
    pub value_area: Option<String>,
    #[serde(rename = "Microsoft.VSTS.Scheduling.Effort")]
    pub effort: Option<f64>,
    #[serde(rename = "Microsoft.VSTS.Scheduling.RemainingWork")]
    pub remaining_work: Option<f64>,
    #[serde(rename = "System.AreaPath")]
    pub area_path: Option<String>,

    #[serde(rename = "System.ChangedBy")]
    pub changed_by: ChangedBy,
    #[serde(rename = "System.CreatedDate")]
    pub changed_date: Option<DateTime<Utc>>,
    #[serde(rename = "System.ChangedDate")]
    pub created_date: Option<DateTime<Utc>>,
    
    #[serde(rename = "System.Description")]
    pub description: Option<String>,
    #[serde(rename = "System.IterationPath")]
    pub iteration_path: Option<String>,
    #[serde(rename = "System.Reason")]
    pub reason: Option<String>,
    #[serde(rename = "System.State")]
    pub state: String,
    #[serde(rename = "System.TeamProject")] 
    pub team_project: Option<String>,
    #[serde(rename = "System.Title")]
    pub title: String,
    #[serde(rename = "System.WorkItemType")]
    pub work_item_type: String,
    #[serde(flatten)]
    pub other: std::collections::HashMap<String, Value>,
}

#[derive(Debug, Deserialize)]
pub struct ChangedBy {
    pub descriptor: String,
    pub display_name: Option<String>,
    pub id: String,
    pub image_url: Option<String>,
    pub unique_name: Option<String>,
    pub url: String,
    #[serde(rename = "_links")]
    pub links: Links,
}

#[derive(Debug, Deserialize)]
pub struct Links {
    pub avatar: Avatar,
}

#[derive(Debug, Deserialize)]
pub struct Avatar {
    pub href: String,
}

pub async fn get_work_items(organization: &str, project: &str, pat: &str) -> Result<Vec<i64>, Box<dyn Error>> {
    let url = format!("https://dev.azure.com/{}/_apis/wit/wiql?api-version=7.0", organization);

    // Work Item Query Language (WIQL) クエリ
    let query = r#"{
        "query": "SELECT [System.Id], [System.Title], [System.WorkItemType] FROM workitems WHERE [System.TeamProject] = '@project' ORDER BY [System.Id]"
    }"#.replace("@project", project);

    let json = request_post(pat, &url, query).await?;
    let mut ids = Vec::new();

    // 結果の表示
    if let Some(work_items) = json["workItems"].as_array() {
        for item in work_items {
            if let Some(id) = item["id"].as_i64() {
                ids.push(id);
            }
        }
    }
    Ok(ids)
}

pub async fn get_work_items_details(organization: &str, project: &str, pat: &str, ids: Vec<i64>) -> Result<Vec<WorkItem>, Box<dyn Error>> {

    // idを200個ずつに分割
    let mut ids_vec = Vec::new();
    for chunk in ids.chunks(200) {
        ids_vec.push(chunk.to_vec());
    }

    let mut all_work_items: Vec<WorkItem> = Vec::new();
    for ids in ids_vec {
        let ids_str: String = ids.iter().map(|ids: &i64| ids.to_string()).collect::<Vec<String>>().join(",");
        let url = format!("https://dev.azure.com/{}/{}/_apis/wit/workitems?ids={}&api-version=7.0", organization, project, &ids_str);
        let json_data = request_get(pat, &url).await?;
        let value = json_data["value"].to_string();
        let work_items: Vec<WorkItem> = serde_json::from_str(&value)?;
        all_work_items.extend(work_items);
    }

    Ok(all_work_items)
}

pub async fn get_work_item_revisions_json(organization: &str, project: &str, pat: &str, id: u64) -> Result<String, Box<dyn Error>> {
    let url = format!("https://dev.azure.com/{}/{}/_apis/wit/workitems/{}/revisions?$expand=all&api-version=7.0", organization, project, id);
    let json_data = request_get(pat, &url).await?;
    Ok(json_data.to_string())
}