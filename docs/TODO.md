- remember to use relative urls for frontend-to-backend calls.
- place the managed identity used to execute the bicep pipeline in its own resource group to avoid lifetime issues with it being recreated/deleted.
- setup a git hook to prevent pushes if tests/builds fail locally?

https://vite.dev/guide/