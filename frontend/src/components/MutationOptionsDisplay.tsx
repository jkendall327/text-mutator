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
        mutationRate: 0.10,
        seed: undefined
    })

    useEffect(() => {
        onOptionsChanged(options);
    });

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

        onOptionsChanged(options);
    }

    return (
        <>
            <label htmlFor="allowHomophones">
                <input name="allowHomophones" type="checkbox" checked={options.allowHomophones} onChange={e => handleChange(e)} />
            </label>
            <label htmlFor="allowSwaps">
                <input name="allowSwaps" type="checkbox" checked={options.allowSwaps} onChange={e => handleChange(e)} />
            </label>
            <label htmlFor="allowPunctuationRemoval">
                <input name="allowPunctuationRemoval" type="checkbox" checked={options.allowPunctuationRemoval} onChange={e => handleChange(e)} />
            </label>
            <label htmlFor="mutationRate">
                <input name="mutationRate" type="number" value={options.mutationRate} onChange={e => handleChange(e)} />
            </label>
            <label htmlFor="seed">
                <input name="seed" type="number" value={options.seed} onChange={e => handleChange(e)} />
            </label>
        </>)
}

export default MutationOptionsDisplay;