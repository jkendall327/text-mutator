import { useState } from 'react'
import { apiService as api } from './api.tsx'
import { MutationRequest, MutationResponse } from './models.tsx'
import './Mutator.css'
import ServerStatus from './ServerStatus.tsx'

export default function Mutator() {

    const [mutation, setMutation] = useState<MutationResponse | undefined>(undefined)

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
            <ServerStatus />
        </>
    )
}