import { useEffect, useState } from 'react'
import { apiService as api } from './api.tsx'
import { MutationRequest, MutationResponse } from './models.tsx'
import { useQuery } from '@tanstack/react-query'
import './Mutator.css'

function useHealthcheck() {
    return useQuery({
        queryKey: ['healthcheck'], queryFn: async (): Promise<string> => {
            const response = await fetch('api/v1/health')
            return await response.text();
        }
    });
}

export default function Mutator() {
    const { status, data, error, isFetching } = useHealthcheck()

    const [mutation, setMutation] = useState<MutationResponse | undefined>(undefined)
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
                Response body: {mutation === undefined ? "(not found)" : JSON.stringify(mutation)}
            </p>
            <p>
                Backend: {backendOk.toString()}
            </p>
        </>
    )
}