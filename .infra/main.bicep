// az deployment sub what-if --location westeurope --template-file main.bicep --parameters parameters.bicepparam
targetScope = 'subscription'

param location string = deployment().location
param environment string = 'dev'
param appName string = 'text-mutator'
param rgName string
param imageName string

@description('Create a resource group')
resource rg 'Microsoft.Resources/resourceGroups@2024-03-01' = {
  name: rgName
  location: location
  tags: {
    application: appName
  }
}

var resourceToken = uniqueString(rg.id)
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

module registry 'modules/acr.bicep' = {
  scope: rg
}

module backend 'modules/backend.bicep' = {
  name: 'backend'
  scope: rg
  params: {
    appName: appName
    environment: environment
    imageName: imageName
    registryLoginServer: registry.outputs.loginServer
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

// Built-in role definition ID for AcrPull
var acrPullRoleDefinitionId = subscriptionResourceId(
  'Microsoft.Authorization/roleDefinitions',
  '7f951dda-4ed3-4680-a7ca-43fe172d538d'
)

resource assignAcrPullRole 'Microsoft.Authorization/roleAssignments@2022-04-01' = {
  name: 'role-${uniqueString(rg.id)}'
  properties: {
    roleDefinitionId: acrPullRoleDefinitionId
    principalId: backend.outputs.appServicePrincipalId
    principalType: 'ServicePrincipal'
  }
}

@description('Output the default hostname')
output endpoint string = swa.outputs.defaultHostname

@description('Output the static web app name')
output staticWebAppName string = swa.outputs.name

@description('Output the name of the resource group')
output rgName string = rg.name
