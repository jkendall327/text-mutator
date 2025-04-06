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
