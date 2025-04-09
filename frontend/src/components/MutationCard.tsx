import { UseQueryResult } from '@tanstack/react-query';
import { MutationResponse } from '../models';
import './MutationCard.css'

interface MutationCardProps {
    response: UseQueryResult<MutationResponse, Error>
    found: number;
    onFound: () => void;
    onDone: () => void;
}

const MutationCard: React.FC<MutationCardProps> = ({ response, found, onFound, onDone }) => {
    const { status, data, error, isFetching } = response;

    // All mutations have been found.
    const done: boolean = status != 'success' || found === data.mutations.length;

    function isDisabled(): boolean {
        if (status != 'success' || !data) {
            return true;
        }

        return found === data.mutations.length;
    }

    return (
        <>
            <div className="mutations-display">

                <div className="mutation-buttons">
                    <button id='found-one' disabled={isDisabled()} onClick={() => onFound()}>Found one!</button>
                    <button id='done' disabled={done} onClick={() => onDone()}>Done</button>
                </div>
                <div className="text-area">
                    {isFetching && <p>Loading...</p>}
                    {error && <p>Error! {error.message}</p>}
                    {!!data && <p>{data?.mutated_text}</p>}
                </div>
            </div>
        </>
    )
}

export default MutationCard;
