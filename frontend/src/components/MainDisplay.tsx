import './MainDisplay.css'
import MutationCard from './MutationCard.tsx'
import { useState } from 'react';
import { MutationOptions, MutationRequest } from '../models.tsx';
import MutationOptionsDisplay from './MutationOptionsDisplay.tsx';
import ServerStatus from './ServerStatus.tsx';

export default function Mutator() {
    const [req, setReq] = useState<MutationRequest>({
        text: "",
        config: {
            allowHomophones: true,
            allowPunctuationRemoval: true,
            allowSwaps: true,
            mutationRate: 0.1,
            seed: undefined
        }
    });

    const [text, setText] = useState<string>("");

    const [options, setOptions] = useState<MutationOptions>({
        allowHomophones: true,
        allowPunctuationRemoval: true,
        allowSwaps: true,
        mutationRate: 1.0,
        seed: undefined
    });

    function handleClick(): void {
        setReq({
            text: text,
            config: options
        })
    }

    const handleOptionsChanged = (options: MutationOptions) => {
        setOptions(options);
    };

    return (
        <>
            <MutationOptionsDisplay
                onOptionsChanged={handleOptionsChanged}
            />

            <label>
                Text input: <input name="myInput" onChange={e => setText(e.target.value)} />
            </label>

            <button onClick={() => handleClick()}>
                Mutate!
            </button>

            <MutationCard
                req={req}
            />

            <ServerStatus />
        </>
    )
}