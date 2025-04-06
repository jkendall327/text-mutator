// Resource group
targetScope = 'subscription'

param location string = deployment().location
param environmentName string = 'dev'
param appName string = 'text-mutator'
param publisherName string
param publisherEmail string

param rgName string

@description('String to make resource names unique')
var resourceToken = uniqueString(subscription().subscriptionId, location)

@description('Create a resource group')
resource rg 'Microsoft.Resources/resourceGroups@2024-03-01' = {
  name: rgName
  location: location
  tags: {
    environment: environmentName
    application: appName
  }
}

output rgName string = rg.name

@description('Create a static web app')
module swa 'br/public:avm/res/web/static-site:0.3.0' = {
  name: '${deployment().name}-${appName}-webapp'
  scope: rg
  params: {
    name: 'swa-${resourceToken}'
    location: location
    sku: 'Free'
  }
}

@description('Output the default hostname')
output endpoint string = swa.outputs.defaultHostname

@description('Output the static web app name')
output staticWebAppName string = swa.outputs.name

// Web App for Containers (backend)
module backend 'modules/backend.bicep' = {
  name: 'foobar'
  scope: rg
  params: {}
}

module api 'modules/api.bicep' = {
  name: 'api'
  scope: rg
  params: {
    publisherEmail: publisherEmail
    publisherName: publisherName
  }
}
