import './ServerStatus.css'
import useHealthcheck from "../../hooks/useHealthcheck";

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

            {isFetching && <p>Loading...</p>}
            {error && <span>Error: {error.message}</span>}
            {!!data && content}
        </div>
    )
}