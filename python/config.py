
organization=''
project=''
repositoryId=''
personal_access_token = ''

# Work Item
work_items_url = f'https://dev.azure.com/{organization}/{project}/_apis/wit/workitems?api-version=6.0'

# Pull Request
pull_requests_url=f'https://dev.azure.com/{organization}/{project}/_apis/git/repositories/{repositoryId}/pullrequests?api-version=7.0&searchCriteria.status=all'
# Pull Request Detail
pull_request_detail_url=f'https://dev.azure.com/{organization}/{project}/_apis/git/repositories/{repositoryId}/pullrequests/%d?api-version=7.0&includeWorkItemRefs=true'
# Pull Request Threads
pull_request_threads_url=f'https://dev.azure.com/{organization}/{project}/_apis/git/repositories/{repositoryId}/pullRequests/%d/threads?api-version=7.0'