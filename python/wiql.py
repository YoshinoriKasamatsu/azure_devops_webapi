import requests
import base64
import settings.webapi_paths as webapi_paths
import settings.config as config
import json


ids=[]

# 条件を指定してワークアイテムを取得する
contents = {
  "query": "Select [System.Id], [System.Title], [System.State] " \
            "From WorkItems " \
            "Where [System.WorkItemType] = 'Task' AND [State] <> 'Closed' AND [State] <> 'Removed' " \
            "order by [Microsoft.VSTS.Common.Priority] asc, [System.CreatedDate] desc"
}

response = requests.post(url=webapi_paths.work_items_wql_url, headers=config.headers, json=contents)
items=json.loads(response.text)
for item in items['workItems']:
    ids.append(item['id'])
    print(item)

# ワークアイテムの詳細を取得する
ids=','.join(map(str, ids))
work_items_url = webapi_paths.work_items_url % ids

workItems_response = requests.get(work_items_url, headers=config.headers)
workItems=json.loads(workItems_response.text)

for workItem in workItems['value']:
    workItem_id = workItem['id']
    print(str(workItem['id']) + '\t' + str(workItem['rev']) + '\t' + workItem['fields']['System.Title'] + '\t' + workItem['fields']['System.State'])
    # ワークアイテムの履歴を取得する

    revisions_response = requests.get(webapi_paths.revisions_url % workItem_id, headers=config.headers)
    revisions=json.loads(revisions_response.text)
    for revision in revisions['value']:  
      print(revision['rev'])
      print(revision)
    print()
