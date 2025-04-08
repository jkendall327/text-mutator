import { ChangeEvent, FC, useState } from "react"
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
            <label htmlFor="homophones">
                <input name="homophones" type="checkbox" checked={options.allowHomophones} onChange={e => handleChange(e)} />
            </label>
            <label htmlFor="swaps">
                <input name="swaps" type="checkbox" checked={options.allowSwaps} onChange={e => handleChange(e)} />
            </label>
            <label htmlFor="punctuation">
                <input name="punctuation" type="checkbox" checked={options.allowPunctuationRemoval} onChange={e => handleChange(e)} />
            </label>
            <label htmlFor="rate">
                <input name="rate" type="number" value={options.mutationRate} onChange={e => handleChange(e)} />
            </label>
            <label htmlFor="seed">
                <input name="seed" type="number" value={options.seed} onChange={e => handleChange(e)} />
            </label>
        </>)
}

export default MutationOptionsDisplay;