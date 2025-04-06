@description('The name of the user-assigned managed identity')
param managedIdentityName string

@description('The location where resources will be deployed')
param location string = resourceGroup().location

@description('The resource group ID to assign the Contributor role to')
param targetResourceGroupId string = resourceGroup().id

// Create the user-assigned managed identity
resource managedIdentity 'Microsoft.ManagedIdentity/userAssignedIdentities@2023-01-31' = {
  name: managedIdentityName
  location: location
}

// Assign the Contributor role to the managed identity on the specified resource group
resource roleAssignment 'Microsoft.Authorization/roleAssignments@2022-04-01' = {
  name: guid(targetResourceGroupId, managedIdentity.id, 'b24988ac-6180-42a0-ab88-20f7382dd24c') // Unique name generated using guid function
  properties: {
    principalId: managedIdentity.properties.principalId
    roleDefinitionId: subscriptionResourceId(
      'Microsoft.Authorization/roleDefinitions',
      'b24988ac-6180-42a0-ab88-20f7382dd24c'
    ) // Contributor role ID
    principalType: 'ServicePrincipal'
  }
  scope: resourceGroup()
}

output managedIdentityPrincipalId string = managedIdentity.properties.principalId
output managedIdentityClientId string = managedIdentity.properties.clientId
output managedIdentityResourceId string = managedIdentity.id
