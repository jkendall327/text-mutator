@description('The deployment environment')
param environment string = 'dev'

@description('The semantic app name')
param appName string

@description('The deployment location')
param location string = resourceGroup().location

@description('The name of the ACR registry instance')
param registryLoginServer string

@description('The name of the Docker image to use for the backend')
param imageName string

@description('The log level for the backend')
param logLevel string = 'INFO'

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
  identity: {
    type: 'SystemAssigned'
  }
  properties: {
    serverFarmId: appServicePlan.id
    siteConfig: {
      alwaysOn: true
      acrUseManagedIdentityCreds: true
      linuxFxVersion: 'DOCKER|${registryLoginServer}/${imageName}:latest'
      appSettings: [
        { name: 'RUST_LOG', value: logLevel }
        { name: 'WEBSITES_PORT', value: '8080' }
        { name: 'MUTATOR_BACKEND_URL', value: '0.0.0.0:8080' }
        { name: 'MUTATOR_FRONTEND_URL', value: 'http://localhost:5173' } // TODO: consider removing this, not really needed in production.
      ]
    }
  }
  tags: {
    application: appName
    environment: environment
  }
}

@description('Output the resource ID of the backend app service instance')
output backendResourceId string = appService.id

@description('The principal ID of the system-assigned managed identity of the app.')
output appServicePrincipalId string = appService.identity.principalId

@description('The default hostname.')
output appServiceHostName string = appService.properties.defaultHostName

@description('The name of the App Service.')
output appServiceName string = appService.name
