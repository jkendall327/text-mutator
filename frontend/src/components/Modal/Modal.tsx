// https://blog.logrocket.com/creating-reusable-pop-up-modal-react/
import { useEffect, useRef } from "react";

interface ModalProps {
    isOpen: boolean;
    hasCloseBtn?: boolean;
    onClose?: () => void;
    children: React.ReactNode;
};

const Modal = ({ isOpen, hasCloseBtn, onClose, children }: ModalProps) => {
    const modalRef = useRef<HTMLDialogElement>(null);

    useEffect(() => {
        // Grabbing a reference to the modal in question
        const modalElement = modalRef.current;
        if (!modalElement) return;

        // Open modal when `isOpen` changes to true
        if (isOpen) {
            modalElement.showModal();
        } else {
            modalElement.close();
        }
    }, [isOpen]);

    const handleCloseModal = () => {
        if (onClose) {
            onClose();
        }
    };

    const handleKeyDown = (event: React.KeyboardEvent<HTMLDialogElement>) => {
        if (event.key === "Escape") {
            handleCloseModal();
        }
    };

    return (
        <dialog ref={modalRef} onKeyDown={handleKeyDown}>
            {hasCloseBtn && (
                <button className="modal-close-btn" onClick={handleCloseModal}>
                    Close
                </button>
            )}
            {children}
        </dialog>
    );
}

export default Modal;