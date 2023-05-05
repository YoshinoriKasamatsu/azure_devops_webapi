import base64

organization=''
project=''
repositoryId=''
personal_access_token = ''

authorization = str(base64.b64encode(bytes(':'+ personal_access_token, 'ascii')), 'ascii')
headers = {
    'Accept': 'application/json',
    'Authorization': 'Basic '+authorization
}