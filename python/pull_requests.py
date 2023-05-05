import requests
import base64
import config
import json

# Pull Requestの一覧を取得する
authorization = str(base64.b64encode(bytes(':'+ config.personal_access_token, 'ascii')), 'ascii')
headers = {
    'Accept': 'application/json',
    'Authorization': 'Basic '+authorization
}
response = requests.get(url=config.pull_requests_url, headers=headers)
items=json.loads(response.text)

for item in items['value']:
    pullRequestId = item['pullRequestId']
    print(str(item['pullRequestId']) + '\t' + item['status'] + '\t' + item['title'] + '\t' + item['createdBy']['displayName'] + '\t' + item['creationDate'] , end='')
    if(item['status']=='completed'):
        print('\t' + item['closedDate'])

    for reviewer in item['reviewers']:
        print(reviewer['displayName'])

    # Pull Requestの詳細を取得する
    pull_request_detail_url=config.pull_request_detail_url % pullRequestId
    pullRequest_response = requests.get(url=pull_request_detail_url, headers=headers)

    # Work Itemの詳細を取得する
    pullRequestResult = json.loads(pullRequest_response.text)
    # print(pullRequest_response.text)
    print('\t----- WOrk Items -----')
    if 'workItemRefs' in pullRequestResult:
        for workItem in pullRequestResult['workItemRefs']:
            workItem_Response = requests.get(url=workItem['url'], headers=headers)
            workItemResult = json.loads(workItem_Response.text)
            
            print('\t' + str(workItem['id']) + '\t' + workItemResult['fields']['System.WorkItemType']  + '\t' + workItemResult['fields']['System.Title'])
    # Pull Requestのスレッドを取得する
    pull_request_threads_url=config.pull_request_threads_url % pullRequestId
    pullRequestThreads_response = requests.get(url=pull_request_threads_url, headers=headers)
    pullRequestThreadsResult = json.loads(pullRequestThreads_response.text)
    print('\t----- Threads -----')
    for thread in pullRequestThreadsResult['value']:
        print('\t' + str(thread['id']) + '\t' + thread['publishedDate'])
        for comment in thread['comments']:
            print('\t\t' + comment['author']['displayName'] + '\t' + str(comment['content']).replace('\n', ' '))
        print()
    print()