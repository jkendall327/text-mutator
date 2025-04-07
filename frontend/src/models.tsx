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

interface Mutation {
    text: string,
    options: MutationOptions
}

interface MutationOptions {
    allowHomophones: boolean
}

export type { MutationRequest, MutationOptionsDto, Mutation, MutationOptions };