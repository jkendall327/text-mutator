// Resource group
targetScope = 'subscription'

param location string = deployment().location
param environmentName string = 'dev'
param appName string = 'text-mutator'

var rgName = 'rg-${appName}-${environmentName}'

@description('String to make resource names unique')
var resourceToken = uniqueString(subscription().subscriptionId, location)

@description('Create a resource group')
resource rg 'Microsoft.Resources/resourceGroups@2024-03-01' = {
  name: rgName
  location: location
  tags: {
    environment: environmentName
    application: appName
  }
}

output rgName string = rg.name

@description('Create a static web app')
module swa 'br/public:avm/res/web/static-site:0.3.0' = {
  name: '${deployment().name}-${appName}-webapp'
  scope: rg
  params: {
    name: 'swa-${resourceToken}'
    location: location
    sku: 'Free'
  }
}

@description('Output the default hostname')
output endpoint string = swa.outputs.defaultHostname

@description('Output the static web app name')
output staticWebAppName string = swa.outputs.name

// Web App for Containers (backend)
module backend 'modules/backend.bicep' = {
  name: 'foobar'
  scope: rg
  params: {
    appServicePlanName: ''
    location: location
  }
}

@description('Create an API management instance to link the frontend and backend.')
resource apiManagement 'Microsoft.ApiManagement/service/apis@2024-06-01-preview' = {
  parent: rg
  name: 'string'
  properties: {
    apiRevision: 'string'
    apiRevisionDescription: 'string'
    apiType: 'string'
    apiVersion: 'string'
    apiVersionDescription: 'string'
    apiVersionSet: {
      description: 'string'
      id: 'string'
      name: 'string'
      versionHeaderName: 'string'
      versioningScheme: 'string'
      versionQueryName: 'string'
    }
    apiVersionSetId: 'string'
    authenticationSettings: {
      oAuth2: {
        authorizationServerId: 'string'
        scope: 'string'
      }
      oAuth2AuthenticationSettings: [
        {
          authorizationServerId: 'string'
          scope: 'string'
        }
      ]
      openid: {
        bearerTokenSendingMethods: [
          'string'
        ]
        openidProviderId: 'string'
      }
      openidAuthenticationSettings: [
        {
          bearerTokenSendingMethods: [
            'string'
          ]
          openidProviderId: 'string'
        }
      ]
    }
    contact: {
      email: 'string'
      name: 'string'
      url: 'string'
    }
    description: 'string'
    displayName: 'string'
    format: 'string'
    isCurrent: true
    license: {
      name: 'string'
      url: 'string'
    }
    path: 'string'
    protocols: [
      'string'
    ]
    serviceUrl: 'string'
    sourceApiId: 'string'
    subscriptionKeyParameterNames: {
      header: 'string'
      query: 'string'
    }
    subscriptionRequired: true
    termsOfServiceUrl: 'string'
    translateRequiredQueryParameters: 'string'
    type: 'string'
    value: 'string'
    wsdlSelector: {
      wsdlEndpointName: 'string'
      wsdlServiceName: 'string'
    }
  }
}
