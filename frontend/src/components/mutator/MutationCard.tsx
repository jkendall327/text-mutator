import { useQuery } from "@tanstack/react-query";
import { MutationRequest, MutationResponse } from "./models";

interface MutationCardProps {
    req: MutationRequest;
}

function useMutation(req: MutationRequest) {
    return useQuery({
        queryKey: ['mutation'], queryFn: async (): Promise<MutationResponse> => {
            const response = await fetch(`http://0.0.0.0:8080/api/v1/mutate`, {
                method: "post",
                headers: {
                    'Accept': 'application/json',
                    'Content-Type': 'application/json'
                },

                body: JSON.stringify(req)
            })

            return await response.json();
        }
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