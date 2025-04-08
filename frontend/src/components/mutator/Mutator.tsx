import { MutationRequest } from './models.tsx'
import './Mutator.css'
import ServerStatus from '../ServerStatus.tsx'
import MutationCard from './MutationCard.tsx'

export default function Mutator() {
    const foo: MutationRequest = {
        text: "Eius saepe enim magnam. Placeat exercitationem quae et omnis sunt dolorum. Molestias cum aut quia consequatur. Omnis explicabo qui est eveniet ipsam ad. Est ut officiis quisquam laudantium dicta. Temporibus autem totam eum ea autem ipsam.",
        config: {
            allowHomophones: true,
            allowPunctuationRemoval: true,
            allowSwaps: true,
            mutationRate: 1.0,
            seed: undefined
        }
    };

    return (
        <>
            <div>
                <button>
                    Mutate!
                </button>
            </div>
            <MutationCard
                req={foo}
            />
            <ServerStatus />
        </>
    )
}