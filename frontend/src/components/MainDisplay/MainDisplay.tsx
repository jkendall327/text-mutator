import './MainDisplay.css'
import MutationCard from '../MutationCard/MutationCard.tsx'
import { useState } from 'react';
import { MutationOptions, MutationRequest } from '../../models.tsx';
import MutationOptionsDisplay from '../MutationOptionsDisplay/MutationOptionsDisplay.tsx';
import ServerStatus from '../ServerStatus/ServerStatus.tsx';
import Modal from '../Modal/Modal.tsx';
import useMutation from '../../hooks/useMutation.tsx';

export default function MainDisplay() {
    const [modalOpen, setModalOpen] = useState<boolean>(false);
    const [found, setFound] = useState<number>(0);
    const [text, setText] = useState<string>("");

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

    const [options, setOptions] = useState<MutationOptions>({
        allowHomophones: true,
        allowPunctuationRemoval: true,
        allowSwaps: true,
        mutationRate: 1.0,
        seed: undefined
    });

    const response = useMutation(req);

    function getNewMutation(): void {
        setFound(0);

        setReq({
            text: text,
            config: options
        })
    }

    const handleOptionsChanged = (options: MutationOptions) => {
        setOptions(options);
    };

    function handleDone(): void {
        if (response?.data == null) return;

        setFound(response.data.mutations.length);
    }

    function getStatusMessage(): string {
        if (response?.data == null) {
            return "Enter some text on the left and click 'mutate' to get started!";
        }

        if (found == response.data.mutations.length) {
            return "You've found all the mutations!";
        }

        return `You've found ${found}/${response.data.mutations.length} mutations.`
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
                            <button id='mutate' disabled={text === ""} onClick={() => getNewMutation()}>
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

                <span id='mutation-count'>{getStatusMessage()}</span>
            </div>

            <div className='server-status'>
                <ServerStatus />
            </div>
        </>
    )
}