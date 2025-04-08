import { MutationRequest } from './models.tsx'
import './Mutator.css'
import ServerStatus from '../ServerStatus.tsx'
import MutationCard from './MutationCard.tsx'
import { useState } from 'react';

export default function Mutator() {
    const [req, setReq] = useState<MutationRequest>({
        text: "",
        config: {
            allowHomophones: true,
            allowPunctuationRemoval: true,
            allowSwaps: true,
            mutationRate: 1.0,
            seed: undefined
        }
    });

    const [text, setText] = useState<string>("");

    function handleClick(): void {
        setReq({
            text: text,
            config: {
                allowHomophones: true,
                allowPunctuationRemoval: true,
                allowSwaps: true,
                mutationRate: 1.0,
                seed: undefined
            }
        })
    }

    return (
        <>
            <div>
                <label>
                    Text input: <input name="myInput" onChange={e => setText(e.target.value)} />
                </label>

                <button onClick={() => handleClick()}>
                    Mutate!
                </button>
            </div>
            <MutationCard
                req={req}
            />
            <ServerStatus />
        </>
    )
}