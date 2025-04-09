import { useQuery } from "@tanstack/react-query";
import './ServerStatus.css'

function useHealthcheck() {
    return useQuery({
        queryKey: ['healthcheck'], queryFn: async (): Promise<string> => {
            const response = await fetch('/api/v1/health')
            return await response.text();
        }
    });
}

export default function ServerStatus() {
    const { status, data, error, isFetching } = useHealthcheck()

    const content =
        <>
            <p className="server-status-description">
                Backend: {data}
            </p>
        </>

    return (
        <div className="server-status">
            <div className={status === 'success' ? 'indicator-alive' : 'indicator-dead'} />

            {isFetching ? (
                'Loading...'
            ) : status === 'error' ? (
                <span>Error: {error.message}</span>
            ) : (
                <>
                    {content}
                </>
            )
            }
        </div>
    )
}