// https://learn.microsoft.com/en-us/azure/static-web-apps/publish-bicep
targetScope = 'resourceGroup'

param location string = resourceGroup().location
param staticWebAppName string
param backendAppResourceId string

@description('Get reference to the static web app')
resource staticWebApp 'Microsoft.Web/staticSites@2023-12-01' existing = {
  name: staticWebAppName
}

@description('Link backend to static web app')
resource linkedStaticWebAppBackend 'Microsoft.Web/staticSites/linkedBackends@2023-12-01' = {
  parent: staticWebApp
  name: 'linkedBackend'
  properties: {
    backendResourceId: backendAppResourceId
    region: location
  }
}
