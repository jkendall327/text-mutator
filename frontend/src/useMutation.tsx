import { useQuery } from "@tanstack/react-query";
import axios from "axios";
import { MutationRequest, MutationResponse } from "./models";

const apiClient = axios.create({
    baseURL: '/api/v1',
    headers: {
        'Content-Type': 'application/json',
        'Accept': 'application/json',
    }
});

function performCall(req: MutationRequest) {
    return async (): Promise<MutationResponse> => {

        console.log("Fetching mutation for: ", req);

        const response = await apiClient.post<MutationResponse>("/mutate", req);

        console.log("Retrieved: ", response.data);

        return response.data;
    };
}

function useMutation(req: MutationRequest) {
    const isEnabled = !!req.text;

    return useQuery({
        queryKey: ['mutation', req], queryFn: performCall(req), enabled: isEnabled, retry: false
    });
}

export default useMutation;