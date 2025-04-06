param location string
param appServicePlanName string

resource appServicePlan 'Microsoft.Web/serverfarms@2020-12-01' = {
  name: appServicePlanName
  location: location
  kind: 'linux'
  properties: {
    reserved: true
  }
  sku:  {
      name: 'B1'
    tier: 'Basic'
  }
}
