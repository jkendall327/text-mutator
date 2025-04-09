import './ServerStatus.css'
import useHealthcheck from "../../hooks/useHealthcheck";

export default function ServerStatus() {
    const { data, error, isFetching } = useHealthcheck();

    return (
        <div className="server-status">
            <div className={error ? 'indicator-dead' : 'indicator-alive'} />

            <p className="server-status-description">
                Backend: {isFetching ? "Checking..." : error ? `Error: ${error.message}` : data}
            </p>
        </div>
    )
}