@description('The name of the App Service instance')
var appServicePlanName = toLower('AppServicePlan-${webAppName}')

@description('The name of the App Service instance')
param webAppName string = uniqueString(resourceGroup().id)

resource appServicePlan 'Microsoft.Web/serverfarms@2020-12-01' = {
  name: appServicePlanName
  location: resourceGroup().location
  kind: 'linux'
  properties: {
    reserved: true
  }
  sku: {
    name: 'B1'
    tier: 'Basic'
  }
}

resource appService 'Microsoft.Web/sites@2020-06-01' = {
  name: toLower('wapp-${webAppName}')
  location: resourceGroup().location
  properties: {
    serverFarmId: appServicePlan.id
    siteConfig: {
      linuxFxVersion: 'node|14-lts'
    }
  }
}
