import { useQuery } from "@tanstack/react-query";
import { MutationRequest, MutationResponse } from "./models";
import axios from "axios";

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

    const { data, error, isFetching } = useMutation(req)

    return (
        <>
            {isFetching ? <p>Loading...</p> :
                error ? <p>Error! {error.message}</p>
                    : data?.mutated_text}
        </>
    )
}

export default MutationCard;
