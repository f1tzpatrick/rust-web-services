param appName string
param location string

module keyvault './keyvault.bicep' = {
  name: 'keyvault'
  params: {
    keyVaultName: '${appName}-kv'
    location: location
  }
}
