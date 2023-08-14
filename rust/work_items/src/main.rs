mod request;
mod work_items;

extern crate reqwest;
extern crate serde_json;
extern crate base64;
extern crate tokio;

use std::error::Error;
use std::env;
use std::fs::{File, create_dir_all};
use std::io::Write;
use crate::work_items::WorkItem;

const THREAD_COUNT: usize = 100;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 引数処理
    let (organization, project, pat) = match get_start_args() {
        Ok(value) => value,
        Err(value) => return value,
    };

    // id一覧の取得
    let ids = work_items::get_work_items(organization.as_str(), project.as_str(), pat.as_str()).await?;

    // WorkItemsの取得
    let work_items = work_items::get_work_items_details(organization.as_str(), project.as_str(), pat.as_str(), ids).await?;

    let mut save_results = Vec::new();
    save_results.push(Vec::new());

    // 結果の表示
    for item in &work_items {
        println!("{}\t{}\t{:?}\t{}\t{}\t{}", item.id, item.rev, item.fields.changed_date.unwrap(), item.fields.state, item.fields.work_item_type,  item.fields.title);

        // リビジョンのJSONを保存を並列実行する
        let save_result = save_revisions_json(organization.as_str(), project.as_str(), pat.as_str(), item);

        if let Some(save_results2) = save_results.last_mut() {
            save_results2.push(save_result);
        }else{
            panic!("save_results.last_mut() is None");
        }
        if save_results.last().unwrap().len() == THREAD_COUNT {
            save_results.push(Vec::new());
        }
    }

    // save_resultsを逐次実行する
    for save_result in save_results {
        println!("{}", save_result.len());
        let results: Vec<_> = futures::future::join_all(save_result).await;
        // 結果のチェック
        for result in results {
            match result {
                Ok(_) => println!("Task succeeded"),
                Err(err) => eprintln!("Task failed: {}", err),
            }
        }
    }

    Ok(())
}

fn get_start_args() -> Result<(String, String, String), Result<(), Box<dyn Error>>> {
    let args: Vec<String> = env::args().collect();

    // 引数のチェック
    if args.len() < 4 {
        println!("引数が足りません。");
        println!("Usage: {} <organization> <project> <pat>", args[0]);
        return Err(Ok(()));
    }

    // 引数を変数化
    let organization = args[1].clone();
    let project = args[2].clone();
    let pat = args[3].clone();
    Ok((organization, project, pat))
}

async fn save_revisions_json(organization: &str, project: &str, pat: &str, item: &WorkItem) -> Result<(), Box<dyn Error>> {
    let id = item.id;
    let json_text = work_items::get_work_item_revisions_json(organization, project, pat, id).await?;
    // テキストファイルに保存
    let dir_path = format!("./data/{}/{}", organization, project);
    create_dir_all(&dir_path)?;
    let filename = format!("{}/{}.json", dir_path, id);
    let mut file = File::create(&filename)?;
    file.write_all(json_text.as_bytes())?;
    Ok(())
}

