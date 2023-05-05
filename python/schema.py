import requests
import base64
import settings.config as config
import json
import settings.webapi_paths as webapi_paths


output_fields = ['isLocked', 'name', 'referenceName', 'description', 'type', 'usage', 'readOnly', 'canSortBy', 'isQueryable', 'isIdentity', 'isPicklist', 'isPicklistSuggested', 'url', 'supportedOperations']
response = requests.get(url=webapi_paths.work_items_schema_url, headers=config.headers)

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