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
        const { name, type, value, checked } = e.target;

        let newValue: string | number | boolean | undefined;

        if (type === 'checkbox') {
            newValue = checked;
        } else if (type === 'number') {
            if (value === '') {
                // Handle empty number input: undefined for seed, maybe 0 for rate?
                newValue = name === 'seed' ? undefined : 0;
            } else {
                // Parse number inputs
                const num = name === 'mutationRate' ? parseFloat(value) : parseInt(value, 10);

                if (!isNaN(num)) {
                    newValue = num;

                    if (name === 'mutationRate') {
                        // Clamp between 0 and 1.
                        newValue = Math.max(0, Math.min(1, newValue));
                    }
                } else {
                    console.warn(`Invalid number input for ${name}: ${value}`);
                    return;
                }
            }
        } else {
            // Handle other types like 'text' if needed
            newValue = value;
        }

        setOptions(prev => {
            const key = name as keyof MutationOptions

            return {
                ...prev,
                [key]: newValue
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
                    <input name="allowSwaps" type="checkbox" checked={options.allowSwaps} onChange={e => handleChange(e)} />
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