.infra/main.bicep
---
// az deployment sub what-if --location westeurope --template-file main.bicep --parameters parameters.bicepparam
targetScope = 'subscription'

param location string = deployment().location
param environment string = 'dev'
param appName string = 'text-mutator'
param rgName string

@description('Create a resource group')
resource rg 'Microsoft.Resources/resourceGroups@2024-03-01' = {
  name: rgName
  location: location
  tags: {
    application: appName
  }
}

var resourceToken = uniqueString(subscription().subscriptionId, location)
var swaName = 'stapp-${appName}-${environment}-${location}-${resourceToken}'

@description('Create a static web app')
module swa 'br/public:avm/res/web/static-site:0.3.0' = {
  name: swaName
  scope: rg
  params: {
    name: 'stapp-${resourceToken}'
    location: location
    sku: 'Free'
  }
}

module backend 'modules/backend.bicep' = {
  name: 'backend'
  scope: rg
  params: {
    appName: appName
    environment: environment
  }
}

module link 'modules/link.bicep' = {
  name: 'link'
  scope: rg
  params: {
    appName: appName
    environment: environment
    location: location
    staticWebAppName: swa.outputs.name
    backendAppResourceId: backend.outputs.backendResourceId
  }
}

@description('Output the default hostname')
output endpoint string = swa.outputs.defaultHostname

@description('Output the static web app name')
output staticWebAppName string = swa.outputs.name

@description('Output the name of the resource group')
output rgName string = rg.name


---
.infra/parameters.bicepparam
---
using 'main.bicep'

param rgName = 'textmutator'


---
.infra/modules/backend.bicep
---
@description('The deployment environment')
param environment string = 'dev'

@description('The semantic app name')
param appName string

@description('The deployment location')
param location string = resourceGroup().location

var servicePlanName = toLower('asp-${appName}-${environment}-${location}')
var serviceName = toLower('as-${appName}-${environment}-${location}-${uniqueString(resourceGroup().id)}')

resource appServicePlan 'Microsoft.Web/serverfarms@2020-12-01' = {
  name: servicePlanName
  location: location
  kind: 'linux'
  properties: {
    reserved: true
  }
  sku: {
    name: 'B1'
    tier: 'Basic'
  }
  tags: {
    application: appName
    environment: environment
  }
}

resource appService 'Microsoft.Web/sites@2020-06-01' = {
  name: serviceName
  location: location
  properties: {
    serverFarmId: appServicePlan.id
    siteConfig: {
      linuxFxVersion: 'node|14-lts'
    }
  }
  tags: {
    application: appName
    environment: environment
  }
}

@description('Output the resource ID of the backend app service instance')
output backendResourceId string = appService.id


---
.infra/modules/link.bicep
---
// https://learn.microsoft.com/en-us/azure/static-web-apps/publish-bicep
targetScope = 'resourceGroup'

@description('The deployment environment')
param environment string = 'dev'

@description('The semantic app name')
param appName string

@description('The deployment location')
param location string = resourceGroup().location

@description('The name of the static web app')
param staticWebAppName string

@description('The resource ID of the backend app')
param backendAppResourceId string

@description('Get reference to the static web app')
resource staticWebApp 'Microsoft.Web/staticSites@2023-12-01' existing = {
  name: staticWebAppName
}

var linkName = toLower('link-${appName}-${environment}-${location}')

@description('Link backend to static web app')
resource link 'Microsoft.Web/staticSites/linkedBackends@2023-12-01' = {
  parent: staticWebApp
  name: linkName
  properties: {
    backendResourceId: backendAppResourceId
    region: location
  }
}


---
