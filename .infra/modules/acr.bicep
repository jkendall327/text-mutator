// https://learn.microsoft.com/en-us/azure/container-registry/container-registry-get-started-bicep?tabs=CLI

@description('Provide a location for the registry.')
param location string = resourceGroup().location

@minLength(5)
@maxLength(50)
@description('Provide a globally unique name of your Azure Container Registry')
param acrName string = 'acr${uniqueString(resourceGroup().id)}'

@description('Provide a tier of your Azure Container Registry.')
param acrSku string = 'Basic'

@description('The deployment environment')
param environment string = 'dev'

@description('The semantic app name')
param appName string

resource acr 'Microsoft.ContainerRegistry/registries@2023-01-01-preview' = {
  name: acrName
  location: location
  sku: {
    name: acrSku
  }
  properties: {
    adminUserEnabled: true
  }
  tags: {
    environment: environment
    appName: appName
  }
}

@description('Output the login server endpoint of the registry.')
output loginServer string = acr.properties.loginServer

@description('The name of the registry')
output registryName string = acr.name

@description('The resource ID of the registry')
output registryResourceId string = acr.id
