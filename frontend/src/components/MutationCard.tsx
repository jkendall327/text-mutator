import { useQuery } from "@tanstack/react-query";
import axios from "axios";
import { MutationRequest, MutationResponse } from "../models";
import { useState } from "react";
import './MutationCard.css'

interface MutationCardProps {
    req: MutationRequest;
}

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

const MutationCard: React.FC<MutationCardProps> = ({ req }) => {
    const { status, data, error, isFetching } = useMutation(req)

    const [found, setFound] = useState<number>(0);

    // All mutations have been found.
    const done: boolean = status === 'success' && !!data && found === data.mutations.length;

    function isDisabled(): boolean {
        if (status != 'success' || !data) {
            return true;
        }

        return found === data.mutations.length;
    }

    function handleFound(): void {
        setFound(found + 1)
    }

    function handleDone(): void {
        if (status != 'success') {
            throw new Error("Tried to complete text when the HTTP status was not yet successful.");
        }

        setFound(data.mutations.length);
    }

    const content =
        <>
            <div className="mutations-display">

                {/* {data?.mutations.length != undefined ? <p>You have found {found}/{data?.mutations.length} mutations</p> : <></>} */}

                <div className="mutation-buttons">
                    <button id='found-one' disabled={isDisabled()} onClick={() => handleFound()}>Found one!</button>
                    <button id='done' disabled={done} onClick={() => handleDone()}>Done</button>
                </div>
                <div className="text-area">
                    <p>{data?.mutated_text}</p>
                </div>
            </div>
        </>

    return (
        isFetching ?
            <p>Loading...</p> :
            error ? <p>Error! {error.message}</p> : content
    )
}

export default MutationCard;
