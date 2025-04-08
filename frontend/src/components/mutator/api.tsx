import { MutationRequest, MutationResponse } from './models.tsx'

export const apiService = {

    async healthcheck(): Promise<boolean> {
        const url = getBaseUrl();

        const response = await fetch(`${url}/health`);

        return response.ok ? true : false;
    },

    async mutate(req: MutationRequest): Promise<MutationResponse> {
        const url = getBaseUrl();

        const response = await fetch(`${url}/mutate`, {
            method: "post",
            headers: {
                'Accept': 'application/json',
                'Content-Type': 'application/json'
            },

            body: JSON.stringify(req)
        })

        const json = await response.json();
        const result = json as MutationResponse
        return result;
        // const parsed = parseJSON<MutationResponse>(result);

        //return parsed
    }
}

function parseJSON<T>(json: string): T {
    return JSON.parse(json) as T;
}

const CURRENT_VERSION: number = 1;

function getBaseUrl(): string {
    return `http://localhost:8080/api/v${CURRENT_VERSION}`;
}