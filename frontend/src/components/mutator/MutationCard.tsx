import { useQuery } from "@tanstack/react-query";
import { MutationRequest, MutationResponse } from "./models";

interface MutationCardProps {
    req: MutationRequest;
}

function performCall(req: MutationRequest) {
    return async (): Promise<MutationResponse> => {
        console.log("Fetching mutation for:", req);

        const response = await fetch(`http://0.0.0.0:8080/api/v1/mutate`, {
            method: "post",
            headers: {
                'Accept': 'application/json',
                'Content-Type': 'application/json'
            },

            body: JSON.stringify(req)
        });

        return await response.json();
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
