// https://learn.microsoft.com/en-us/azure/container-registry/container-registry-get-started-bicep?tabs=CLI

@minLength(5)
@maxLength(50)
@description('Provide a globally unique name of your Azure Container Registry')
param acrName string = 'acr${uniqueString(resourceGroup().id)}'

@description('Provide a location for the registry.')
param location string = resourceGroup().location

@description('Provide a tier of your Azure Container Registry.')
param acrSku string = 'Basic'

resource acrResource 'Microsoft.ContainerRegistry/registries@2023-01-01-preview' = {
  name: acrName
  location: location
  identity: {
    type: 'SystemAssigned'
  }
  sku: {
    name: acrSku
  }
  properties: {
    adminUserEnabled: false
  }
}

@description('Output the login server property for later use')
output loginServer string = acrResource.properties.loginServer

@description('The ID of the system-assigned managed identity for the registry')
output registryIdentityId string = acrResource.identity.principalId

@description('The name of the registry')
output registryName string = 'foo'
