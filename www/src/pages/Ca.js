import { CaControl } from '../components/CaControl';
import { useHistory } from "react-router-dom";

export function Ca({ seed }) {
    const history = useHistory();

    if (!seed) {
        console.info("No seed. Redirecting to main page");
        history.push("/");
    }

    return (seed ? <CaControl seed={seed} /> : null);
}