import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import Mutator from './components/mutator/Mutator.tsx'
import './App.css'

const queryClient = new QueryClient()

function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <Mutator />
    </QueryClientProvider>)
}

export default App
