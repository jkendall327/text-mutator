interface MutationRequest {
    text: string,
    config: MutationOptionsDto
}

interface MutationOptionsDto {
    allowSwaps: boolean
    allowPunctuationRemoval: boolean
    allowHomophones: boolean
    seed: number | undefined
    mutationRate: number
}

interface MutationResponse {
    mutated_text: string,
    mutations: MutationItem[]
}

interface MutationItem {
    start: number,
    end: number,
    type: "SwapLetters" | "RemovePunctuation" | "ReplaceHomophone"
}

interface Mutation {
    text: string,
    options: MutationOptions
}

interface MutationOptions {
    allowHomophones: boolean
}

export type { MutationRequest, MutationOptionsDto, Mutation, MutationOptions, MutationResponse };