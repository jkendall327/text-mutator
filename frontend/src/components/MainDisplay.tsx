import './MainDisplay.css'
import MutationCard from './MutationCard.tsx'
import { useState } from 'react';
import { MutationOptions, MutationRequest, MutationResponse } from '../models.tsx';
import MutationOptionsDisplay from './MutationOptionsDisplay.tsx';
import ServerStatus from './ServerStatus.tsx';
import Modal from './Modal/Modal.tsx';
import axios from 'axios';
import { useQuery } from '@tanstack/react-query';

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

export default function Mutator() {
    const [modalOpen, setModalOpen] = useState<boolean>(false);
    const [found, setFound] = useState<number>(0);

    const [req, setReq] = useState<MutationRequest>({
        text: "",
        config: {
            allowHomophones: true,
            allowPunctuationRemoval: true,
            allowSwaps: true,
            mutationRate: 0.1,
            seed: undefined
        }
    });

    const response = useMutation(req)

    const [text, setText] = useState<string>("");

    const [options, setOptions] = useState<MutationOptions>({
        allowHomophones: true,
        allowPunctuationRemoval: true,
        allowSwaps: true,
        mutationRate: 1.0,
        seed: undefined
    });

    function handleClick(): void {
        setReq({
            text: text,
            config: options
        })
    }

    const handleOptionsChanged = (options: MutationOptions) => {
        setOptions(options);
    };

    function handleDone(): void {
        throw new Error('Function not implemented.');
    }

    return (
        <>
            <div className='main-display'>

                <Modal
                    isOpen={modalOpen}
                    onClose={() => setModalOpen(false)}
                    hasCloseBtn={true}>
                    <MutationOptionsDisplay
                        onOptionsChanged={handleOptionsChanged}
                    />
                </Modal>

                <div className='main-columns'>
                    <div className='input-text-section'>
                        <div className='input-text-buttons'>
                            <button id='settings' disabled={modalOpen} onClick={() => setModalOpen(true)}>Settings</button>
                            <button id='mutate' disabled={text === ""} onClick={() => handleClick()}>
                                Mutate!
                            </button>
                        </div>

                        <div className='text-area'>
                            <textarea name="myInput" placeholder="Place your plain text here..." onChange={e => setText(e.target.value)} />
                        </div>
                    </div>

                    <MutationCard
                        response={response}
                        found={found}
                        onFound={() => setFound(found + 1)}
                        onDone={() => handleDone()}
                    />
                </div>

                {!!response.data && <span id='mutation-count'>You've found {found}/{response.data.mutations.length} mutations.</span>}
            </div>

            <div className='server-status'>
                <ServerStatus />
            </div>
        </>
    )
}