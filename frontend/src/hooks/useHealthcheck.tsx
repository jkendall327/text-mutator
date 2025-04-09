import { useQuery } from "@tanstack/react-query";

function useHealthcheck() {
    return useQuery({
        queryKey: ['healthcheck'], queryFn: async (): Promise<string> => {
            const response = await fetch('/api/v1/health')
            return await response.text();
        }, refetchInterval: 5000
    });
}

export default useHealthcheck;