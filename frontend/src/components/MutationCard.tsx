import { useQuery } from "@tanstack/react-query";
import axios from "axios";
import { MutationRequest, MutationResponse } from "../models";
import { useState } from "react";

interface MutationCardProps {
    req: MutationRequest;
}

const apiClient = axios.create({
    baseURL: 'http://0.0.0.0:8080/api/v1',
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

const MutationCard: React.FC<MutationCardProps> = ({ req }) => {
    const { status, data, error, isFetching } = useMutation(req)

    const [found, setFound] = useState<number>(0);
    const [done, setDone] = useState<boolean>(false);

    function isDisabled(): boolean {
        if (status === 'pending' || status === 'error') {
            return true;
        }

        return found === data.mutations.length;
    }

    const content =
        <>
            <p>Mutations:</p> <span>3/{data?.mutations.length}</span>
            <button disabled={isDisabled()} onClick={() => setFound(found + 1)}>Found one!</button>
            <button disabled={done} onClick={() => setDone(true)}>Done</button>
            <p>{data?.mutated_text}</p>
        </>

    return (
        <>
            {
                isFetching ?
                    <p>Loading...</p> :
                    error ? <p>Error! {error.message}</p> : content
            }
        </>
    )
}

export default MutationCard;
