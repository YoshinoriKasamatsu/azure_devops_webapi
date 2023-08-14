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


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    // 引数のチェック
    if args.len() < 4 {
        println!("引数が足りません。");
        println!("Usage: {} <organization> <project> <pat>", args[0]);
        return Ok(());
    }

    let organization = &args[1];
    let project = &args[2];
    let pat = &args[3];

    let ids = work_items::get_work_items(organization, project, pat).await?;

    let work_items = work_items::get_work_items_details(organization, project, pat, ids).await?;

    // 結果の表示
    for item in &work_items {
        println!("{}\t{}\t{:?}\t{}\t{}\t{}", item.id, item.rev, item.fields.changed_date.unwrap(), item.fields.state, item.fields.work_item_type,  item.fields.title);
        let id = item.id;
        let json_text = work_items::get_work_item_revisions_json(organization, project, pat, id).await?;
        // テキストファイルに保存
        let dir_path = format!("./data/{}/{}", organization, project);
        create_dir_all(&dir_path)?;
        let filename = format!("{}/{}.json", dir_path, id);
        let mut file = File::create(&filename)?;
        file.write_all(json_text.as_bytes())?;
    }

    Ok(())
}

