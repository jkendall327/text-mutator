import { describe, expect, it } from "vitest";
import MutationOptionsDisplay from "./MutationOptionsDisplay";
import { fireEvent, render } from "@testing-library/react";
import { MutationOptions } from "../../models";
import { defaultOptions } from "./defaultMutationOptions";

describe('ServerStatus Component', () => {
    it('should return the default options when no action is taken', () => {
        let options: MutationOptions | undefined;

        render(<MutationOptionsDisplay onOptionsChanged={p => { options = p }} />)

        expect(options === defaultOptions);
    })

    it('should update the options when a checkbox is toggled', () => {
        let options: MutationOptions | undefined;

        const cut = render(<MutationOptionsDisplay onOptionsChanged={p => { options = p }} />)

        const input = cut.container.querySelector('input[name="allowPunctuationRemoval"]');

        expect(input != null);

        fireEvent.click(input!);

        const newLocal_1 = !defaultOptions.allowPunctuationRemoval;
        // It should now be the opposite of whatever the defaults are.
        expect(options?.allowPunctuationRemoval === newLocal_1);
    })

    it('should update the mutation rate when the input is incremented', () => {
        let options: MutationOptions | undefined;

        const cut = render(<MutationOptionsDisplay onOptionsChanged={p => { options = p }} />)

        const input = cut.container.querySelector('input[name="mutationRate"]');

        expect(input != null);

        fireEvent.keyUp(input!);

        expect(options!.mutationRate === 0.05);
    })
})