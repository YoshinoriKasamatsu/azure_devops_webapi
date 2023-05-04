import requests
import base64
import config
import json


ids=[]

# 条件を指定してワークアイテムを取得する
contents = {
  "query": "Select [System.Id], [System.Title], [System.State] " \
            "From WorkItems " \
            "Where [System.WorkItemType] = 'Task' AND [State] <> 'Closed' AND [State] <> 'Removed' " \
            "order by [Microsoft.VSTS.Common.Priority] asc, [System.CreatedDate] desc"
}

url=f'https://dev.azure.com/{config.organization}/{config.project}/_apis/wit/wiql?api-version=7.0&$top=9999&$expand=all'
authorization = str(base64.b64encode(bytes(':'+ config.personal_access_token, 'ascii')), 'ascii')
headers = {
    'Accept': 'application/json',
    'Authorization': 'Basic '+authorization
}

response = requests.post(url=url, headers=headers, json=contents)
items=json.loads(response.text)
for item in items['workItems']:
    ids.append(item['id'])
    print(item)

# ワークアイテムの詳細を取得する
ids=','.join(map(str, ids))
workItems_url = f'https://dev.azure.com/{config.organization}/{config.project}/_apis/wit/workitems?ids={ids}&api-version=7.0&$expand=all'

workItems_response = requests.get(workItems_url, headers=headers)
workItems=json.loads(workItems_response.text)

for workItem in workItems['value']:
    workItem_id = workItem['id']
    print(str(workItem['id']) + '\t' + str(workItem['rev']) + '\t' + workItem['fields']['System.Title'] + '\t' + workItem['fields']['System.State'])
    # ワークアイテムの履歴を取得する
    revisions_url = f'https://dev.azure.com/{config.organization}/{config.project}/_apis/wit/workitems/{workItem_id}/revisions?api-version=6.0'
    revisions_response = requests.get(revisions_url, headers=headers)
    revisions=json.loads(revisions_response.text)
    for revision in revisions['value']:  
      print(revision['rev'])
      print(revision)
    print()
