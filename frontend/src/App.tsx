import { useEffect, useState } from 'react'
import { apiService as api } from './api.tsx'
import './App.css'
import { Mutation, MutationRequest } from './models.tsx'

function App() {
  const [mutation, setMutation] = useState<Mutation | undefined>(undefined)
  const [backendOk, setBackendOk] = useState<boolean>(false)

  const performHealthcheck = async () => {
    const ok = await api.healthcheck();
    setBackendOk(ok);
  };

  const performMutation = async () => {
    const req: MutationRequest = {
      text: "Eius saepe enim magnam. Placeat exercitationem quae et omnis sunt dolorum. Molestias cum aut quia consequatur. Omnis explicabo qui est eveniet ipsam ad. Est ut officiis quisquam laudantium dicta. Temporibus autem totam eum ea autem ipsam.",
      config: {
        allowHomophones: true,
        allowPunctuationRemoval: true,
        allowSwaps: true,
        mutationRate: 1.0,
        seed: undefined
      }
    };

    const response = await api.mutate(req);

    setMutation(response);
  };

  useEffect(() => {
    performHealthcheck();
  }, []);

  return (
    <>
      <div>
        <button onClick={() => performMutation()}>
          Mutate!
        </button>
      </div>
      <p>
        Response body: {JSON.stringify(mutation)}
      </p>
      <p>
        Backend: {backendOk.toString()}
      </p>
    </>
  )
}

export default App
