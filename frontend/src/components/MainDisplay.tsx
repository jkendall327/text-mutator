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
            <div className='main-display'>
                {/* <div className='mutation-options'>
                    <MutationOptionsDisplay
                        onOptionsChanged={handleOptionsChanged}
                    />
                </div> */}

                <div className='input-text-section'>
                    <button onClick={() => handleClick()}>
                        Mutate!
                    </button>

                    <div className='text-area'>
                        <input name="myInput" onChange={e => setText(e.target.value)} />
                    </div>
                </div>

                <MutationCard
                    req={req}
                />

            </div>

            <div className='server-status'>
                <ServerStatus />
            </div>
        </>
    )
}