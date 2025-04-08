import { ChangeEvent, FC, useEffect, useState } from "react"
import { MutationOptions } from "./mutator/models";

interface MutationOptionsDisplayProps {
    onOptionsChanged: (options: MutationOptions) => void;
}

const MutationOptionsDisplay: FC<MutationOptionsDisplayProps> = ({ onOptionsChanged }) => {
    const [options, setOptions] = useState<MutationOptions>({
        allowHomophones: true,
        allowPunctuationRemoval: true,
        allowSwaps: true,
        mutationRate: 0.05,
        seed: undefined
    })

    useEffect(() => {
        onOptionsChanged(options);
    }, [options, onOptionsChanged]);

    function handleChange(e: ChangeEvent<HTMLInputElement>): void {
        const name = e.target.name;
        const value = e.target.value;

        setOptions(prev => {
            const key = name as keyof MutationOptions

            return {
                ...prev,
                [key]: value
            }
        })
    }

    return (
        <>
            <div className="mutation-options">
                <label htmlFor="allowHomophones">
                    Allow homophones
                    <input name="allowHomophones" type="checkbox" checked={options.allowHomophones} onChange={e => handleChange(e)} />
                </label>
                <label htmlFor="allowSwaps">
                    Allow swaps
                    <input name="" type="checkbox" checked={options.allowSwaps} onChange={e => handleChange(e)} />
                </label>
                <label htmlFor="allowPunctuationRemoval">
                    Allow punctuation to be removed
                    <input name="allowPunctuationRemoval" type="checkbox" checked={options.allowPunctuationRemoval} onChange={e => handleChange(e)} />
                </label>
                <label htmlFor="mutationRate">
                    Mutation rate (0 - 1.00)
                    <input
                        name="mutationRate"
                        step="0.01"
                        min="0"
                        max="1"
                        type="number"
                        value={options.mutationRate}
                        onChange={e => handleChange(e)} />
                </label>
                <label htmlFor="seed">
                    Seed (optional)
                    <input name="seed" type="number" value={options.seed ?? ''} onChange={e => handleChange(e)} />
                </label>
            </div>
        </>)
}

export default MutationOptionsDisplay;