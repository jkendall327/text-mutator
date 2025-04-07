import { Mutation, MutationRequest } from './models.tsx'

export const apiService = {

    async healthcheck(): Promise<boolean> {
        //const apiUrl = import.meta.env.VITE_API_BASE_URL || 'http://localhost:8080';
        //const response = await fetch(`${apiUrl}/api/v1/health/`);

        const response = await fetch(`http://localhost:8080/api/v1/health`);

        return response.ok ? true : false;
    },

    async mutate(req: MutationRequest): Promise<Mutation> {
        //const apiUrl = import.meta.env.VITE_API_BASE_URL || 'http://localhost:8080';
        //const response = await fetch(`${apiUrl}/api/v1/health/`);

        const response = await fetch("http://localhost:8080/api/v1/mutate", {
            method: "post",
            headers: {
                'Accept': 'application/json',
                'Content-Type': 'application/json'
            },

            body: JSON.stringify(req)
        })

        const json = await response.json();

        return json
    }
}