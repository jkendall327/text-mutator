// az deployment sub what-if --location westeurope --template-file main.bicep --parameters parameters.bicepparam
// az deployment sub create --location westeurope --template-file .infra/main.bicep --parameters .infra/parameters.bicepparam
// az deployment group create --resource-group textmutator --template-file .infra/main.bicep --parameters .infra/parameters.bicepparam
param environment string = 'dev'
param appName string = 'text-mutator'
param imageName string

var resourceToken = uniqueString(resourceGroup().id)

module registry 'modules/acr.bicep' = {
  params: {
    appName: appName
    environment: environment
  }
}

module backend 'modules/backend.bicep' = {
  name: 'backend'
  params: {
    appName: appName
    environment: environment
    imageName: imageName
    registryLoginServer: registry.outputs.loginServer
  }
}

var swaName = 'sta-${appName}-${environment}-${resourceToken}'

@description('Create a static web app')
module swa 'br/public:avm/res/web/static-site:0.3.0' = {
  name: swaName
  params: {
    name: swaName
    sku: 'Standard'
    tags: {
      environment: environment
      appName: appName
    }
  }
}

@description('Output the default hostname')
output endpoint string = swa.outputs.defaultHostname

@description('Output the static web app name')
output staticWebAppName string = swa.outputs.name

output appServiceName string = backend.outputs.appServiceName
output acrName string = registry.outputs.registryName
output acrLoginServer string = registry.outputs.loginServer
