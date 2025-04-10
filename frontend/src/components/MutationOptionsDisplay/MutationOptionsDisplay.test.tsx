import { describe, expect, it } from "vitest";
import MutationOptionsDisplay from "./MutationOptionsDisplay";
import { render } from "@testing-library/react";
import { MutationOptions } from "../../models";
import { defaultOptions } from "./defaultMutationOptions";

describe('ServerStatus Component', () => {
    it('should return the default options when no action is taken', () => {
        let options: MutationOptions | undefined;

        render(<MutationOptionsDisplay onOptionsChanged={p => { options = p }} />)

        expect(options === defaultOptions);
    })
})