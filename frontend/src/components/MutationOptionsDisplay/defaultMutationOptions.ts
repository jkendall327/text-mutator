import { MutationOptions } from "../../models";

export const defaultOptions: MutationOptions = {
    allowHomophones: true,
    allowPunctuationRemoval: true,
    allowSwaps: true,
    mutationRate: 0.05,
    seed: undefined
};