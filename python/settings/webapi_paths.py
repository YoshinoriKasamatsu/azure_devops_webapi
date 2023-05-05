import settings.config as config

# Work Item Schema
work_items_schema_url=f'https://dev.azure.com/{config.organization}/{config.project}/_apis/wit/fields?api-version=7.0'
# Work Item Query Language
work_items_wql_url=f'https://dev.azure.com/{config.organization}/{config.project}/_apis/wit/wiql?api-version=7.0&$top=9999&$expand=all'
# Work Item
# work_items_url = f'https://dev.azure.com/{config.organization}/{config.project}/_apis/wit/workitems?api-version=6.0'
work_items_url = f'https://dev.azure.com/{config.organization}/{config.project}/_apis/wit/workitems?ids=%s&api-version=7.0&$expand=all'
revisions_url = f'https://dev.azure.com/{config.organization}/{config.project}/_apis/wit/workitems/%d/revisions?api-version=6.0'

# Pull Request
pull_requests_url=f'https://dev.azure.com/{config.organization}/{config.project}/_apis/git/repositories/{config.repositoryId}/pullrequests?api-version=7.0&searchCriteria.status=all'
# Pull Request Detail
#   Args:PullRequestId
pull_request_detail_url=f'https://dev.azure.com/{config.organization}/{config.project}/_apis/git/repositories/{config.repositoryId}/pullrequests/%d?api-version=7.0&includeWorkItemRefs=true'
# Pull Request Threads
#   Args:ThreadId
pull_request_threads_url=f'https://dev.azure.com/{config.organization}/{config.project}/_apis/git/repositories/{config.repositoryId}/pullRequests/%d/threads?api-version=7.0'