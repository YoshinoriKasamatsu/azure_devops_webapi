import requests
import base64
import config
import json

url=f'https://dev.azure.com/{config.organization}/{config.project}/_apis/wit/fields?api-version=7.0'
authorization = str(base64.b64encode(bytes(':'+ config.personal_access_token, 'ascii')), 'ascii')
headers = {
    'Accept': 'application/json',
    'Authorization': 'Basic '+authorization
}

output_fields = ['isLocked', 'name', 'referenceName', 'description', 'type', 'usage', 'readOnly', 'canSortBy', 'isQueryable', 'isIdentity', 'isPicklist', 'isPicklistSuggested', 'url', 'supportedOperations']

response = requests.get(url=url, headers=headers)

items=json.loads(response.text)

with open('schema.csv', 'w') as f:

    for item in items['value'][0:1]:
        for key in item.keys():
            if key in output_fields:
                f.write(key + '\t')
    f.write('\n')
    print()        
    for item in items['value']:
        for key in item.keys():
            if key in output_fields:
                f.write(str(item[key]) + '\t')
        f.write('\n')