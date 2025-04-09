import './MainDisplay.css'
import MutationCard from './MutationCard.tsx'
import { useState } from 'react';
import { MutationOptions, MutationRequest } from '../models.tsx';
import MutationOptionsDisplay from './MutationOptionsDisplay.tsx';
import ServerStatus from './ServerStatus.tsx';
import Modal from './Modal/Modal.tsx';

export default function Mutator() {
    const [modalOpen, setModalOpen] = useState<boolean>(false);

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
                        req={req}
                    />
                </div>

                <span id='mutation-count'>You've found X/Y mutations.</span>
            </div>

            <div className='server-status'>
                <ServerStatus />
            </div>
        </>
    )
}