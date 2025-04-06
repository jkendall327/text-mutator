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
