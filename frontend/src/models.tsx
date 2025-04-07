interface MutationRequest {
    text: string,
    options: MutationOptionsDto
}

interface MutationOptionsDto {
    allowHomophones: boolean
}

interface Mutation {
    text: string,
    options: MutationOptions
}

interface MutationOptions {
    allowHomophones: boolean
}

export type { MutationRequest, MutationOptionsDto, Mutation, MutationOptions };