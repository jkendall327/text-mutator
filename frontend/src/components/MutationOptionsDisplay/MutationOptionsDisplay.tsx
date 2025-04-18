import { ChangeEvent, useEffect, useState } from "react"
import { MutationOptions } from "../../models";
import './MutationOptionsDisplay.css'
import { defaultOptions } from "./defaultMutationOptions";

interface MutationOptionsDisplayProps {
    onOptionsChanged: (options: MutationOptions) => void;
}

const optionsKeys: { [K in keyof MutationOptions]: K } = {
    allowHomophones: "allowHomophones",
    allowPunctuationRemoval: "allowPunctuationRemoval",
    allowSwaps: "allowSwaps",
    mutationRate: "mutationRate",
    seed: "seed"
};

const MutationOptionsDisplay = ({ onOptionsChanged }: MutationOptionsDisplayProps) => {
    const [options, setOptions] = useState<MutationOptions>(defaultOptions);

    useEffect(() => {
        onOptionsChanged(options);
    }, [options, onOptionsChanged]);

    function handleChange(e: ChangeEvent<HTMLInputElement>): void {
        const { name, type, value, checked } = e.target;

        let newValue: string | number | boolean | undefined;

        const key = name as keyof MutationOptions;

        if (type === 'checkbox') {
            newValue = checked;
        } else if (type === 'number') {
            if (value === '') {
                // Handle empty number input: undefined for seed, maybe 0 for rate?
                newValue = key === 'seed' ? undefined : 0;
            } else {
                // Parse number inputs
                const num = key === 'mutationRate' ? parseFloat(value) : parseInt(value, 10);

                if (!isNaN(num)) {
                    newValue = num;

                    if (key === 'mutationRate') {
                        // Clamp between 0 and 1.
                        newValue = Math.max(0, Math.min(1, newValue));
                    }
                } else {
                    console.warn(`Invalid number input for ${key}: ${value}`);
                    return;
                }
            }
        } else {
            // Handle other types like 'text' if needed
            newValue = value;
        }

        setOptions(prev => {
            return {
                ...prev,
                [key]: newValue
            }
        })
    }

    return (
        <>
            <div className="mutation-options">
                <label htmlFor={optionsKeys.allowHomophones} data-text="Swaps common homophones (e.g. 'your' for 'you're')" className="tooltip">
                    Allow homophones
                    <input name={optionsKeys.allowHomophones} type="checkbox" checked={options.allowHomophones} onChange={e => handleChange(e)} />
                </label>
                <label htmlFor={optionsKeys.allowSwaps} data-text="Swaps two consecutive characters (e.g. 'hello' becomes 'hlelo')" className="tooltip">
                    Allow swaps
                    <input name={optionsKeys.allowSwaps} type="checkbox" checked={options.allowSwaps} onChange={e => handleChange(e)} />
                </label>
                <label htmlFor={optionsKeys.allowPunctuationRemoval} data-text="Deletes individual punctuation marks, such as commas" className="tooltip">
                    Allow punctuation to be removed
                    <input name={optionsKeys.allowPunctuationRemoval} type="checkbox" checked={options.allowPunctuationRemoval} onChange={e => handleChange(e)} />
                </label>
                <label htmlFor={optionsKeys.mutationRate} data-text="Sets the overall amount of mutations added" className="tooltip">
                    Mutation rate (0 - 1.00)
                    <input
                        name={optionsKeys.mutationRate}
                        step="0.01"
                        min="0"
                        max="1"
                        type="number"
                        value={options.mutationRate}
                        onChange={e => handleChange(e)} />
                </label>
                <label htmlFor={optionsKeys.seed} data-text="Replaces randomness with a deterministic outcome" className="tooltip">
                    Seed (optional)
                    <input name={optionsKeys.seed} type="number" value={options.seed ?? ''} onChange={e => handleChange(e)} />
                </label>
            </div>
        </>)
}

export default MutationOptionsDisplay;