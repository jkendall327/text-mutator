import { useQuery } from "@tanstack/react-query";

function useHealthcheck() {
    return useQuery({
        queryKey: ['healthcheck'], queryFn: async (): Promise<string> => {
            const response = await fetch('http://0.0.0.0:8080/api/v1/health')
            return await response.text();
        }
    });
}

export default function ServerStatus() {
    const { status, data, error, isFetching } = useHealthcheck()

    return (
        <div>
            {status === 'pending' ? (
                'Loading...'
            ) : status === 'error' ? (
                <span>Error: {error.message}</span>
            ) : (
                <>
                    <p>
                        Backend: {isFetching ? "Checking... " : data}
                    </p>
                </>
            )
            }
        </div>
    )
}