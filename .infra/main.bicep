// az deployment sub what-if --location westeurope --template-file main.bicep --parameters parameters.bicepparam
// az deployment sub create --location westeurope --template-file .infra/main.bicep --parameters .infra/parameters.bicepparam
// az deployment group create --resource-group textmutator --template-file .infra/main.bicep --parameters .infra/parameters.bicepparam
param location string = resourceGroup().location
param environment string = 'dev'
param appName string = 'text-mutator'
param managedIdentityName string
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
    //location: location
    // linkedBackend: {
    //   backendId: backend.outputs.backendResourceId
    // }
    sku: 'Standard'
    tags: {
      environment: environment
      appName: appName
    }
  }
}

module link 'modules/link.bicep' = {
  name: 'link'
  params: {
    appName: appName
    environment: environment
    location: location
    staticWebAppName: swaName
    backendAppResourceId: backend.outputs.backendResourceId
  }
}

// TODO: figure out how to make this role assignment idempotent.

// Built-in role definition ID for AcrPull
// var acrPullRoleDefinitionId = subscriptionResourceId(
//   'Microsoft.Authorization/roleDefinitions',
//   '7f951dda-4ed3-4680-a7ca-43fe172d538d'
// )

// resource assignAcrPullRole 'Microsoft.Authorization/roleAssignments@2022-04-01' = {
//   name: guid(registry.name, backend.name, acrPullRoleDefinitionId)
//   properties: {
//     roleDefinitionId: acrPullRoleDefinitionId
//     principalId: backend.outputs.appServicePrincipalId
//     principalType: 'ServicePrincipal'
//   }
// }

module identity 'modules/identity.bicep' = {
  params: {
    managedIdentityName: managedIdentityName
    appName: appName
    environment: environment
  }
}

@description('Output the default hostname')
output endpoint string = swa.outputs.defaultHostname

@description('Output the static web app name')
output staticWebAppName string = swa.outputs.name

output identityClientId string = identity.outputs.managedIdentityClientId
output identityPrincipalId string = identity.outputs.managedIdentityPrincipalId
output identityResourceId string = identity.outputs.managedIdentityResourceId
output appServiceName string = backend.outputs.appServiceName
output acrName string = registry.outputs.registryName
output acrLoginServer string = registry.outputs.loginServer
